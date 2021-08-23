mod config;
mod song;

use ansi_term::Colour::Green;
use clap::{App, AppSettings::ColoredHelp, Arg};
use config::{get_config, write_config, Config};
use song::get_song_lyrics;

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();
    let matches = App::new("lyrs")
        .about("Command line aplication to view lyrics")
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
                .help("Search for a song like (e.g Slipknot Duality)"),
        ])
        .get_matches();

    if matches.is_present("login") {
        println!("lyrs oppened a url in your browser to login\nLogin and paste the token here:");
        open::that_in_background(genius_rs::auth::auth_url(
            "eq0nkVmDHjhIZ8NjUbg9TWPXqHEt0oRa4tCQZ7ez2qgoQKGclsAgW7aLyARy67FK",
            "",
            "",
            "",
            "token",
        ));
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let x: &[_] = &['\n', '\r'];
        input = input.trim_end_matches(x).to_string();
        let config = Config { token: input };
        write_config(config);
    }

    if let Some(v) = matches.values_of("SEARCH") {
        let query = v.collect::<Vec<_>>().join(" ");
        let config = get_config();
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
