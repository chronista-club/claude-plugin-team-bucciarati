---
name: team-bucciarati
description: "JoJo Part 5 Stand-themed agent team for software development pipelines. Use this skill when asked about team composition, pipeline patterns, or dispatching agents."
triggers:
  - "チーム"
  - "パイプライン"
  - "ブチャラティ"
  - "ディスパッチ"
  - "スタンド"
  - "pipeline"
  - "dispatch"
  - "team"
---

# Team Bucciarati

JoJo Part 5「チーム・ブチャラティ」をモチーフにした7体のスタンド・エージェントチーム。

各スタンドは開発パイプラインの特定フェーズを担当し、Aerosmith がオーケストレーターとして統率する。

## チームロスター

| Stand | User | Role | Model | 能力 |
|-------|------|------|-------|------|
| **Aerosmith** | Narancia | Orchestrator | opus | パイプライン全体を俯瞰・制御 |
| **Purple Haze** | Fugo | Research | opus | 深掘り調査、副作用なし |
| **Moody Blues** | Abbacchio | Quality Gate | sonnet | CI + 多角的コードレビュー |
| **Sticky Fingers** | Bucciarati | Shipping | sonnet | commit → push → PR → merge |
| **Gold Experience** | Giorno | Deploy | sonnet | build → migrate → deploy → health check |
| **Sex Pistols** | Mista | Parallel Workers | sonnet | 並列ワーカー管理（4体禁止w） |
| **Spice Girl** | Trish | Test Generation | sonnet | t-wada流テストピラミッド |

## MCP ツール連携

各スタンドは利用可能な MCP ツールを活用して能力を強化する。詳細は [reference/mcp-tools.md](reference/mcp-tools.md) を参照。

スタンド間のコンテキスト引き継ぎと Issue コンテキストの仕様は [reference/stand-context.md](reference/stand-context.md) を参照。

| MCP | 用途 | 使うスタンド |
|-----|------|-------------|
| **gitnexus** | コードベースナレッジグラフ | 全スタンド（`rename` 以外の6ツールを活用） |
| **serena** | シンボリックコード解析 | Purple Haze, Moody Blues, Spice Girl |
| **context7** | ライブラリドキュメント | Purple Haze, Spice Girl |
| **linear** | Issue 管理 | Aerosmith, Sticky Fingers |

> 全て**オプショナル** — MCP が利用不可でも各スタンドは動作する。

## パイプラインパターン

詳細は [reference/pipelines.md](reference/pipelines.md) を参照。

| Pattern | Flow | Use Case |
|---------|------|----------|
| **Ship**（デフォルト） | Moody Blues → Sticky Fingers | レビュー → シップ（日常の 80%） |
| **Full** | (Purple Haze) → Moody Blues → Sticky Fingers → (Gold Experience) | フルパイプライン |
| **Deploy** | Gold Experience | デプロイのみ |
| **Custom** | 自由に組み合わせ | テスト&シップ、並列スプリント等 |

> 1スタンドで完結する場合は直接呼び出し（パイプライン不要）。詳細は [reference/pipelines.md](reference/pipelines.md)

## 使い方

### 直接呼び出し

各スタンドは独立したエージェントとして直接呼び出せる:

- 「Moody Blues でレビューして」→ Moody Blues エージェントが起動
- 「Sticky Fingers でシップして」→ Sticky Fingers エージェントが起動

### パイプライン実行

Aerosmith 経由でパイプラインを組む:

- 「レビューからデプロイまで全部やって」→ Aerosmith が Full Release パイプラインを実行
- 「テスト書いてからシップして」→ Aerosmith が Test & Ship パイプラインを実行

### /dispatch コマンド

`/dispatch` コマンドで Aerosmith を起動し、対話的にパイプラインを選択できる。

## 連携ルール

1. **責務分離** — 各スタンドは自分の責務のみ実行し、他のスタンドの領域に踏み込まない
2. **順次実行** — パイプラインは必ず順次実行。前のスタンドの結果を確認してから次へ
3. **停止条件** — Moody Blues が BLOCKED 判定、または任意のスタンドがエラーの場合、パイプライン停止
4. **結果引き継ぎ** — 各スタンドの出力を次のスタンドに渡す（PR番号、デプロイURL 等）
