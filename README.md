# Team Bucciarati

JoJo's Bizarre Adventure Part 5 "Vento Aureo" をモチーフにした、Claude Code 向けエージェントチームプラグイン。

7体のスタンド・エージェントが**強く美しいコード**を作るための各フェーズを担当し、Aerosmith がオーケストレーターとして全体を統率する。

**チームの終点は「コミット可能な working tree」。** コミット・PR・マージ・デプロイはユーザーとメインセッションの領分 — team-b はコミットラインを越えない。

## Install

```bash
claude plugin install chronista-club/claude-plugin-team-bucciarati
```

## Team Roster

| Stand | User | Role | Model |
|-------|------|------|-------|
| Aerosmith | Narancia | Orchestrator | sonnet |
| Purple Haze | Fugo | Research | opus |
| Gold Experience | Giorno | Implementation | opus |
| Spice Girl | Trish | Test Generation | sonnet |
| Moody Blues | Abbacchio | Quality Gate | sonnet |
| Sticky Fingers | Bucciarati | Refactoring | opus |
| Sex Pistols | Mista | Parallel Code Work | sonnet |

> Model policy: deep-thinking work (implement / refactor / research) runs on opus; frequent, discipline-driven work (review / test / orchestrate) runs on sonnet.

## Pipeline Patterns

| Pattern | Flow | Use Case |
|---------|------|----------|
| Finish (default) | (Spice Girl) -> Moody Blues | 手元の変更をコミット可能な品質に仕上げる |
| Forge | (Purple Haze) -> Gold Experience -> Spice Girl -> Moody Blues | 要件から実装一式 |
| Polish | Sticky Fingers -> Moody Blues | 挙動を変えずに構造改善 |
| Barrage | Sex Pistols -> Moody Blues | 独立作業の並列一斉実行 |
| Research | Purple Haze | 調査のみ |

## Usage

### Direct

Call any Stand agent directly:

```
Moody Blues でレビューして
Gold Experience で実装して
Sticky Fingers でリファクタして
```

### Pipeline via Aerosmith

Let Aerosmith orchestrate:

```
この機能、実装からレビューまで全部やって
```

### /dispatch Command

```
/dispatch
/dispatch forge
```

## License

MIT
