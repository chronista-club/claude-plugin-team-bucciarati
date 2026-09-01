---
name: team-bucciarati
description: "JoJo Part 5 Stand-themed quality team for strong, beautiful code — research before development, tests and reviews after. Use this skill when asked about team composition or how to call the stands."
triggers:
  - "チーム"
  - "ブチャラティ"
  - "スタンド"
  - "team"
---

# Team Bucciarati

JoJo Part 5「チーム・ブチャラティ」をモチーフにした3体のスタンド・エージェントチーム。

**開発の前後を支える品質チーム。** 前（調査）と後（テスト・レビュー）で強く美しいコードに貢献する。真ん中（実装）はユーザーとメインセッションの領分。

**チームの終点は「コミット可能な working tree」。** コミット・PR・マージ・デプロイもユーザーとメインセッションの領分（team-b はコミットラインを越えない）。

## チームロスター

| Stand | User | Role | Model | 能力 |
|-------|------|------|-------|------|
| **Purple Haze** | Fugo | Research | opus | 前: 深掘り調査、副作用なし |
| **Spice Girl** | Trish | Test Generation | sonnet | 後: t-wada流テストピラミッド |
| **Moody Blues** | Abbacchio | Quality Gate | sonnet | 後: ローカルチェック + 多角的コードレビュー |

> **モデル配分の考え方**: 思考の深さが質を決める仕事（調査）= opus。規律とプロセスが質を決める頻出の仕事（レビュー・テスト）= sonnet。さらに深い思考が必要な場面では、呼び出し時に fable へのエスカレーションを指定できる。

## 使い方

各スタンドは独立したエージェントとして直接呼び出す:

- 「Purple Haze で調べて」→ 着手前の深掘り調査（副作用なし）
- 「Spice Girl でテスト書いて」→ テストリスト設計・テスト生成
- 「Moody Blues でレビューして」→ コミット前のローカル品質ゲート

high-stakes な成果物（本番直行のコード、外部公開ドキュメント等）には、独立2レビュアーの敵対的検証 — [santa-method](../santa-method/SKILL.md) スキルを使う。

## MCP ツール連携

各スタンドは利用可能な MCP ツールを活用して能力を強化する。詳細は [reference/mcp-tools.md](reference/mcp-tools.md) を参照。

| MCP | 用途 | 使うスタンド |
|-----|------|-------------|
| **gitnexus** | コードベースナレッジグラフ | 全スタンド |
| **serena** | シンボリックコード解析 | Purple Haze, Moody Blues, Spice Girl |
| **context7** | ライブラリドキュメント | Purple Haze, Spice Girl |

> 全て**オプショナル** — MCP が利用不可でも各スタンドは動作する。

- 副作用ガード（コミットライン・破壊コマンド）のフック定義: [reference/hooks.md](reference/hooks.md)
- レビュー観点 ↔ Stand 対応表: [reference/stand-mapping.md](reference/stand-mapping.md)
- スタンドパラメータ（JoJo フレーバー）: [reference/stand-params.md](reference/stand-params.md)

## 連携ルール

1. **責務分離** — 各スタンドは自分の責務のみ実行し、他のスタンドの領域に踏み込まない
2. **コミットラインを越えない** — どのスタンドも commit / push / PR / deploy をしない
