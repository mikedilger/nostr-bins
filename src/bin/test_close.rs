
use std::env;

fn main() {
    let mut args = env::args();
    let _ = args.next(); // program name
    let relay_url = match args.next() {
        Some(u) => u,
        None => panic!("Usage: test_close <RelayURL> <SubId>"),
    };
    let subid = match args.next() {
        Some(subid) => subid,
        None => panic!("Usage: test_close <RelayURL> <SubId>"),
    };

    nostr_bins::send_raw_message(&relay_url, format!(r#"["CLOSE","{}"]"#, subid));

    std::thread::sleep(std::time::Duration::new(1,0));

    nostr_bins::send_raw_message(&relay_url, format!(r#"["CLOSE","{}"]"#, subid));
}
