#[macro_use]
extern crate hyper;
extern crate toml;
extern crate rustc_serialize;
extern crate time;

use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, ContentType, ContentLength};
use rustc_serialize::json;
use rustc_serialize::{Encodable, Decodable};
use std::process;
use std::env;

header! { (XRedmineAPIKey, "X-Redmine-API-Key") => [String] }

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
struct IssueContents {
    project_id: String,
    tracker_id: String,
    subject: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
struct Issue {
    issue: IssueContents,
}

#[derive(Debug, RustcDecodable)]
struct Configuration {
    settings: Config,
}

#[derive(Debug, Clone, PartialEq, Eq, RustcDecodable)]
struct Config {
    apikey: String,
    redmine: String,
    project_id: String,
    tracker_id: String,
    title_suffix: String,
    description: String,
}

fn parse_toml() -> Config {
    let config_content = include_str!("settings.toml");
    println!("config:\n{}", config_content);
    let mut parser = toml::Parser::new(&config_content);
    let toml = match parser.parse() {
        Some(toml) => toml::Value::Table(toml),
        None => panic!("Couldn't parse toml"),
    };
    let mut decoder = toml::Decoder::new(toml);
    let config = match Configuration::decode(&mut decoder) {
        Ok(config) => config,
        Err(_) => panic!("Couldn't decode toml with Configuration struct"),
    };
    config.settings
}

fn get_date() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Must specify date.");
    }
    let date = match time::strptime(&*args[1], "%Y-%m-%d") {
        Ok(d) => time::strftime("%Y-%m-%d", &d).unwrap(),
        Err(_) => {
            panic!("Invalid time format. Use: %Y-%m-%d");
        }
    };
    date
}

fn build_issue(config: Config, date: String) -> String {
    let contents = IssueContents {
        project_id: config.project_id,
        tracker_id: config.tracker_id,
        subject: format!("{} {}", date, config.title_suffix),
        description: format!("{}", config.description),
    };
    let issue = Issue { issue: contents };
    let json = match json::encode(&issue) {
        Ok(json) => json,
        Err(_) => {
            println!("Couldn't convert as json");
            process::exit(3)
        }
    };
    json.to_owned()
}

fn main() {
    let config = parse_toml();
    let date = get_date();
    let json = build_issue(config.clone(), date);
    println!("{}", &*json);

    let client = Client::new();
    let mut headers = Headers::new();
    let redmine = config.redmine;
    headers.set(XRedmineAPIKey(config.apikey));
    headers.set(ContentType::json());
    headers.set(ContentLength(json.len() as u64));
    let response = client.post(&*redmine).headers(headers).body(&*json).send();
    let mut body = String::new();
    let _ = match response {
        Ok(mut result) => {
            println!("{}", result.status);
            result.read_to_string(&mut body)
        }
        Err(_) => {
            println!("Couldn't read as string");
            process::exit(-2)
        }
    };
    println!("{}", body);
}
