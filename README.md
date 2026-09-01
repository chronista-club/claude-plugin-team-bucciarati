# Team Bucciarati

JoJo's Bizarre Adventure Part 5 "Vento Aureo" をモチーフにした、Claude Code 向けエージェントチームプラグイン。

3体のスタンド・エージェントが**開発の前後**を支える — 前（調査）と後（テスト・レビュー）で強く美しいコードに貢献する品質チーム。真ん中（実装）はユーザーとメインセッションの領分。

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
| Moody Blues | Abbacchio | Quality Gate | sonnet |

> Model policy: deep-thinking work (research) runs on opus; frequent, discipline-driven work (test / review) runs on sonnet.

## Usage

Call any Stand agent directly:

```
Purple Haze で調べて
Spice Girl でテスト書いて
Moody Blues でレビューして
```

For high-stakes deliverables, use the adversarial dual-review skill `santa-method`.

## License

MIT
