# Team Bucciarati Plugin

JoJo Part 5 スタンドをモチーフにした Claude Code エージェントチームプラグイン。

**スコープ: 開発の前後（調査・テスト・レビュー・検証）で強く美しいコードに貢献するレイヤーまで。** チームの終点は「コミット可能な working tree」— 真ん中（実装）とコミット・PR・マージ・デプロイ（CI/CD 以降のフロー）はユーザーとメインセッションの領分。

## 構成

```
team.kdl            # ★ SSOT — ロスター/モデル配分/コミットライン境界の構造化ファクト
.claude-plugin/     # プラグインメタデータ (plugin.json)
agents/             # スタンドエージェント定義 (4体) — 散文の正はこちら
skills/             # スキル定義 (team-bucciarati, santa-method)
mcp-server/         # teamb-check — SSOT ドリフト検出器 (Rust)
scripts/            # 共有スクリプト (detect-ci.sh, check-teamdef.sh, nightly-release.sh)
```

## エージェント一覧

| Agent | 役割 |
|-------|------|
| Purple Haze | 前: 深層リサーチ・調査 |
| Spice Girl | 後: テスト生成 |
| Moody Blues | 後: ローカル品質チェック・コードレビュー |
| Sticky Fingers | 後: 敵対的検証 — 嘘の味（santa-method の独立レビュアー） |

## リリースフロー（nightly 積み方式）

```
昼   feature PR → squash merge → main に積む（リリースしない）
夜   scripts/nightly-release.sh が自動棚卸し（毎晩 23:30、スケジュールタスク）
     新コミットあり → 品質ゲート（check-teamdef / cargo test / clippy）
     → green なら nightly-YYYYMMDD タグ + GitHub prerelease
安定  人間の判断で cut: version bump + CHANGELOG + Release + marketplace 同期
```

- **nightly は plugin.json / marketplace に触れない** — git スナップショット + prerelease のみ。ユーザーに届く経路は安定版だけ（0.17.x のインストール事故の教訓）
- nightly が品質ゲートで落ちたら翌朝調査（夜中に main を勝手に直さない）

## 開発ルール

- **SSOT は `team.kdl`** — ロスター（モデル・カラー）・コミットライン境界を変更する時は必ず team.kdl から更新し、`scripts/check-teamdef.sh` でドキュメント群との整合を検証すること（コミット前必須）
- エージェント定義の散文は `agents/*.md` が正（team.kdl は構造化ファクトのみ持つ）
- スキルは `skills/<name>/SKILL.md` に配置
- バージョンは `.claude-plugin/plugin.json` で管理し、リリース時は `CHANGELOG.md` にエントリを追加すること
- どのスタンドも commit / push / PR / deploy をしない（コミットライン厳守）
- コミットメッセージは日本語
