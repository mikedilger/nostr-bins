
use base64::Engine;
use http::Uri;
use nostr_types::{ClientMessage, Event, Filter, RelayMessage, SubscriptionId};
use tungstenite::protocol::Message;

pub(crate) fn filters_to_wire(filters: Vec<Filter>) -> String {
    let message = ClientMessage::Req(SubscriptionId("111".to_owned()), filters);
    serde_json::to_string(&message).expect("Could not serialize message")
}

pub(crate) fn event_to_wire(event: Event) -> String {
    let message = ClientMessage::Event(Box::new(event));
    serde_json::to_string(&message).expect("Could not serialize message")
}

pub(crate) fn fetch(host: String, uri: Uri, wire: String) -> Vec<Event> {

    let mut events: Vec<Event> = Vec::new();

    let key: [u8; 16] = rand::random();
    let request = http::request::Request::builder()
        .method("GET")
        .header("Host", host)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header(
            "Sec-WebSocket-Key",
            base64::engine::general_purpose::STANDARD.encode(key),
        )
        .uri(uri)
        .body(())
        .expect("Could not build request");

    let (mut websocket, _response) =
        tungstenite::connect(request).expect("Could not connect to relay");

    websocket
        .write_message(Message::Text(wire))
        .expect("Could not send message to relay");

    loop {
        let message = match websocket.read_message() {
            Ok(m) => m,
            Err(e) => {
                println!("Problem reading from websocket: {}", e);
                return events;
            }
        };

        match message {
            Message::Text(s) => {
                println!("RAW MESSAGE: {}", s);
                let relay_message: RelayMessage =
                    serde_json::from_str(&s).expect(&s);
                match relay_message {
                    RelayMessage::Event(_, e) => events.push(*e),
                    RelayMessage::Closed(_, msg) => println!("CLOSED: {}", msg),
                    RelayMessage::Notice(s) => println!("NOTICE: {}", s),
                    RelayMessage::Eose(_) => {
                        let message = ClientMessage::Close(SubscriptionId("111".to_owned()));
                        let wire = match serde_json::to_string(&message) {
                            Ok(w) => w,
                            Err(e) => {
                                println!("Could not serialize message: {}", e);
                                return events;
                            }
                        };
                        if let Err(e) = websocket.write_message(Message::Text(wire)) {
                            println!("Could not write close subscription message: {}", e);
                            return events;
                        }
                        if let Err(e) = websocket.write_message(Message::Close(None)) {
                            println!("Could not write websocket close message: {}", e);
                            return events;
                        }
                    }
                    RelayMessage::Ok(_id, ok, reason) => {
                        println!("OK: ok={} reason={}", ok, reason)
                    }
                    RelayMessage::Auth(challenge) => {
                        // FIXME
                        println!("AUTH: {}", challenge)
                    }
                }
            }
            Message::Binary(_) => println!("IGNORING BINARY MESSAGE"),
            Message::Ping(vec) => if let Err(e) = websocket.write_message(Message::Pong(vec)) {
                println!("Unable to pong: {}", e);
            }
            Message::Pong(_) => println!("IGNORING PONG"),
            Message::Close(_) => {
                println!("Closing");
                break;
            }
            Message::Frame(_) => println!("UNEXPECTED RAW WEBSOCKET FRAME"),
        }
    }

    events
}

pub(crate) fn post(host: String, uri: Uri, wire: String) {
    let key: [u8; 16] = rand::random();
    let request = http::request::Request::builder()
        .method("GET")
        .header("Host", host)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header(
            "Sec-WebSocket-Key",
            base64::engine::general_purpose::STANDARD.encode(key),
        )
        .uri(uri)
        .body(())
        .expect("Could not build request");

    println!("Request built");

    let (mut websocket, _response) =
        tungstenite::connect(request).expect("Could not connect to relay");

    println!("Connected to relay");

    websocket
        .write_message(Message::Text(wire))
        .expect("Could not send message to relay");

    // Get and print one response message

    println!("Wrote message");

    let message = match websocket.read_message() {
        Ok(m) => m,
        Err(e) => {
            println!("Problem reading from websocket: {}", e);
            return;
        }
    };

    println!("Got response");

    match message {
        Message::Text(s) => {
            let relay_message: RelayMessage =
                serde_json::from_str(&s).expect(&s);
            match relay_message {
                RelayMessage::Event(_, e) => println!("EVENT: {}", serde_json::to_string(&e).unwrap()),
                RelayMessage::Closed(_, msg) => println!("CLOSED: {}", msg),
                RelayMessage::Notice(s) => println!("NOTICE: {}", s),
                RelayMessage::Eose(_) => println!("EOSE"),
                RelayMessage::Ok(_id, ok, reason) => println!("OK: ok={} reason={}", ok, reason),
                RelayMessage::Auth(challenge) => println!("AUTH: {}", challenge),
            }
        }
        Message::Binary(_) => println!("IGNORING BINARY MESSAGE"),
        Message::Ping(vec) => if let Err(e) = websocket.write_message(Message::Pong(vec)) {
            println!("Unable to pong: {}", e);
        }
        Message::Pong(_) => println!("IGNORING PONG"),
        Message::Close(_) => {
            println!("Closing");
            return;
        }
        Message::Frame(_) => println!("UNEXPECTED RAW WEBSOCKET FRAME"),
    }
}

pub(crate) fn send_message(host: String, uri: Uri, wire: String) {
    let key: [u8; 16] = rand::random();
    let request = http::request::Request::builder()
        .method("GET")
        .header("Host", host)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header(
            "Sec-WebSocket-Key",
            base64::engine::general_purpose::STANDARD.encode(key),
        )
        .uri(uri)
        .body(())
        .expect("Could not build request");

    let (mut websocket, _response) =
        tungstenite::connect(request).expect("Could not connect to relay");

    websocket
        .write_message(Message::Text(wire))
        .expect("Could not send message to relay");

    loop {
        let message = match websocket.read_message() {
            Ok(m) => m,
            Err(e) => {
                println!("Problem reading from websocket: {}", e);
                break;
            }
        };

        match message {
            Message::Text(s) => {
                println!("RAW MESSAGE: {}", s);
                std::thread::sleep(std::time::Duration::new(1,0));
                websocket.write_message(Message::Text(r#"["CLOSE","2"]"#.to_string()));
            }
            Message::Binary(_) => println!("IGNORING BINARY MESSAGE"),
            Message::Ping(vec) => if let Err(e) = websocket.write_message(Message::Pong(vec)) {
                println!("Unable to pong: {}", e);
            }
            Message::Pong(_) => println!("IGNORING PONG"),
            Message::Close(_) => {
                println!("Closing");
                break;
            }
            Message::Frame(_) => println!("UNEXPECTED RAW WEBSOCKET FRAME"),
        }
    }

}
