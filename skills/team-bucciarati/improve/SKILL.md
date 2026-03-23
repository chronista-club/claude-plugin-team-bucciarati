---
name: improve
description: Run an autonomous improvement loop on the codebase. Measures metrics (test coverage, lint errors, etc.), makes targeted improvements, compares results, and iterates until the goal is met or max iterations reached. Use when asked to "improve tests", "increase coverage", "fix all lint errors", or "make the code better".
user-invocable: true
argument-hint: "[goal] e.g. 'テストカバレッジを80%に', 'lint エラーをゼロに'"
allowed-tools:
  - Bash
  - Read
  - Edit
  - Write
  - Glob
  - Grep
  - Agent
  - mcp__teamb-metrics__teamb_measure
  - mcp__teamb-metrics__teamb_decide
  - mcp__teamb-metrics__teamb_log
---

# Improve — 自律改善ループ

autoresearch 的な「放置して勝手に良くなる」改善ループ。
メトリクスを計測 → 改善 → 再計測 → 比較判断を繰り返す。

## ループフロー

```
1. ゴール確認 — ユーザーの目標を明確化（例: カバレッジ 80%）
2. ベースライン計測 — teamb_measure でメトリクス取得
3. 改善実行 — Spice Girl（テスト）or 直接編集
4. 再計測 — teamb_measure で改善後メトリクス取得
5. 比較判断 — teamb_decide の提案を参考に、文脈込みで最終判断
   - keep → teamb_log に記録、次のイテレーションへ
   - revert → git checkout で戻す、別アプローチを試行
6. 終了判定 — 目標達成 or 最大イテレーション到達 or 改善停滞
```

## 起動時にやること

### 1. ゴールのヒアリング

ユーザーの指示から以下を特定:

| 項目 | 例 |
|------|---|
| **対象メトリクス** | テストカバレッジ、lint エラー数、テスト pass 数 |
| **目標値** | 80%、0件、全 pass |
| **スコープ** | プロジェクト全体、特定モジュール |
| **最大イテレーション** | デフォルト 5回 |

### 2. メトリクス計測の設定

`teamb_measure` に渡すコマンドとパターンを決定する。

よくあるパターン:

**テストカバレッジ（Rust）:**
```json
{
  "command": "cargo tarpaulin --skip-clean 2>&1",
  "patterns": {
    "coverage": "(\\d+\\.\\d+)% coverage",
    "pass": "(\\d+) passed",
    "fail": "(\\d+) failed"
  }
}
```

**テストカバレッジ（Node/Bun）:**
```json
{
  "command": "bun test --coverage 2>&1",
  "patterns": {
    "pass": "(\\d+) pass",
    "fail": "(\\d+) fail",
    "coverage": "All files.*?(\\d+\\.\\d+)"
  }
}
```

**Lint エラー（Biome）:**
```json
{
  "command": "bunx biome check . 2>&1; echo \"EXIT:$?\"",
  "patterns": {
    "errors": "Found (\\d+) error",
    "warnings": "Found (\\d+) warning"
  }
}
```

パターンが不明な場合は、まずコマンドを一度実行して出力を確認してからパターンを決める。

## 改善の実行

メトリクスの種類によって改善手法を選択:

| メトリクス | 手法 |
|-----------|------|
| テストカバレッジ | Spice Girl エージェントでテスト生成 |
| lint エラー | 直接修正（`biome check --write` or 手動） |
| テスト失敗 | バグ修正 |

**Spice Girl を呼ぶ場合:**
```
Agent(subagent_type: "team-bucciarati:spice-girl",
      prompt: "以下のモジュールにテストを追加: ...")
```

## 判断基準

`teamb_decide` の提案を受け取ったら、以下を考慮して最終判断:

- **improved** → 基本 keep。ただし改善幅が微小なら別アプローチ検討
- **regressed** → 基本 revert。ただしリファクタリングで一時的に下がるのは許容する場合がある
- **unchanged** → 別のアプローチを試行

### revert の方法
```bash
git checkout -- .  # 全ファイル戻し
# or
git checkout -- path/to/file  # 特定ファイルだけ
```

## 終了条件

以下のいずれかで終了:

1. **目標達成** — メトリクスが目標値に到達
2. **最大イテレーション** — デフォルト 5回
3. **改善停滞** — 3回連続で unchanged or regressed
4. **ユーザー中断** — Ctrl+C

## 最終レポート

```
## Improve Loop Report

### Goal
テストカバレッジを 80% に

### Result: ACHIEVED / PARTIAL / STALLED

### Iterations
| # | Action | Coverage | Verdict |
|---|--------|----------|---------|
| 0 | (baseline) | 65.2% | - |
| 1 | auth モジュールにテスト追加 | 71.8% | keep |
| 2 | handler テスト追加 | 78.3% | keep |
| 3 | エッジケーステスト | 81.1% | keep |

### Log
loop-log.tsv に記録済み
```
