#![feature(try_trait)]

#[macro_use]
extern crate failure;
extern crate json;
extern crate reqwest;

use std::env;

const ENDPOINT: &'static str = "https://www.googleapis.com/youtube/v3/channels?id=UCZ_EWaQZCZuGGfnuqUoHujw&part=statistics&key=";

#[derive(Debug, Fail)]
enum RaspberryTubeError {
    #[fail(display = "{}", _0)]
    Request(#[cause] reqwest::Error),

    #[fail(display = "{}", _0)]
    ParseInt(#[cause] std::num::ParseIntError),

    #[fail(display = "{}", _0)]
    Json(#[cause] json::Error),

    #[fail(display = "{}", _0)]
    Env(#[cause] std::env::VarError),

    #[fail(display = "None error")]
    NoneError(std::option::NoneError),
}

impl From<reqwest::Error> for RaspberryTubeError {
    fn from(err: reqwest::Error) -> RaspberryTubeError {
        RaspberryTubeError::Request(err)
    }
}

impl From<json::Error> for RaspberryTubeError {
    fn from(err: json::Error) -> RaspberryTubeError {
        RaspberryTubeError::Json(err)
    }
}

impl From<std::env::VarError> for RaspberryTubeError {
    fn from(err: std::env::VarError) -> RaspberryTubeError {
        RaspberryTubeError::Env(err)
    }
}

impl From<std::num::ParseIntError> for RaspberryTubeError {
    fn from(err: std::num::ParseIntError) -> RaspberryTubeError {
        RaspberryTubeError::ParseInt(err)
    }
}

impl From<std::option::NoneError> for RaspberryTubeError {
    fn from(err: std::option::NoneError) -> RaspberryTubeError {
        RaspberryTubeError::NoneError(err)
    }
}

fn get_subscribers() -> Result<u64, RaspberryTubeError> {
    let api_key = env::var("API_KEY")?;
    let body = reqwest::get(&format!("{}{}", ENDPOINT, api_key))?.text()?;
    let parsed = json::parse(&body)?;

    // let views = &parsed["items"][0]["statistics"]["viewCount"];
    let subscribers = &parsed["items"][0]["statistics"]["subscriberCount"];
    Ok(subscribers.as_str()?.parse::<u64>()?)
}

fn main() -> Result<(), RaspberryTubeError> {
    let subscribers_bin = format!("{:b}", get_subscribers()?);
    println!("{}", subscribers_bin);
    Ok(())
}
