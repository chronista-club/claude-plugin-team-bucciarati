use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;

use chrono::Utc;
use regex::Regex;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::ServerCapabilities;
use rmcp::{ServerHandler, ServiceExt, tool, tool_handler, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

// --- Server ---

#[derive(Debug, Clone)]
struct TeambMetrics {
    data_dir: PathBuf,
    tool_router: ToolRouter<Self>,
}

impl TeambMetrics {
    fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            tool_router: Self::tool_router(),
        }
    }
}

// --- Tool input/output types ---

#[derive(Debug, Deserialize, JsonSchema)]
struct MeasureParams {
    /// The shell command to execute (e.g. "cargo test", "bun test")
    command: String,
    /// Metric extraction patterns: key -> regex with a capture group for the value.
    /// Example: { "pass": "(\\d+) passed", "fail": "(\\d+) failed" }
    patterns: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct MeasureOutput {
    command: String,
    exit_code: i32,
    metrics: HashMap<String, f64>,
    raw_output: String,
    timestamp: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct DecideParams {
    /// Metrics from the baseline measurement
    before: HashMap<String, f64>,
    /// Metrics from the post-change measurement
    after: HashMap<String, f64>,
    /// Threshold for regression detection (percentage, default: 5.0).
    /// A metric that drops by more than this percentage is flagged as regressed.
    #[serde(default = "default_threshold")]
    threshold: f64,
}

fn default_threshold() -> f64 {
    5.0
}

#[derive(Debug, Serialize)]
struct DecideOutput {
    suggestion: String,
    deltas: HashMap<String, DeltaInfo>,
    summary: String,
}

#[derive(Debug, Serialize)]
struct DeltaInfo {
    before: f64,
    after: f64,
    delta: f64,
    percent_change: f64,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct LogParams {
    /// Iteration number in the improvement loop
    iteration: u32,
    /// Description of the action taken (e.g. "added tests for auth module")
    action: String,
    /// Metrics snapshot as key-value pairs
    metrics: HashMap<String, f64>,
    /// The verdict: "keep", "revert", or any custom string
    verdict: String,
}

#[derive(Debug, Serialize)]
struct LogOutput {
    logged: bool,
    file: String,
    total_entries: usize,
}

// --- Tool implementations ---

#[tool_router]
impl TeambMetrics {
    #[tool(
        name = "teamb_measure",
        description = "Execute a shell command and extract numeric metrics from its output using regex patterns. Each pattern should contain one capture group that matches a number."
    )]
    async fn measure(&self, Parameters(params): Parameters<MeasureParams>) -> String {
        match self.do_measure(params).await {
            Ok(json) => json,
            Err(e) => format!("{{\"error\": \"{e}\"}}"),
        }
    }

    #[tool(
        name = "teamb_decide",
        description = "Compare before/after metrics and suggest whether to keep or revert the change. Returns deltas with percentage changes and a suggestion (improved/regressed/unchanged). The final decision is always made by the calling agent, not this tool."
    )]
    async fn decide(&self, Parameters(params): Parameters<DecideParams>) -> String {
        match self.do_decide(params) {
            Ok(json) => json,
            Err(e) => format!("{{\"error\": \"{e}\"}}"),
        }
    }

    #[tool(
        name = "teamb_log",
        description = "Append a log entry to the improvement loop TSV log file. Records iteration number, action taken, metrics snapshot, and verdict. The log is stored in the plugin's persistent data directory."
    )]
    async fn log(&self, Parameters(params): Parameters<LogParams>) -> String {
        match self.do_log(params).await {
            Ok(json) => json,
            Err(e) => format!("{{\"error\": \"{e}\"}}"),
        }
    }
}

// --- Business logic (separated for clean error handling) ---

impl TeambMetrics {
    async fn do_measure(&self, params: MeasureParams) -> anyhow::Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&params.command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{stdout}{stderr}");

