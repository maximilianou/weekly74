use std::collections::HashMap;
use config::Config;
fn main() {
    let settings = Config::builder()
        // Add in `./settings.yaml`
        .add_source(config::File::with_name("settings"))
        .add_source(config::File::with_name(".env.yaml"))
        .add_source(config::File::with_name(".env.json"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();
    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

}