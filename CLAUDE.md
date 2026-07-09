# Team Bucciarati Plugin

JoJo Part 5 スタンドをモチーフにした Claude Code エージェントチームプラグイン。

**スコープ: 強く美しいコードに貢献するレイヤーまで。** チームの終点は「コミット可能な working tree」— コミット・PR・マージ・デプロイ（CI/CD 以降のフロー）は扱わない。

## 構成

```
.claude-plugin/     # プラグインメタデータ (plugin.json)
.mcp.json           # MCP サーバー定義 (teamb-metrics, auto-discovery)
agents/             # スタンドエージェント定義 (7体)
commands/           # スラッシュコマンド (/dispatch)
skills/             # スキル定義 (team-bucciarati, improve)
mcp-server/         # teamb-metrics MCP サーバー (Rust)
scripts/            # 共有スクリプト (detect-ci.sh)
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

- エージェント定義は `agents/*.md` に配置
- スキルは `skills/<name>/SKILL.md` に配置
- コマンドは `commands/*.md` に配置
- バージョンは `.claude-plugin/plugin.json` で管理し、リリース時は `CHANGELOG.md` にエントリを追加すること
- どのスタンドも commit / push / PR / deploy をしない（コミットライン厳守）
- コミットメッセージは日本語
