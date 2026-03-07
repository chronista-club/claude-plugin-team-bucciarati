---
description: "Aerosmith を起動してパイプラインをディスパッチする"
---

Aerosmith エージェントを起動してください。

ユーザーの現在の状況（git status, diff, log）を偵察し、最適なパイプラインを提案してください。

ユーザーが特定のパイプラインを指定していない場合は、以下の選択肢を提示してください:

1. **Full Release** — 調査 → レビュー → シップ → デプロイ
2. **Review & Ship** — レビュー → シップ
3. **Ship & Deploy** — シップ → デプロイ
4. **Test & Ship** — テスト → レビュー → シップ
5. **Research Only** — 調査のみ
6. **Deploy Only** — デプロイのみ
7. **Parallel Sprint** — 並列ワーカー

$ARGUMENTS
