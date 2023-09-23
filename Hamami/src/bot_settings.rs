use config::{Config, File};

pub fn get_bot_token() -> String {
    let settings = Config::builder()
        .add_source(File::with_name("settings.toml"))
        .build()
        .unwrap();

    settings.get_string("bot.token").unwrap()
}
