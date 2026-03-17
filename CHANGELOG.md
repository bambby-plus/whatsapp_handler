# Changelog

All notable changes to this project will be documented here.

## [0.2.0] - 2026-03-17
### Added
- **WhatsApp Business Template Messages Support**
  - Text templates with body parameters
  - Media templates (image, video, document in header)
  - Interactive templates with buttons (URL and quick_reply)
  - Support for currency and date_time parameters
- Example test for OTP verification template

### Fixed
- Minor code cleanup and warning fixes

### Changed
- Updated API version reference to v22.0 in documentation

---

## [0.1.1] - 2025-05-10
### Added
- Additional error handling improvements

### Fixed
- Documentation updates

---

## [0.1.0] - 2025-05-05
### Added
- Initial release of `whatsapp_handler`.
- Support for parsing WhatsApp webhook messages.
- Handles text, image, video, audio, and document message types.
- Basic error handling with `thiserror`.
- Unit tests for core processing functions.

### Fixed
- N/A

### Changed
- N/A

---

## [Unreleased]
### Added
- Support for additional WhatsApp features.
- More comprehensive test coverage.

### Fixed
- Minor bug fixes.

### Changed
- Performance improvements.
