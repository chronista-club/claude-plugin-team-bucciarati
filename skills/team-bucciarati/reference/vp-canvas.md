# VP Canvas 連携 — Aerosmith Radar

Vantage Point (VP) が利用可能な場合、Aerosmith はパイプラインの進行を
VP Canvas に「作戦盤（Radar）」として描く。ナランチャのレーダーの実体化。

**全てオプショナル** — VP が利用不可（ツールなし / `SP 未接続` エラー）なら
Radar をスキップしてパイプラインを続行する。VP のためにパイプラインを止めない。

## 責務の原則

**pane を描くのは Aerosmith（または /dispatch を実行する Lead）だけ。**
各スタンドは Canvas を触らない — スタンドの報告（Report / StandContext）を
Lead が受け取り、Radar に反映する。スタンド定義は VP を知らないまま保たれる。

## 使用ツール

| ツール | 用途 |
|--------|------|
| `mcp__vantage-point__show` | Radar 盤面の表示・更新（content_type: markdown、title: "Aerosmith Radar"） |
| `mcp__vantage-point__capture_canvas` | mission 完了時の盤面キャプチャ（任意、作戦記録用） |

> VP Canvas は stack model — 同じ title で再 `show` すると盤面の更新として扱う。

## 更新タイミング

1. **パイプライン決定時** — 初期盤面（全スタンド待機状態）を表示
2. **各スタンド完了時** — 該当行の Status / Summary を更新し、最新 StandContext を反映
3. **BLOCKED / エラー時** — 盤面に停止理由を明示（ユーザーが Canvas だけ見ても状況がわかる状態に）
4. **mission 完了時** — 最終盤面。必要なら `capture_canvas` で記録

## Radar 盤面テンプレート

```markdown
# 🛰 Aerosmith Radar

**Mission: Forge** — VP-12 セッションタイムアウト実装
Pipeline: 2/4 進行中

| Stand | Phase | Status | Summary |
|-------|-------|--------|---------|
| Purple Haze | 調査 | – SKIPPED | 要件明確のため省略 |
| Gold Experience | 実装 | ✅ ALIVE | +230 −12 (6 files) · 全 green |
| Spice Girl | テスト強化 | ▶ 実行中 | — |
| Moody Blues | 品質検証 | ⬚ 待機 | — |

**StandContext（最新）** ← Gold Experience
tests: PASS · checks: PASS
notes: timeout 値は config 経由。AuthService パターン踏襲

---
**— COMMIT LINE —** この先はユーザーの領分
```

### Status 語彙

| 表示 | 意味 |
|------|------|
| `⬚ 待機` | 未着手 |
| `▶ 実行中` | 現在このスタンドが作業中 |
| `✅ <verdict>` | 完了（ALIVE / COMMIT READY / Done 等、スタンドの終端語彙） |
| `🛑 BLOCKED` | 停止 — Summary に理由を書く |
| `– SKIPPED` | 任意ステップの省略 |

## 検出と degradation

1. `mcp__vantage-point__show` ツールが利用可能かで判定（なければ Radar なし）
2. `show` が `SP 未接続` 等のエラーを返したら、**以降の Radar 更新を全てスキップ**
   （毎回リトライしない — パイプラインの速度を優先）
3. Radar の有無はパイプラインの成否・報告内容に影響しない
