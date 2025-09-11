use std::{env, sync::LazyLock};

pub mod metadata;
pub mod pom;
pub mod pom_parser;
pub mod project;
pub mod reporter;
pub mod settings;

pub const HOME: LazyLock<String> = LazyLock::new(|| env::var("HOME").unwrap());
pub const MAVEN_HOME: LazyLock<String> =
    LazyLock::new(|| env::var("MAVEN_HOME").unwrap_or("".to_string()));
pub const CUSTOM_SETTINGS_LOCATION: LazyLock<String> =
    LazyLock::new(|| env::var("SETTINGS_PATH").unwrap_or("".to_string()));
