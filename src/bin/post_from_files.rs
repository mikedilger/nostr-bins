
use std::env;
use std::fs;
use std::io::Read;
use nostr_types::Event;

fn main() {
    let mut args = env::args();
    let _ = args.next(); // program name
    let relay_url = match args.next() {
        Some(u) => u,
        None => panic!("Usage: post_from_files <RelayURL> <Directory>"),
    };
    let directory = match args.next() {
        Some(d) => d,
        None => panic!("Usage: post_from_files <RelayURL> <Directory>"),
    };

    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(entry.path())
            .unwrap();
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap();
        let event: Event = serde_json::from_str(&contents).unwrap();
        event.verify(None).unwrap();

        nostr_bins::post_event(&relay_url, event);
    }
}
