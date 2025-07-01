# Changelog

<!-- https://keepachangelog.com/en/1.0.0/ -->

## [0.4.0] - 2025-07-01

- Upgrade `tree-sitter` to 0.25
- Simplify CI configuration

## [0.3.0] - 2023-04-06

- Give a default implementation for `Editor::in_order_edits`

## [0.2.0] - 2023-04-06

- Redesign `Editor` interface for efficiency
- Fix a bug introduced in 0.1.2 that caused a panic

## [0.1.2] - 2023-04-01

- Make `collect_edits` non-recursive to avoid stack overflows

## [0.1.1] - 2023-04-01

- Configure Dependabot
- Make `Editor::contains_edit` non-recursive to avoid stack overflows
- Fix Clippy lints

## [0.1.0] - 2022-11-06

Initial release!

[0.1.0]: https://github.com/langston-barrett/tree-sitter-edit/releases/tag/v0.1.0
[0.1.1]: https://github.com/langston-barrett/tree-sitter-edit/releases/tag/v0.1.1
[0.1.2]: https://github.com/langston-barrett/tree-sitter-edit/releases/tag/v0.1.2
[0.2.0]: https://github.com/langston-barrett/tree-sitter-edit/releases/tag/v0.2.0
[0.3.0]: https://github.com/langston-barrett/tree-sitter-edit/releases/tag/v0.3.0
[0.4.0]: https://github.com/langston-barrett/tree-sitter-edit/releases/tag/v0.4.0
