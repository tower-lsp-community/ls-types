# Changelog

## [Unreleased]

[Unreleased]: https://github.com/tower-lsp-community/ls-types/compare/v0.1.0...HEAD


## [0.1.0] - 2025-08-07

[0.1.0]: https://github.com/tower-lsp-community/ls-types/releases/tag/v0.1.0

### Added

- add `From` and `DerefMut` trait to `Uri` (#16)
- add `to_file_path` and `from_file_path` to `Uri` (#17)

### Changed

- moved items to their respective module

  You can still import all items via the prelude.

  ```rust
  use ls_types::prelude::*;
  ```

### Fixed

- fix typo in `WorkspaceClientCapabilities` (#3)
- move `annotationId` from `DeleteFileOptions` to `DeleteFile` (#4)
- fix typo in `ReferenceOptions` name (#8)
- add `TextDocumentRegistrationOptions` struct (#10)
- improve item docs for `FoldingRange.{startLine,endLine}` (#11)
- add `labelSupport` to `DocumentSymbolClientCapabilities` (#12)
- add `DocumentSymbolRegistrationOptions` struct (#13)

---

Check [`lsp-types`'s changelog](https://github.com/gluon-lang/lsp-types/blob/master/CHANGELOG.md) for older versions.
