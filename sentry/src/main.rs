extern crate sentry;

extern crate toml;
extern crate serde;
extern crate url;
#[macro_use]
extern crate serde_derive;

//use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::thread;
//use std::env;
//use toml::Value as Toml;
use url::Url;
use sentry::{SentryCredential, Sentry};

#[derive(Debug, Deserialize)]
struct Config {
    sentry: Option<SentryConfig>
}

#[derive(Debug, Deserialize)]
struct SentryConfig {
    url: Option<String>
}

fn main() {
    let file_name = "profile.lock";
    let mut sentry_toml = String::new();
    if fs::metadata(file_name).is_ok() {
        File::open(file_name).and_then(|mut f| {
            f.read_to_string(&mut sentry_toml)
        }).unwrap();
        let sentry_config: Config = toml::from_str(&sentry_toml).unwrap();
        println!("{:#?}", sentry_config);

        let sentry_url = sentry_config.sentry.unwrap().url.unwrap();
        println!("{:#?}", sentry_url);

        let dest_url = Url::parse(&sentry_url).unwrap();
        println!("{:#?} {:#?}", dest_url.username(), dest_url.password().unwrap());

        //        dest_url.host

        //        let credential = SentryCredential { key: String::from(dest_url.username()), secret: String::from(dest_url.password().unwrap()), host: dest_url.host_str().unwrap().to_string() + ":" + &dest_url.port().unwrap().to_string(), project_id: String::from(dest_url.path()).trim_matches('/').to_string() };
        let credential: SentryCredential = sentry_url.parse().unwrap();
        println!("{:#?}", credential);
        let sentry = Sentry::new("Server Name".to_string(), "release".to_string(), "test_env".to_string(), credential);
        sentry.info("test.logger", "Test Message", None);
        println!("done");
        thread::sleep(std::time::Duration::new(5, 0));
    } else {
        println!("{} not found!!", file_name);
    }

    //    match sentry_url.parse() {
    //        Ok(toml) => {
    //            println!("Hello")
    //        }
    //        Err(error) => println!("failed to parse TOML: {}", error)
    //    }
}
