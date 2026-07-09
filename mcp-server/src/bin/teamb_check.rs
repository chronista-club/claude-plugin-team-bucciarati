//! teamb-check — team.kdl (SSOT) と各ドキュメントのドリフト検出
//!
//! team.kdl の parse は club-kdl の derive（型付き struct ↔ KDL）で行う。
//! team.kdl が持つ構造化ファクト（ロスター・モデル配分・パイプライン・コミットライン境界）が
//! agents/*.md frontmatter / SKILL.md / README.md / CLAUDE.md / reference/pipelines.md /
//! commands/dispatch.md / plugin.json / CHANGELOG.md と一致しているかを検証する。
//!
//! Usage: teamb-check [--root <plugin-repo-root>]

use anyhow::{Context, Result, bail};
use club_kdl::KdlDeserialize;
use std::fs;
use std::path::{Path, PathBuf};

// ---------- team.kdl の型定義（SSOT スキーマ） ----------

#[derive(Debug, KdlDeserialize)]
#[kdl(name = "team")]
struct Team {
    #[kdl(argument)]
    #[allow(dead_code)]
    name: String,

    #[kdl(child, unwrap_arg)]
    #[allow(dead_code)]
    scope: String,

    #[kdl(child)]
    commit_line: CommitLine,

    #[kdl(child)]
    roster: Roster,

    #[kdl(child)]
    pipelines: Pipelines,
}

#[derive(Debug, KdlDeserialize)]
#[kdl(name = "commit-line")]
struct CommitLine {
    #[kdl(children)]
    forbidden: Vec<Forbidden>,
}

#[derive(Debug, KdlDeserialize)]
#[kdl(name = "forbidden")]
struct Forbidden {
    #[kdl(argument)]
    value: String,
}

#[derive(Debug, KdlDeserialize)]
#[kdl(name = "roster")]
struct Roster {
    #[kdl(children)]
    stands: Vec<Stand>,
}

#[derive(Debug, KdlDeserialize)]
#[kdl(name = "stand")]
struct Stand {
    #[kdl(argument)]
    name: String,
    #[kdl(property)]
    display: String,
    #[kdl(property)]
    user: String,
    #[kdl(property)]
    role: String,
    #[kdl(property)]
    model: String,
    #[kdl(property)]
    color: String,
}

#[derive(Debug, KdlDeserialize)]
#[kdl(name = "pipelines")]
struct Pipelines {
    #[kdl(children)]
    pipelines: Vec<Pipeline>,
}

#[derive(Debug, KdlDeserialize)]
#[kdl(name = "pipeline")]
struct Pipeline {
    #[kdl(argument)]
    name: String,
    #[kdl(property, default)]
    default: bool,
    #[kdl(property(rename = "use-case"))]
    use_case: String,
    #[kdl(children)]
    steps: Vec<Step>,
}

#[derive(Debug, KdlDeserialize)]
#[kdl(name = "step")]
struct Step {
    #[kdl(argument)]
    stand: String,
    #[kdl(property, default)]
    #[allow(dead_code)]
    optional: bool,
}

// ---------- ヘルパー ----------

fn read(root: &Path, rel: &str) -> Result<String> {
    fs::read_to_string(root.join(rel)).with_context(|| format!("{rel} が読めません"))
}

/// frontmatter (--- ... ---) から `key: value` を取り出す
fn frontmatter_value(md: &str, key: &str) -> Option<String> {
    let mut in_fm = false;
    for line in md.lines() {
        if line.trim() == "---" {
            if in_fm {
                break;
            }
            in_fm = true;
            continue;
        }
        if in_fm && let Some(rest) = line.strip_prefix(&format!("{key}:")) {
            return Some(rest.trim().to_string());
        }
    }
    None
}

