#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate toml;
#[macro_use]
extern crate serde_derive;

use reqwest::header::{ContentLength, ContentType, Headers};
use reqwest::Client;
use std::env;
use std::io::Read;
use std::process;
use toml::de::Error;

// Use headers.set_raw(...) for now.
// header! { (XRedmineAPIKey, "X-Redmine-API-Key") => [String] }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct IssueContents {
    project_id: String,
    tracker_id: String,
    subject: String,
    description: String,
    start_date: Option<String>,
    due_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Issue {
    issue: IssueContents,
}

#[derive(Debug, Deserialize)]
struct Configuration {
    settings: Config,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct Config {
    apikey: String,
    redmine: String,
    project_id: String,
    tracker_id: String,
    title_suffix: String,
    description: String,
    specify_deadline: bool,
}

fn parse_toml(config_content: &'static str) -> Config {
    println!("config:\n{}", config_content);
    let configuration: Result<Configuration, Error> = toml::from_str(config_content);
    let config = match configuration {
        Ok(config) => config,
        Err(_) => panic!("Couldn't decode toml with Configuration struct"),
    };
    config.settings
}

fn validate_argument(args: Vec<String>) {
    if args.len() < 2 {
        panic!("Must specify date.");
    }
}

fn get_date(args: Vec<String>) -> String {
    match time::strptime(&*args[1], "%Y-%m-%d") {
        Ok(d) => time::strftime("%Y-%m-%d", &d).unwrap(),
        Err(_) => {
            panic!("Invalid time format. Use: %Y-%m-%d");
        }
    }
}

fn get_deadline(config: &Config, date: &String) -> Option<String> {
    if config.specify_deadline {
        Some(format!("{}", date))
    } else {
        None
    }
}

fn build_issue(config: Config, date: String) -> String {
    let deadline = get_deadline(&config, &date);
    let contents = IssueContents {
        project_id: config.project_id,
        tracker_id: config.tracker_id,
        subject: format!("{} {}", date, config.title_suffix),
        description: config.description,
        start_date: deadline.clone(),
        due_date: deadline.clone(),
    };

    let issue = Issue { issue: contents };
    let json = match serde_json::to_string(&issue) {
        Ok(json) => json,
        Err(_) => {
            println!("Couldn't convert as json");
            process::exit(3)
        }
    };
    json.to_owned()
}

fn send_redmine(config: Config, json: String) {
    let client = Client::new();
    let mut headers = Headers::new();
    let redmine = config.redmine;
    headers.set_raw("X-Redmine-API-Key", &*config.apikey);
    headers.set(ContentType::json());
    headers.set(ContentLength(json.len() as u64));
    let response = client.post(&*redmine).headers(headers).body(json).send();
    let mut body = String::new();
    let _ = match response {
        Ok(mut result) => {
            println!("{}", result.status());
            result.read_to_string(&mut body)
        }
        Err(_) => {
            println!("Couldn't read as string");
            process::exit(-2)
        }
    };
    println!("{}", body);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    validate_argument(args.clone());
    let config_content = include_str!("settings.toml");
    let config = parse_toml(config_content);
    let date = get_date(args.clone());
    let json = build_issue(config.clone(), date);
    println!("{}", &*json);

    send_redmine(config, json);
}
