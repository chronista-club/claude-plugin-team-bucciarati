# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.17.2] - 2026-05-02

### Fixed
- **Install error 修正**: plugin.json の `mcpServers` を string path (`"${CLAUDE_PLUGIN_ROOT}/.mcp.json"`) で指定していたため schema validation で `Invalid input` エラー。string path 形式は公式仕様で未サポート。フィールドを削除し、`.mcp.json` の auto-discovery に任せる形に修正
  - 公式仕様: https://code.claude.com/docs/en/mcp.md (`.mcp.json` at plugin root or inline object in `plugin.json`)
- **0.17.0 / 0.17.1 はインストール不可**。0.17.2 へ更新してください

## [0.17.1] - 2026-05-02 [BROKEN — install fails]

### Removed
- Redundant inner `.claude-plugin/marketplace.json` (single-plugin self-referential、 公式 spec 準拠で plugin.json のみに統一)

> ⚠️ このリリースは plugin.json schema 違反でインストール不可。0.17.2 で修正済み。

## [0.17.0] - 2026-05-02

### Changed
- Spec compliance: separated mcpServers to .mcp.json, added homepage, CHANGELOG, .gitignore
- Dropped legacy skills.txt
