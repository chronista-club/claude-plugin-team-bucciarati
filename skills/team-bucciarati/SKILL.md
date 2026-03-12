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

## パイプラインパターン

詳細は [reference/pipelines.md](reference/pipelines.md) を参照。

| Pattern | Flow | Use Case |
|---------|------|----------|
| **Full Release** | Purple Haze → Moody Blues → Sticky Fingers → Gold Experience | フル機能リリース |
| **Review & Ship** | Moody Blues → Sticky Fingers | レビュー後にシップ |
| **Ship & Deploy** | Sticky Fingers → Gold Experience | シップ後にデプロイ |
| **Test & Ship** | Spice Girl → Moody Blues → Sticky Fingers | テスト強化してシップ |
| **Research Only** | Purple Haze | 調査のみ |
| **Deploy Only** | Gold Experience | デプロイのみ |
| **Parallel Sprint** | Sex Pistols | 複数タスク並列実行 |
| **Issue Pipeline** | Aerosmith → Sticky Fingers → (Moody Blues →) Gold Experience | Issue 起点のエンドツーエンド |

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
