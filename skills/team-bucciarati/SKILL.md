---
name: team-bucciarati
description: "JoJo Part 5 Stand-themed agent team for strong, beautiful code. Use this skill when asked about team composition, pipeline patterns, or dispatching agents."
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

各スタンドは**コードを強く美しくする**フェーズを担当し、Aerosmith がオーケストレーターとして統率する。

**チームの終点は「コミット可能な working tree」。** コミット・PR・マージ・デプロイはユーザーとメインセッションの領分（team-b はコミットラインを越えない）。

## チームロスター

| Stand | User | Role | Model | 能力 |
|-------|------|------|-------|------|
| **Aerosmith** | Narancia | Orchestrator | sonnet | コード品質パイプラインを俯瞰・制御 |
| **Purple Haze** | Fugo | Research | opus | 深掘り調査、副作用なし |
| **Gold Experience** | Giorno | Implementation | opus | 要件に生命を吹き込む: 理解 → 実装 → ローカル検証 |
| **Spice Girl** | Trish | Test Generation | sonnet | t-wada流テストピラミッド |
| **Moody Blues** | Abbacchio | Quality Gate | sonnet | ローカルチェック + 多角的コードレビュー |
| **Sticky Fingers** | Bucciarati | Refactoring | opus | 分解 → 移動 → 再結合。挙動を変えず構造を美しく |
| **Sex Pistols** | Mista | Parallel Code Work | sonnet | 並列一斉コード作業（4体禁止w） |

> **モデル配分の考え方**: 思考の深さが質を決める仕事（実装・リファクタ・調査）= opus。規律とプロセスが質を決める頻出の仕事（レビュー・テスト・統率・分配）= sonnet。さらに深い思考が必要な場面では、呼び出し時に fable へのエスカレーションを指定できる。

## MCP ツール連携

各スタンドは利用可能な MCP ツールを活用して能力を強化する。詳細は [reference/mcp-tools.md](reference/mcp-tools.md) を参照。

スタンド間のコンテキスト引き継ぎと Issue コンテキストの仕様は [reference/stand-context.md](reference/stand-context.md) を参照。

- 副作用ガード（コミットライン・破壊コマンド）のフック定義: [reference/hooks.md](reference/hooks.md)
- Sex Pistols の並列タスク指示規約: [reference/worker-conventions.md](reference/worker-conventions.md)
- スタンドパラメータ（JoJo フレーバー）: [reference/stand-params.md](reference/stand-params.md)

| MCP | 用途 | 使うスタンド |
|-----|------|-------------|
| **gitnexus** | コードベースナレッジグラフ | 全スタンド |
| **serena** | シンボリックコード解析 | Purple Haze, Gold Experience, Moody Blues, Sticky Fingers, Spice Girl |
| **context7** | ライブラリドキュメント | Purple Haze, Gold Experience, Spice Girl |
| **linear** | Issue 参照（読み取りのみ） | Aerosmith, Gold Experience |

> 全て**オプショナル** — MCP が利用不可でも各スタンドは動作する。

## パイプラインパターン

詳細は [reference/pipelines.md](reference/pipelines.md) を参照。

| Pattern | Flow | Use Case |
|---------|------|----------|
| **Finish**（デフォルト） | (Spice Girl) → Moody Blues | 手元の変更をコミット可能な品質に仕上げる |
| **Forge** | (Purple Haze) → Gold Experience → Spice Girl → Moody Blues | 要件から実装一式 |
| **Polish** | Sticky Fingers → Moody Blues | 挙動を変えずに構造改善 |
| **Barrage** | Sex Pistols → Moody Blues | 独立作業の並列一斉実行 |
| **Research** | Purple Haze | 調査のみ（副作用なし） |
| **Custom** | 自由に組み合わせ | 調査→テスト、実装のみ等 |

> 1スタンドで完結する場合は直接呼び出し（パイプライン不要）。詳細は [reference/pipelines.md](reference/pipelines.md)

## 使い方

### 直接呼び出し

各スタンドは独立したエージェントとして直接呼び出せる:

- 「Moody Blues でレビューして」→ Moody Blues エージェントが起動
- 「Sticky Fingers でリファクタして」→ Sticky Fingers エージェントが起動
- 「Gold Experience で実装して」→ Gold Experience エージェントが起動

### パイプライン実行

Aerosmith 経由でパイプラインを組む:

- 「この機能、実装からレビューまでやって」→ Aerosmith が Forge パイプラインを実行
- 「テスト書いてから仕上げて」→ Aerosmith が Finish パイプラインを実行

### /dispatch コマンド

`/dispatch` コマンドで Aerosmith を起動し、対話的にパイプラインを選択できる。

## 連携ルール

1. **責務分離** — 各スタンドは自分の責務のみ実行し、他のスタンドの領域に踏み込まない
2. **順次実行** — パイプラインは必ず順次実行。前のスタンドの結果を確認してから次へ
3. **停止条件** — Moody Blues が BLOCKED 判定、または任意のスタンドがエラーの場合、パイプライン停止
4. **結果引き継ぎ** — 各スタンドの出力を次のスタンドに渡す（diff サマリ、テスト状態 等）
5. **コミットラインを越えない** — どのスタンドも commit / push / PR / deploy をしない
