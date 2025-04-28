use std::collections::HashMap;
use config::Config;
use chrono::{DateTime, Local};
fn main() {

    let curr_time: DateTime<Local> = Local::now();
    let curr_time_format = curr_time.format("%Y%m%d");
    println!("{}", curr_time_format);

    let settings = Config::builder()
        // Add in `./settings.yaml`
        .add_source(config::File::with_name("settings"))
        .add_source(config::File::with_name(".env.yaml"))
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