// ---------- 検証 ----------

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let root = match args.iter().position(|a| a == "--root") {
        Some(i) => PathBuf::from(args.get(i + 1).context("--root にパスが必要")?),
        None => PathBuf::from("."),
    };

    let team: Team =
        club_kdl::from_str(&read(&root, "team.kdl")?).context("team.kdl の parse に失敗")?;
    let stands = &team.roster.stands;
    let pipelines = &team.pipelines.pipelines;

    if stands.is_empty() {
        bail!("roster にスタンドが定義されていません");
    }

    let mut errors: Vec<String> = Vec::new();
    let mut err = |msg: String| errors.push(msg);

    // --- 1. agents/*.md frontmatter ---
    for s in stands {
        let rel = format!("agents/{}.md", s.name);
        let md = match read(&root, &rel) {
            Ok(m) => m,
            Err(e) => {
                err(format!("{e:#}"));
                continue;
            }
        };
        for (key, expected) in [("name", &s.name), ("model", &s.model), ("color", &s.color)] {
            match frontmatter_value(&md, key) {
                Some(v) if v == *expected => {}
                got => err(format!("{rel}: frontmatter {key} = {got:?}、期待 {expected:?}")),
            }
        }
    }

    // --- 2. ロスター表 (SKILL.md / README.md / CLAUDE.md) ---
    let skill = read(&root, "skills/team-bucciarati/SKILL.md")?;
    let readme = read(&root, "README.md")?;
    let claude_md = read(&root, "CLAUDE.md")?;
    for s in stands {
        // SKILL/README の表は作中の呼び名（名 or 姓）を使うため、user のいずれかのトークンで照合
        let user_ok = |row: &str| {
            s.user
                .split_whitespace()
                .any(|token| row.contains(&format!("| {token} |")))
        };
        let skill_row = skill
            .lines()
            .find(|l| l.contains(&format!("**{}**", s.display)) && l.starts_with('|'));
        match skill_row {
            Some(row) => {
                for (what, needle) in [
                    ("モデル", format!("| {} |", s.model)),
                    ("ロール", format!("| {} |", s.role)),
                ] {
                    if !row.contains(&needle) {
                        err(format!(
                            "SKILL.md: {} の行の{what}が team.kdl と不一致（期待: {needle}）",
                            s.display
                        ));
                    }
                }
                if !user_ok(row) {
                    err(format!(
                        "SKILL.md: {} の行のユーザーが team.kdl ({}) と不一致",
                        s.display, s.user
                    ));
                }
            }
            None => err(format!("SKILL.md: ロスター表に {} の行がありません", s.display)),
        }
        let readme_row = readme
            .lines()
            .find(|l| l.contains(&format!("| {} |", s.display)));
        match readme_row {
            Some(row) => {
                for (what, needle) in [
                    ("モデル", format!("| {} |", s.model)),
                    ("ロール", format!("| {} |", s.role)),
                ] {
                    if !row.contains(&needle) {
                        err(format!(
                            "README.md: {} の行の{what}が team.kdl と不一致（期待: {needle}）",
                            s.display
                        ));
                    }
                }
            }
            None => err(format!("README.md: ロスター表に {} の行がありません", s.display)),
        }
        if !claude_md.contains(&format!("| {} |", s.display)) {
            err(format!("CLAUDE.md: エージェント一覧に {} がありません", s.display));
        }
    }

    // --- 3. パイプライン ---
    let default_count = pipelines.iter().filter(|p| p.default).count();
    if default_count != 1 {
        err(format!("default パイプラインは1つであるべき（現在 {default_count}）"));
    }
    let pipelines_md = read(&root, "skills/team-bucciarati/reference/pipelines.md")?;
    let dispatch_md = read(&root, "commands/dispatch.md")?;
    let stand_names: Vec<&str> = stands.iter().map(|s| s.name.as_str()).collect();
    for p in pipelines {
        if !pipelines_md.contains(&format!("## {}", p.name)) {
            err(format!("pipelines.md: `## {}` セクションがありません", p.name));
        }
        if !dispatch_md.contains(&format!("**{}**", p.name)) {
            err(format!("dispatch.md: パイプライン {} への言及がありません", p.name));
        }
        match skill
            .lines()
            .find(|l| l.contains(&format!("**{}**", p.name)) && l.starts_with('|'))
        {
            Some(row) if row.contains(&p.use_case) => {}
            Some(_) => err(format!(
                "SKILL.md: パイプライン {} の use-case が team.kdl と不一致（期待: {}）",
                p.name, p.use_case
            )),
            None => err(format!("SKILL.md: パイプライン {} の行がありません", p.name)),
        }
        for step in &p.steps {
            if !stand_names.contains(&step.stand.as_str()) {
                err(format!(
                    "team.kdl: pipeline {} の step {:?} が roster に存在しません",
                    p.name, step.stand
                ));
            }
        }
    }

    // --- 4. コミットライン・ガード ---
    let hooks_md = read(&root, "skills/team-bucciarati/reference/hooks.md")?;
    for f in &team.commit_line.forbidden {
        let spaced = f.value.replace(' ', "\\s+");
        if !hooks_md.contains(f.value.as_str()) && !hooks_md.contains(&spaced) {
            err(format!(
                "hooks.md: forbidden {:?} に対応するガード記述がありません",
                f.value
            ));
        }
    }

    // --- 5. version 同期 (plugin.json ↔ CHANGELOG) ---
    let plugin_json: serde_json::Value =
        serde_json::from_str(&read(&root, ".claude-plugin/plugin.json")?)
            .context("plugin.json の parse に失敗")?;
    let version = plugin_json["version"].as_str().unwrap_or_default();
    let changelog = read(&root, "CHANGELOG.md")?;
    let latest = changelog
        .lines()
        .find(|l| l.starts_with("## ["))
        .and_then(|l| l.split(['[', ']']).nth(1))
        .unwrap_or_default();
    if version != latest {
        err(format!(
            "version 不一致: plugin.json = {version} / CHANGELOG 最新 = {latest}"
        ));
    }

    // --- 結果 ---
    if errors.is_empty() {
        println!(
            "✔ team.kdl SSOT 整合 OK — stands: {} / pipelines: {} / version: {}",
            stands.len(),
            pipelines.len(),
            version
        );
        Ok(())
    } else {
        eprintln!("✘ SSOT ドリフト検出: {} 件", errors.len());
        for e in &errors {
            eprintln!("  - {e}");
        }
        std::process::exit(1);
    }
}
