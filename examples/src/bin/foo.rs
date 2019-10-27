use crossterm::{event_stream, poll_event, RawScreen};

fn main() {
    let r = RawScreen::into_raw_mode().unwrap();

    let mut stream = event_stream();

    while true {
        poll_event();

        for event in stream.key_events() {
            println!("{:?}", event);
        }
    }
}
