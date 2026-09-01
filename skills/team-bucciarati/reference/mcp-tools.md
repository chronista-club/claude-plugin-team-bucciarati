# MCP Tools Reference

Team Bucciarati の各スタンドが活用できる MCP ツール。
全てオプショナル — ツールが利用不可でもスタンドは動作する。

## gitnexus — コードベースナレッジグラフ

Tree-sitter でコードを解析し、依存関係・呼び出しチェーン・シンボル関係をグラフ化。
事前に `gitnexus analyze` でリポジトリをインデックスする必要がある。

### 主なツール
| ツール | 用途 |
|--------|------|
| `query` | 自然言語/キーワードで実行フローを検索（BM25 + セマンティック） |
| `context` | シンボルの360度ビュー（呼び出し元/先、参照、プロセス参加） |
| `impact` | 変更の blast radius 分析（depth 別リスク分類） |
| `detect_changes` | git diff から影響を受ける実行フローを特定 |
| `cypher` | Cypher クエリでナレッジグラフを直接検索 |

### スタンド別活用
- **Purple Haze**: `query` で全体像把握 → `context` でシンボル深堀り → `impact` で影響範囲特定
- **Moody Blues**: `detect_changes` で diff の影響フロー検出 → `impact` で blast radius 確認
- **Spice Girl**: `context` でテスト対象の依存関係把握
- **Sticky Fingers**: `context` / `query` で成果物の claim をコード実体と突き合わせ

## sem — エンティティレベルのコードインテリジェンス

関数・クラス単位の実体と、ファイル横断の呼び出し / import グラフ。
構造的な質問（「何が呼んでいるか」「変えたら何が壊れるか」）は grep より先に sem。

### 主なツール
| ツール | 用途 |
|--------|------|
| `sem_context` | 関数/クラスの全ソース + 呼び出し元/先を一括取得 |
| `sem_impact` | 変更の影響範囲（transitive callers 含む） |
| `sem_entities` | エンティティ検索（本文の部分一致 / パス指定） |
| `sem_diff` | エンティティ単位の変更レビュー |
| `sem_blame` / `sem_log` | 誰がいつ変えたか / どう進化してきたか |

### スタンド別活用
- **Purple Haze**: `sem_context` で構造把握、`sem_impact` で影響範囲、`sem_log` で設計の来歴
- **Moody Blues**: `sem_diff` でエンティティ単位レビュー、`sem_impact` でリグレッションリスク評価
- **Spice Girl**: `sem_context` でテスト対象の API surface を正確に把握
- **Sticky Fingers**: `sem_blame` / `sem_log` で「コードの来歴に関する claim」を実測検証

## context7 — ライブラリドキュメント

ライブラリの最新ドキュメントとコード例を取得。

### 主なツール
| ツール | 用途 |
|--------|------|
| `resolve-library-id` | ライブラリ ID の解決 |
| `query-docs` | ドキュメント・コード例の取得 |

### スタンド別活用
- **Purple Haze**: 技術調査でライブラリの API 仕様・ベストプラクティスを確認
- **Spice Girl**: テストフレームワークの API・パターンを確認
