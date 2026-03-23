# Pipeline Patterns

Team Bucciarati のパイプラインパターン。シンプルに3+1構成。

## Ship（デフォルト）

日常の 80% がこれ。レビューしてシップ。

```
Moody Blues → Sticky Fingers
  品質検証  →  シッピング
```

### フロー

1. **Moody Blues**: CI + コードレビュー + lint/format 自動修正
   - BLOCKED → パイプライン停止
   - NEEDS WORK → ユーザーに修正を促して再実行
   - SHIP IT → 次へ
2. **Sticky Fingers**: コミット → プッシュ → PR作成 → CI確認 → マージ

### トリガー例
- 「レビューしてシップして」
- 「チェックしてからPR出して」
- `/dispatch`（引数なし → デフォルトでこれ）

## Full

フルパイプライン。調査 → レビュー → シップ → デプロイ。

```
(Purple Haze) → Moody Blues → Sticky Fingers → (Gold Experience)
  調査(任意)  →   品質検証  →   シッピング   →   デプロイ(任意)
```

### フロー

1. **Purple Haze** (任意): 実装の背景調査、関連コードの影響範囲を調査
2. **Moody Blues**: CI チェック + 多角的コードレビュー + lint/format 自動修正
3. **Sticky Fingers**: コミット → PR → マージ
4. **Gold Experience** (任意): ビルド → マイグレーション → デプロイ → ヘルスチェック

### トリガー例
- 「フルリリースして」
- 「レビューからデプロイまで全部やって」
- `/dispatch full`

## Deploy

マージ済みのコードをデプロイ。

```
Gold Experience
  デプロイ
```

### フロー

1. **Gold Experience**: ビルド → マイグレーション → デプロイ → ヘルスチェック

### トリガー例
- 「本番にデプロイして」
- `/dispatch deploy`

## Custom

上記に当てはまらない場合、スタンドを自由に組み合わせる。

### 例

| パターン | フロー | ユースケース |
|---------|--------|------------|
| テスト & シップ | Spice Girl → Moody Blues → Sticky Fingers | テスト強化してからシップ |
| シップ & デプロイ | Sticky Fingers → Gold Experience | レビュー済みをデプロイ |
| 並列スプリント | Sex Pistols | 複数タスク並列実行 |
| テスト + レビュー | Spice Girl → Moody Blues | テスト追加してレビューのみ |
| 調査 → テスト | Purple Haze → Spice Girl | 調査してからテスト設計 |

### トリガー例
- 「テスト書いてからシップして」→ Spice Girl → Moody Blues → Sticky Fingers
- 「この3つのIssueを並列で」→ Sex Pistols
- `/dispatch custom`

## 単体呼び出し

パイプラインを組まず、スタンドを直接呼ぶ:

| 呼び方 | スタンド |
|--------|---------|
| 「レビューして」 | Moody Blues 直接 |
| 「調べて」 | Purple Haze 直接 |
| 「テスト書いて」 | Spice Girl 直接 |
| 「デプロイして」 | Gold Experience 直接 |
| 「シップして」 | Sticky Fingers 直接 |
| 「並列でやって」 | Sex Pistols 直接 |

> 1スタンドで完結する場合はパイプラインを組む必要なし。

## パイプライン途中再開

パイプラインが途中で停止した場合（Moody Blues が BLOCKED、CI 失敗等）、修正後に途中から再開:

```
/dispatch resume
```

### 再開フロー

1. 前回の停止ポイントを確認（git log、PR 状態、CI 結果）
2. 停止原因が解消されているか検証
3. 停止したステップから再開（最初からやり直さない）

### 再開可能な停止パターン

| 停止原因 | 再開ポイント |
|---------|-------------|
| Moody Blues BLOCKED (CI fail) | 修正後、Moody Blues から再実行 |
| Sticky Fingers CI fail | 修正後、Sticky Fingers の CI 確認から |
| Gold Experience deploy fail | Gold Experience のデプロイから |
