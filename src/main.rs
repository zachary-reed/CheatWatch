extern crate anyhow;
use anyhow::Result;
extern crate regex;
extern crate reqwest;
extern crate tokio;
use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_prof_url(name: impl AsRef<str>) -> String {
    let mut url = String::from("https://r6.tracker.network/r6siege/profile/ubi/");
    url += name.as_ref();
    url += "/overview";
    println!("Generated profile URL: {}", &url);
    url
}

fn gen_regex_percentage(category: impl AsRef<str>) -> Result<Regex> {
    let mut buf = String::from(category.as_ref());
    buf += r".{103}([0-9]{2}\.[0-9])";
    Ok(Regex::new(&buf)?)
}

fn get_headshot_percentage(resp: impl AsRef<str>) -> Result<f32> {
    let regex = gen_regex_percentage("HS%")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut hsp = caps.get(1).unwrap().as_str().parse::<f32>()?;
    hsp /= 100.0;
    Ok(hsp)
}

fn get_winrate_percentage(resp: impl AsRef<str>) -> Result<f32> {
    let regex = gen_regex_percentage("Win Rate")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut hsp = caps.get(1).unwrap().as_str().parse::<f32>()?;
    hsp /= 100.0;
    Ok(hsp)
}

#[tokio::main]
async fn main() -> Result<()> {
    let f = File::open("names.txt")?;
    for line in BufReader::new(f).lines() {
        let url = get_prof_url(line?);
        let resp = reqwest::get(&url).await?.text().await?;
        println!("Headshot percentage: {}", get_headshot_percentage(&resp)?);
        println!("Win rate: {}", get_winrate_percentage(&resp)?);
    }

    Ok(())
}
