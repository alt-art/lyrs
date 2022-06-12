use crate::config::Config;
use ansi_term::Colour;
use genius_rs::Genius;

#[cfg(unix)]
use pager::Pager;

pub async fn get_song_lyrics(query: &str, config: Config) {
    let genius = Genius::new(config.token);
    let search = genius.search(query).await.unwrap();
    if search.is_empty() {
        println!("No results found.");
        std::process::exit(1);

    } else {
        let song = &search[0].result;
        let lyric = genius.get_lyrics(song.id).await.unwrap();
        let green = Colour::Green;
        let title = format!("{} - {}", song.primary_artist.name, song.title);
        #[cfg(unix)]
        Pager::with_default_pager("less -r").setup();

        println!("{}\n{}", green.bold().paint(title), song.url);

        for verse in lyric {
            if verse.contains('[') && verse.contains(']') {
                println!("\n{}\n", Colour::Blue.bold().paint(verse.trim()));
            } else {
                println!("{}", verse.trim());
            }
        }

        let others_len = if search.len() < 4 { search.len() } else { 4 };
        if others_len - 1 > 0 {
            let other_results = format!("\n{} other results:", others_len - 1);
            println!("{}", Colour::Yellow.bold().paint(other_results));
            for res in search.iter().take(others_len).skip(1) {
                let song = &res.result;
                let title = format!("{} - {}", song.primary_artist.name, song.title);
                println!("{}", green.bold().paint(title));
            }
        }
    }
}
