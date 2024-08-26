use quote::{quote::*, state::*};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::time;

const QUOTE_DATA: &str = include_str!("../quotes.jsonl");

#[tokio::main]
async fn main() {
    let mut state = State::new(
        QUOTE_DATA
            .lines()
            .cycle()
            .flat_map(|quote| serde_json::from_str::<QuoteItem>(quote)),
    );
    let mut interval = time::interval(time::Duration::from_secs(10));
    let socket = TcpListener::bind("127.0.0.1:9393").await.unwrap();
    loop {
        tokio::select! {
                stream = socket.accept() => {
                    let (mut socket, _) = stream.unwrap();
                    socket.write_all(state.get_quote().to_string().as_bytes()).await.unwrap();
                },
                _ = interval.tick() => {
                    state.next();
                }
        }
    }
}
