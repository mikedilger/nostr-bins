
use std::env;
use nostr_types::IdHex;

fn main() {
    let mut args = env::args();
    let _ = args.next(); // program name
    let relay_url = match args.next() {
        Some(u) => u,
        None => panic!("Usage: fetch_by_id <RelayURL> <EventID>"),
    };
    let id = match args.next() {
        Some(id) => id,
        None => panic!("Usage: fetch_by_id <RelayURL> <EventID>"),
    };

    let idhex = IdHex::try_from_str(&id).unwrap();

    if let Some(event) = nostr_bins::fetch_by_id(&relay_url, idhex) {
        nostr_bins::print_event(&event);
    } else {
        println!("Not found");
    }
}
