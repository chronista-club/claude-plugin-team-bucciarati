# Team Bucciarati Plugin

JoJo Part 5 スタンドをモチーフにした Claude Code エージェントチームプラグイン。

## 構成

```
.claude-plugin/     # プラグインメタデータ (plugin.json, marketplace.json, skills.txt)
agents/             # スタンドエージェント定義 (7体)
commands/           # スラッシュコマンド (/dispatch)
skills/             # スキル定義 (team-bucciarati)
```

## エージェント一覧

| Agent | 役割 |
|-------|------|
| Aerosmith | オーケストレーター — パイプライン全体を統率 |
| Purple Haze | 深層リサーチ・調査 |
| Moody Blues | CI チェック・コードレビュー |
| Sticky Fingers | コミット → PR → マージ |
| Gold Experience | デプロイ・ヘルスチェック |
| Sex Pistols | 並列ワーカー管理 |
| Spice Girl | テスト生成 |

## 開発ルール

- エージェント定義は `agents/*.md` に配置
- スキルは `skills/<name>/SKILL.md` に配置
- コマンドは `commands/*.md` に配置
- バージョンは `.claude-plugin/plugin.json` で管理
- リリース時は `marketplace.json` の version も同期すること
- コミットメッセージは日本語
