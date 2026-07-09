# ピストルズ指示規約

Sex Pistols が並列サブエージェント（Agent ツール）に渡すタスク指示と、受け取る完了報告の規約。

## タスク指示（Lead → ピストルズ）

各サブエージェントへのプロンプトは**自己完結**させる。以下の構造を含めること:

```
## Task
fetchV1 → fetchV2 への API 移行

## Files（担当）
- src/api/users.ts
- src/api/posts.ts

## Files（触ってはいけない）
- src/api/client.ts（No.3 の担当）
- src/services/**（No.2 の担当）

## Change
before:
  const res = await fetchV1(url, params)
after:
  const res = await fetchV2({ url, ...params })

## Acceptance
- 担当ファイル内の fetchV1 呼び出しが 0 件
- typecheck が通る（担当ファイルのみで確認）

## Report
完了時に以下を報告: 変更ファイル一覧、変更箇所数、判断に迷った点
```

### ポイント

- **before/after の具体例は必須** — 抽象的な指示は各ピストルズで解釈がブレる
- **担当外ファイルを明示** — 「触ってはいけない」の明示が着弾点の重複を防ぐ
- 共通ファイル（import 集約、index.ts 等）への変更は指示に含めず、統合フェーズで Lead 自身が行う

## 完了報告（ピストルズ → Lead）

各サブエージェントの最終報告に含めさせる:

```
## Result
status: done / failed / partial

## Changed
- src/api/users.ts: 3 箇所
- src/api/posts.ts: 5 箇所

## Notes
- users.ts:42 の呼び出しはレスポンス型が異なるため要確認
```

## 統合フェーズ（Lead の責務）

全ピストルズの報告後、Lead が必ず実行する:

1. 共通ファイルへの変更をまとめて適用
2. 全体で build / typecheck / lint / テスト
3. diff 全体の見直し（重複・矛盾・取り残し）
4. `Notes` に上がった要確認事項をユーザーへの報告に含める
