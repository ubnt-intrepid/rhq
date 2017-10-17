use std::path::PathBuf;

error_chain! {
    errors {
        ConfigLoad(path: PathBuf) {
            description("Configuration file is not found")
            display("Failed to read configuration file `{}`", path.display())
        }
    }

    foreign_links {
        Io(::std::io::Error);
        TomlSer(::toml::ser::Error);
        TomlDe(::toml::de::Error);
        Json(::serde_json::Error);
        ShellExpand(::shellexpand::LookupError<::std::env::VarError>);
        UrlParse(::url::ParseError);
    }
}