        let mut metrics = HashMap::new();
        for (key, pattern) in &params.patterns {
            let re = Regex::new(pattern)?;
            if let Some(caps) = re.captures(&combined) {
                if let Some(m) = caps.get(1) {
                    if let Ok(val) = m.as_str().parse::<f64>() {
                        metrics.insert(key.clone(), val);
                    }
                }
            }
        }

        let result = MeasureOutput {
            command: params.command,
            exit_code: output.status.code().unwrap_or(-1),
            metrics,
            raw_output: truncate(&combined, 4000),
            timestamp: Utc::now().to_rfc3339(),
        };

        Ok(serde_json::to_string_pretty(&result)?)
    }

    fn do_decide(&self, params: DecideParams) -> anyhow::Result<String> {
        let mut deltas = HashMap::new();
        let mut improved_count = 0u32;
        let mut regressed_count = 0u32;

        let all_keys: std::collections::HashSet<&String> =
            params.before.keys().chain(params.after.keys()).collect();

        for key in all_keys {
            let before = params.before.get(key).copied().unwrap_or(0.0);
            let after = params.after.get(key).copied().unwrap_or(0.0);
            let delta = after - before;
            let percent_change = if before.abs() > f64::EPSILON {
                (delta / before) * 100.0
            } else if delta.abs() > f64::EPSILON {
                if delta > 0.0 { 100.0 } else { -100.0 }
            } else {
                0.0
            };

            if percent_change > params.threshold {
                improved_count += 1;
            } else if percent_change < -params.threshold {
                regressed_count += 1;
            }

            deltas.insert(
                key.clone(),
                DeltaInfo {
                    before,
                    after,
                    delta,
                    percent_change: (percent_change * 100.0).round() / 100.0,
                },
            );
        }

        let suggestion = if regressed_count > 0 {
            "regressed"
        } else if improved_count > 0 {
            "improved"
        } else {
            "unchanged"
        };

        let summary = format!(
            "{} metrics: {} improved, {} regressed (threshold: {}%)",
            deltas.len(),
            improved_count,
            regressed_count,
            params.threshold
        );

        let result = DecideOutput {
            suggestion: suggestion.to_string(),
            deltas,
            summary,
        };

        Ok(serde_json::to_string_pretty(&result)?)
    }

    async fn do_log(&self, params: LogParams) -> anyhow::Result<String> {
        tokio::fs::create_dir_all(&self.data_dir).await?;

        let log_file = self.data_dir.join("loop-log.tsv");
        let file_exists = log_file.exists();

        let metrics_json = serde_json::to_string(&params.metrics)?;
        let timestamp = Utc::now().to_rfc3339();

        let mut content = String::new();
        if !file_exists {
            content.push_str("timestamp\titeration\taction\tmetrics\tverdict\n");
        }
        content.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            timestamp, params.iteration, params.action, metrics_json, params.verdict
        ));

        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .await?;
        file.write_all(content.as_bytes()).await?;

        let total = tokio::fs::read_to_string(&log_file)
            .await
            .map(|s| s.lines().count().saturating_sub(1))
            .unwrap_or(1);

        let result = LogOutput {
            logged: true,
            file: log_file.display().to_string(),
            total_entries: total,
        };

        Ok(serde_json::to_string_pretty(&result)?)
    }
}

fn truncate(s: &str, max_chars: usize) -> String {
    if s.len() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!(
            "{truncated}\n... (truncated, {total} chars total)",
            total = s.len()
        )
    }
}

// --- ServerHandler ---

#[tool_handler(router = self.tool_router)]
impl ServerHandler for TeambMetrics {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .build(),
        )
    }
}

// --- Entry point ---

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let data_dir = std::env::var("CLAUDE_PLUGIN_DATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".claude")
                .join("plugin-data")
                .join("team-bucciarati")
        });

    let server = TeambMetrics::new(data_dir);
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;

    Ok(())
}
