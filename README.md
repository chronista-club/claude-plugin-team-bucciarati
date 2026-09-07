> **保守終了・アーカイブ（2026-09-07）**
>
> このリポジトリの保守は終了しました。今後の開発・更新は [plugin-team-bucciarati](https://github.com/chronista-club/plugin-team-bucciarati) で行います。
> 導入・移行方法は [新カタログ chronista-plugins](https://github.com/chronista-club/chronista-plugins#旧配布先からの移行) を参照してください。
> 旧・新カタログの marketplace 名は同じ `chronista-plugins` です。旧登録を利用中の場合は、設定を退避したうえで旧登録を解除し、新カタログから再導入してください。
> 以下は保守終了時点の記録です。

# Team Bucciarati

JoJo's Bizarre Adventure Part 5 "Vento Aureo" をモチーフにした、Claude Code 向けエージェントチームプラグイン。

4体のスタンド・エージェントが**開発の前後**を支える — 前（調査）と後（テスト・レビュー）で強く美しいコードに貢献する品質チーム。真ん中（実装）はユーザーとメインセッションの領分。

**チームの終点は「コミット可能な working tree」。** コミット・PR・マージ・デプロイはユーザーとメインセッションの領分 — team-b はコミットラインを越えない。

## Install

```bash
claude plugin install chronista-club/claude-plugin-team-bucciarati
```

## Team Roster

| Stand | User | Role | Model |
|-------|------|------|-------|
| Purple Haze | Fugo | Research | opus |
| Spice Girl | Trish | Test Generation | sonnet |
| Moody Blues | Abbacchio | Quality Gate | opus |
| Sticky Fingers | Bucciarati | Adversarial Verification | opus |

> Model policy: work whose failures are silent (research / review / adversarial verification) runs on opus; self-verifying work (test generation) runs on sonnet.

## Usage

Call any Stand agent directly:

```
Purple Haze で調べて
Spice Girl でテスト書いて
Moody Blues でレビューして
santa レビューして（Moody Blues × Sticky Fingers の敵対的 dual review）
```

Review depth menu — quick (Moody Blues solo) / deep (8-pass parallel) / adversarial (santa-method). See `skills/team-bucciarati/SKILL.md`.

## License

MIT
