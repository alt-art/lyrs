use std::env;
use genius_rs::Genius;
use pager::Pager;
use ansi_term::Colour;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let genius = Genius::new(dotenv::var("TOKEN").unwrap());
    let search = genius.search(&args[1]).await.unwrap();
    if search.len() > 0 {
        let lyric = genius.get_lyrics(&search[0].result.url).await.unwrap();
        let green = Colour::Green;
        let title = format!("{} - {}", search[0].result.primary_artist.name, search[0].result.title);
        Pager::with_default_pager("less -r").setup();
        println!("{}\n{}", green.bold().paint(title), search[0].result.url);

        for verse in lyric {
            if verse.contains("[") && verse.contains("]") {
                println!("\n{}\n", Colour::Blue.bold().paint(verse));
            } else {
                println!("{}", verse)
            }
        }

        let others_len =  if search.len() < 4 {search.len()} else {4};
        if others_len - 1 > 0 {
            let other_results = format!("\n{} other results:", others_len - 1);
            println!("{}", Colour::Yellow.bold().paint(other_results));
            for i in 1..others_len {
                let title = format!("{} - {}", search[i].result.primary_artist.name, search[i].result.title);
                println!("{}", green.bold().paint(title));
            }
        }
    } else {
        println!("No results found.");
        std::process::exit(1);
    }
    std::process::exit(0);
}
