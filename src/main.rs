#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::str_to_string
)]
#![allow(clippy::module_name_repetitions)]

mod config;
mod song;

use ansi_term::Colour::Green;
use anyhow::Result;
use clap::Parser;
use config::{get_config, write_config, Config};
use song::get_song_lyrics;

#[derive(Parser)]
#[clap(about, author, version)]
struct Opt {
    #[clap(
        short,
        long,
        help = "Login in with a genius account",
        conflicts_with = "search"
    )]
    login: bool,
    #[clap(
        help = "Search for a song like (e.g Slipknot Duality)",
        multiple_values = true,
        index = 1,
        conflicts_with = "login",
        required = false,
        default_value = "",
        required_unless_present = "login",
    )]
    search: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();
    if opt.login {
        println!("lyrs oppened a url in your browser to login\nLogin and paste the token here:");
        open::that(
            genius_rs::auth::auth_url(
                "eq0nkVmDHjhIZ8NjUbg9TWPXqHEt0oRa4tCQZ7ez2qgoQKGclsAgW7aLyARy67FK",
                "token",
                None,
                None,
                None,
            )
            .to_string(),
        )?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let x: &[_] = &['\n', '\r'];
        input = input.trim_end_matches(x).to_owned();
        let config = Config { token: input };
        write_config(&config);
    } else {
        let config = get_config();
        let query = opt.search.join(" ");
        match config {
            Some(c) => get_song_lyrics(&query, c).await,
            None => {
                println!(
                    "You are not logged, use {} to login",
                    Green.paint("--login")
                );
                std::process::exit(1);
            }
        }
    }
    std::process::exit(0);
}
