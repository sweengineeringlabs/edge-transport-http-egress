//! `ApplicationConfigBuilder` — maps to `config/application.toml`.

#[expect(
    dead_code,
    reason = "SEA api/ type alias anchor — intentionally unused; satisfies \
              config_file_to_builder_naming for config/application.toml"
)]
/// Config type corresponding to `config/application.toml`'s `[cassette]`
/// section — the only section this file declares.
pub type ApplicationConfigBuilder = crate::api::CassetteConfig;
