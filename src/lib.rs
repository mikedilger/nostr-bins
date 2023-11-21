
use http::Uri;
use nostr_types::{Event, Filter, IdHex};

mod internal;
use internal::*;

pub fn url_to_host_and_uri(url: &str) -> (String, Uri) {
    let uri: http::Uri = url.parse::<http::Uri>().expect("Could not parse url");
    let authority = uri.authority().expect("Has no hostname").as_str();
    let host = authority
        .find('@')
        .map(|idx| authority.split_at(idx + 1).1)
        .unwrap_or_else(|| authority);
    if host.is_empty() {
        panic!("URL has empty hostname");
    }
    (host.to_owned(), uri)
}

pub fn fetch_by_filter(url: &str, filter: Filter) -> Vec<Event> {
    let (host,uri) = url_to_host_and_uri(url);
    let wire = filters_to_wire(vec![filter]);
    fetch(host, uri, wire)
}

pub fn fetch_by_id(url: &str, id: IdHex) -> Option<Event> {
    let mut filter = Filter::new();
    filter.add_id(&id);
    let events = fetch_by_filter(url, filter);
    if events.is_empty() {
        None
    } else {
        Some(events[0].clone())
    }
}

pub fn post_event(url: &str, event: Event) {
    let (host,uri) = url_to_host_and_uri(url);
    let wire = event_to_wire(event);
    post(host, uri, wire)
}

pub fn print_event(event: &Event) {
    println!("{}", serde_json::to_string(event).expect("Cannot serialize event to JSON"));
}
