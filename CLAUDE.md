# Team Bucciarati Plugin

JoJo Part 5 スタンドをモチーフにした Claude Code エージェントチームプラグイン。

**スコープ: 強く美しいコードに貢献するレイヤーまで。** チームの終点は「コミット可能な working tree」— コミット・PR・マージ・デプロイ（CI/CD 以降のフロー）は扱わない。

## 構成

```
team.kdl            # ★ SSOT — ロスター/モデル配分/パイプライン/境界の構造化ファクト
.claude-plugin/     # プラグインメタデータ (plugin.json)
.mcp.json           # MCP サーバー定義 (teamb-metrics, auto-discovery)
agents/             # スタンドエージェント定義 (7体) — 散文の正はこちら
commands/           # スラッシュコマンド (/dispatch)
skills/             # スキル定義 (team-bucciarati, improve)
mcp-server/         # teamb-metrics MCP サーバー + teamb-check (Rust)
scripts/            # 共有スクリプト (detect-ci.sh, check-teamdef.sh)
```

## エージェント一覧

| Agent | 役割 |
|-------|------|
| Aerosmith | オーケストレーター — コード品質パイプラインを統率 |
| Purple Haze | 深層リサーチ・調査 |
| Gold Experience | 実装 — 要件に生命を吹き込む |
| Spice Girl | テスト生成 |
| Moody Blues | ローカル品質チェック・コードレビュー |
| Sticky Fingers | リファクタリング — 分解・移動・再結合 |
| Sex Pistols | 並列コード作業 |

## 開発ルール

- **SSOT は `team.kdl`** — ロスター（モデル・カラー）・パイプライン・コミットライン境界を変更する時は必ず team.kdl から更新し、`scripts/check-teamdef.sh` でドキュメント群との整合を検証すること（コミット前必須）
- エージェント定義の散文は `agents/*.md` が正（team.kdl は構造化ファクトのみ持つ）
- エージェント定義は `agents/*.md` に配置
- スキルは `skills/<name>/SKILL.md` に配置
- コマンドは `commands/*.md` に配置
- バージョンは `.claude-plugin/plugin.json` で管理し、リリース時は `CHANGELOG.md` にエントリを追加すること
- どのスタンドも commit / push / PR / deploy をしない（コミットライン厳守）
- コミットメッセージは日本語
