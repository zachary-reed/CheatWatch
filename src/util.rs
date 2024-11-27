use serde::{Serialize,Deserialize};
use regex::Regex;
use anyhow::Result;
use std::{
    fs::OpenOptions,
    io::Read,
};

pub fn get_prof_url(name: impl AsRef<str>) -> String {
    let mut url = String::from("https://r6.tracker.network/r6siege/profile/ubi/");
    url += name.as_ref();
    url += "/overview";
    //println!("Generated profile URL: {}", &url);
    url
}

pub fn gen_regex_decimal(category: impl AsRef<str>) -> Result<Regex> {
    let mut buf = String::from(category.as_ref());
    buf += r".{103}([0-9]+\.[0-9]+)";
    Ok(Regex::new(&buf)?)
}

pub fn gen_regex_integer(category: impl AsRef<str>) -> Result<Regex> {
    let mut buf = String::from(category.as_ref());
    buf += r".{103}([[0-9]+,?]+)";
    Ok(Regex::new(&buf)?)
}

pub fn get_headshot_percentage(resp: impl AsRef<str>) -> Result<f32> {
    let regex = gen_regex_decimal("HS%")?;
    let caps = regex.captures(resp.as_ref());
    //println!("{:?}", &caps);
    let caps = caps.unwrap();
    let mut hsp = caps.get(1).unwrap().as_str().parse::<f32>()?;
    hsp /= 100.0;
    Ok(hsp)
}

pub fn get_winrate_percentage(resp: impl AsRef<str>) -> Result<f32> {
    let regex = gen_regex_decimal("Win Rate")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    //println!("{:?}",caps);
    let mut wrp = caps.get(1).unwrap().as_str().parse::<f32>()?;
    wrp /= 100.0;
    Ok(wrp)
}

pub fn get_wins_total(resp: impl AsRef<str>) -> Result<usize> {
    let regex = gen_regex_integer("Wins")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut wins = String::from(caps.get(1).unwrap().as_str());
    wins.retain(|c| c != ',');
    let wins = wins.parse::<usize>()?;
    Ok(wins)
}

pub fn get_losses_total(resp: impl AsRef<str>) -> Result<usize> {
    let regex = gen_regex_integer("Losses")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut losses = String::from(caps.get(1).unwrap().as_str());
    losses.retain(|c| c != ',');
    let losses = losses.parse::<usize>()?;
    Ok(losses)
}

pub fn get_matches_total(resp: impl AsRef<str>) -> Result<usize> {
    let regex = gen_regex_integer("Matches")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut matches = String::from(caps.get(1).unwrap().as_str());
    matches.retain(|c| c != ',');
    let matches = matches.parse::<usize>()?;
    Ok(matches)
}

pub fn get_kd(resp: impl AsRef<str>) -> Result<f32> {
    let regex = gen_regex_decimal("KD")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let kd = caps.get(1).unwrap().as_str().parse::<f32>()?;
    Ok(kd)
}

pub fn get_kills(resp: impl AsRef<str>) -> Result<usize> {
    let regex = gen_regex_integer("Kills")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut kills = String::from(caps.get(1).unwrap().as_str());
    kills.retain(|c| c != ',');
    let kills = kills.parse::<usize>()?;
    Ok(kills)
}

pub fn get_deaths(resp: impl AsRef<str>) -> Result<usize> {
    let regex = gen_regex_integer("Deaths")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let mut deaths = String::from(caps.get(1).unwrap().as_str());
    deaths.retain(|c| c != ',');
    let deaths = deaths.parse::<usize>()?;
    Ok(deaths)
}

pub fn get_kpm(resp: impl AsRef<str>) -> Result<f32> {
    let regex = gen_regex_decimal("Avg Kills")?;
    let caps = regex.captures(resp.as_ref()).unwrap();
    let kpm = caps.get(1).unwrap().as_str().parse::<f32>()?;
    Ok(kpm)
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
    pub kills: usize,
    pub deaths: usize,
    pub kpm: f32,
    pub cheater: bool,
}

impl User {
    pub async fn new(username: impl AsRef<str>) -> Result<User> {
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
        user.kills = get_kills(&resp)?;
        user.deaths = get_deaths(&resp)?;
        user.kpm = get_kpm(&resp)?;
        Ok(user)
    }
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub username: String,
    pub hsp: f32,
    pub wrp: f32,
    pub wins: usize,
    pub losses: usize,
    pub matches: usize,
    pub kd: f32,
    pub kills: usize,
    pub deaths: usize,
    pub kpm: f32,    
}
impl From<User> for InputUser {
    fn from(u: User) -> InputUser {
        InputUser {
            username: u.username,
            hsp: u.hsp,
            wrp: u.wrp,
            wins: u.wins,
            losses: u.losses,
            matches: u.matches,
            kd: u.kd,
            kills: u.kills,
            deaths: u.deaths,
            kpm: u.kpm
        }
    }
}


/// Parses a single usize from the file provided, or returns 0 if file not found. 
pub fn get_starting_index(file: impl AsRef<str>) -> Result<usize> {
    if let Ok(mut index_f) = OpenOptions::new().create(true).read(true).write(true).open(file.as_ref()) {
        let mut index = String::new();
        index_f.read_to_string(&mut index)?;
        let index = index.parse::<usize>().unwrap_or(0);
        Ok(index)
    }
    else {
        Ok(0)
    }
}
