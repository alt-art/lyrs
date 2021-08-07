mod config;
mod song;

use clap::{App, Arg, AppSettings::ColoredHelp};
use ansi_term::Colour::Green;
use config::{Config, get_config, write_config};
use song::get_song_lyrics;

#[tokio::main]
async fn main() {
    let matches = App::new("lyrs")
        .about("Command line aplication for view lyrics")
        .author("Pedro H. M. <pedromendescraft@gmail.com>")
        .version("v0.1.0")
        .version_short("v")
        .setting(ColoredHelp)
        .args(&[
            Arg::with_name("login")
                .short("l")
                .long("login")
                .conflicts_with("SEARCH")
                .help("Login in with a genius account"),
            Arg::with_name("SEARCH")
                .multiple(true)
                .required(true)
                .index(1)
                .required_unless("login")
                .help("Search for a song like (i.e) Slipknot Duality")
        ])
        .get_matches();

    if matches.is_present("login") {
        println!("lyrs oppened a url in your broser to login\nLogin and paste the token here:");
        open::that_in_background(
            genius_rs::auth::auth_url("eq0nkVmDHjhIZ8NjUbg9TWPXqHEt0oRa4tCQZ7ez2qgoQKGclsAgW7aLyARy67FK","", "", "", "token")
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop();
        let config = Config {token: input};
        write_config(config);
    }

    if let Some(v) = matches.values_of("SEARCH") {
        let query = v.collect::<Vec<_>>().join(" ");
        let config = get_config();
        match config {
            Some(c) => get_song_lyrics(&query, c).await,
            None => {
                println!("You are not logged, use {} to login", Green.paint("--login"));
                std::process::exit(1);
            }
        }
    }
    std::process::exit(0);
}
