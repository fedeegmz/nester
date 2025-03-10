# Changelog

## [0.2.0] - 2025-03-10

### Added

- Create `.nester` directory for config files.
- Load configs from `.nester/config.toml`.
- Clone templates repository from `repository` config arg in `.nester/templates`.

### Changed

- Use Tera for templates.

## [0.1.1] - 2025-02-05

### Changed

- Move `Module.kt` to `Injection.kt`.
- Inject dependencies using Koin.

## [0.1.0] - 2025-02-04

### Initial Release

- First release of **Nester**.
- The `nester -g module -n example` command is available for creating a module into Ktor backend project. Including:
  - **Routing.kt** → basic route handler.
  - **Service.kt** → service scaffold.
  - **Module.kt** → init module.
