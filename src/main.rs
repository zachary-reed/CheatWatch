extern crate anyhow;
extern crate regex;
extern crate reqwest;
extern crate tokio;
extern crate serde;
extern crate csv;
extern crate clap;
extern crate serde_json;


use anyhow::Result;
use clap::Parser;
use std::{
    fs::{File,OpenOptions},
    io::Write,
    thread,
    time,
    io::{
        BufRead,BufReader
    },
};

mod util;
use util::*;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    username: Option<String>,
}


#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(username) = cli.username {
        let user  = User::new(&username).await?;
        let user = InputUser::from(user);
        let serialized = serde_json::to_string(&user)?;
        println!("{:?}", serialized);
    }
    else {
        let delay: u64 = 30000;

        let csv_write = OpenOptions::new().create(true).append(true).write(true).open("R6_Data.csv")?;
        // if output csv file already has content, don't write headers again
        let write_headers = csv_write.metadata()?.len() == 0;
        let mut wtr = csv::WriterBuilder::new().has_headers(write_headers).from_writer(csv_write);
        let f = File::open("Cheat.txt")?;
        let mut lines_iter = BufReader::new(f).lines();
        let mut legit_index = get_starting_index("legit_index.txt")?;
        let mut cheat_index = get_starting_index("cheat_index.txt")?;
        println!("Skipping to non-cheater {}", &legit_index);
        for _ in 0..legit_index {
            lines_iter.next();
        }
        for line in lines_iter {
            let line = line?;
            let mut user  = User::new(line).await?;
            user.cheater = false;
            wtr.serialize(&user)?;
            wtr.flush()?;
            println!("Wrote user to file:\t{:?}", &user);
            legit_index += 1;
            let mut index_f = OpenOptions::new().truncate(true).write(true).open("legit_index.txt")?;
            index_f.write_all(legit_index.to_string().as_bytes())?;
            println!("Wrote index {} to legit_index.txt", &legit_index);
            thread::sleep(time::Duration::from_millis(delay));
            
        }

        let f = File::open("Cheat.txt")?;
        let mut lines_iter = BufReader::new(f).lines();
        println!("Skipping to cheater {}", &cheat_index);
        for _ in 0..cheat_index {
            lines_iter.next();
        }
        for line in lines_iter {
            let line = line?;
            let mut user  = User::new(line).await?;
            user.cheater = true;
            println!("{:?}", &user);
            wtr.serialize(user)?;
            thread::sleep(time::Duration::from_millis(delay));
            cheat_index += 1;
            let mut index_f = OpenOptions::new().truncate(true).write(true).open("cheat_index.txt")?;
            index_f.write_all(cheat_index.to_string().as_bytes())?;
            println!("Wrote index {} to cheat_index.txt", &cheat_index);
            wtr.flush()?;
        }
        wtr.flush()?;
    }
    Ok(())
}
