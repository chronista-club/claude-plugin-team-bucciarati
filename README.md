# Team Bucciarati

JoJo's Bizarre Adventure Part 5 "Vento Aureo" をモチーフにした、Claude Code 向けエージェントチームプラグイン。

7体のスタンド・エージェントが開発パイプラインの各フェーズを担当し、Aerosmith がオーケストレーターとして全体を統率する。

## Install

```bash
claude plugin install chronista-club/claude-plugin-team-bucciarati
```

## Team Roster

| Stand | User | Role | Model |
|-------|------|------|-------|
| Aerosmith | Narancia | Orchestrator | opus |
| Purple Haze | Fugo | Research | opus |
| Moody Blues | Abbacchio | Quality Gate | sonnet |
| Sticky Fingers | Bucciarati | Shipping | sonnet |
| Gold Experience | Giorno | Deploy | sonnet |
| Sex Pistols | Mista | Parallel Workers | sonnet |
| Spice Girl | Trish | Test Generation | sonnet |

## Pipeline Patterns

| Pattern | Flow |
|---------|------|
| Full Release | Purple Haze -> Moody Blues -> Sticky Fingers -> Gold Experience |
| Review & Ship | Moody Blues -> Sticky Fingers |
| Ship & Deploy | Sticky Fingers -> Gold Experience |
| Test & Ship | Spice Girl -> Moody Blues -> Sticky Fingers |
| Research Only | Purple Haze |
| Deploy Only | Gold Experience |
| Parallel Sprint | Sex Pistols |

## Usage

### Direct

Call any Stand agent directly:

```
Moody Blues でレビューして
Sticky Fingers でシップして
Gold Experience でデプロイして
```

### Pipeline via Aerosmith

Let Aerosmith orchestrate:

```
レビューからデプロイまで全部やって
```

### /dispatch Command

```
/dispatch
/dispatch Review & Ship
```

## License

MIT
