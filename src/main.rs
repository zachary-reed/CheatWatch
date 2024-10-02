extern crate anyhow;
extern crate regex;
extern crate reqwest;
extern crate tokio;
extern crate serde;
extern crate csv;

use anyhow::Result;
use regex::Regex;
use serde::{Serialize,Deserialize};
use std::fs::File;
use std::{
    thread,
    time,
    io::{
        BufRead,BufReader
    },
};

fn get_prof_url(name: impl AsRef<str>) -> String {
    let mut url = String::from("https://r6.tracker.network/r6siege/profile/ubi/");
    url += name.as_ref();
    url += "/overview";
    println!("Generated profile URL: {}", &url);
    url
}

fn gen_regex_decimal(category: impl AsRef<str>) -> Result<Regex> {
    let mut buf = String::from(category.as_ref());
    buf += r".{103}([0-9]+\.[0-9]+)";
    Ok(Regex::new(&buf)?)
}

fn gen_regex_integer(category: impl AsRef<str>) -> Result<Regex> {
    let mut buf = String::from(category.as_ref());
    buf += r".{103}([[0-9]+,?]+)";
    Ok(Regex::new(&buf)?)
}

fn get_headshot_percentage(resp: impl AsRef<str>) -> Result<f32> {
    let regex = gen_regex_decimal("HS%")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut hsp = caps.get(1).unwrap().as_str().parse::<f32>()?;
    hsp /= 100.0;
    Ok(hsp)
}

fn get_winrate_percentage(resp: impl AsRef<str>) -> Result<f32> {
    let regex = gen_regex_decimal("Win Rate")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    println!("{:?}",caps);
    let mut wrp = caps.get(1).unwrap().as_str().parse::<f32>()?;
    wrp /= 100.0;
    Ok(wrp)
}

fn get_wins_total(resp: impl AsRef<str>) -> Result<usize> {
    let regex = gen_regex_integer("Wins")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut wins = String::from(caps.get(1).unwrap().as_str());
    wins.retain(|c| c != ',');
    let wins = wins.parse::<usize>()?;
    Ok(wins)
}

fn get_losses_total(resp: impl AsRef<str>) -> Result<usize> {
    let regex = gen_regex_integer("Losses")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut losses = String::from(caps.get(1).unwrap().as_str());
    losses.retain(|c| c != ',');
    let losses = losses.parse::<usize>()?;
    Ok(losses)
}

fn get_matches_total(resp: impl AsRef<str>) -> Result<usize> {
    let regex = gen_regex_integer("Matches")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut matches = String::from(caps.get(1).unwrap().as_str());
    matches.retain(|c| c != ',');
    let matches = matches.parse::<usize>()?;
    Ok(matches)
}

fn get_kd(resp: impl AsRef<str>) -> Result<f32> {
    let regex = gen_regex_decimal("KD")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let kd = caps.get(1).unwrap().as_str().parse::<f32>()?;
    Ok(kd)
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub hsp: f32,
    pub wrp: f32,
    pub wins: usize,
    pub losses: usize,
    pub matches: usize,
    pub kd: f32,
}

impl User {
    async fn new(username: impl AsRef<str>) -> Result<User> {
        let mut user = User::default();
        user.username = String::from(username.as_ref());
        let url = get_prof_url(&username);
        let resp = reqwest::get(&url).await?.text().await?;
        user.hsp = get_headshot_percentage(&resp)?;
        user.wrp = get_winrate_percentage(&resp)?;
        user.wins = get_wins_total(&resp)?;
        user.losses = get_losses_total(&resp)?;
        user.matches = get_matches_total(&resp)?;
        user.kd = get_kd(&resp)?;
        Ok(user)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut wtr = csv::Writer::from_path("R6_Data.csv")?;
    //wtr.write_record(&["Usernames", "Headshot Percentage", "Win Rate Percentage", "Total Wins", "Total Losses", "Total Matches", "KD"])?;
    let f = File::open("names.txt")?;
    for line in BufReader::new(f).lines() {
        let line = line?;
        let user  = User::new(line).await?;
        println!("{:?}", &user);
        wtr.serialize(user)?;
        thread::sleep(time::Duration::from_millis(100));
    
    }
    wtr.flush()?;
    Ok(())
}
