use torrent_file::Torrent;
use url::Url;

use clap::{
    Arg,
    App
};
use std::path::Path;

fn main() {
    let matches = App::new("Bittorrent Parser")
        .version("1.0")
        .author("Edwin Amsler <edwinguy@gmail.com>")
        .about("Parses dem Bittorrent files!")
        .arg(Arg::with_name("torrent")
            .short("t")
            .long("torrent")
            .required(true)
            .value_name("FILE")
            .help("The file should we parse")
            .takes_value(true))
        .arg(Arg::with_name("web-seed")
            .short("w'")
            .long("web")
            .multiple(true)
            .help("One or more URLs to set as the GetRight web seeds")
            .takes_value(true))
        .get_matches();

    let filename = Path::new(matches.value_of("torrent").unwrap());
    let mut torrent = Torrent::read(filename)
        .expect("Unable to parse torrent file");

    if matches.is_present("web-seed") {
        let web_seeds = matches.values_of("web-seed")
            .unwrap()
            .map(|x| Url::parse(&x).expect("Invalid URL").into_string())
            .collect();

        torrent.url_list = Some(web_seeds);
    }

    Torrent::write(&torrent, &filename).unwrap();

    println!("Torrent: {:#?}", torrent);
}