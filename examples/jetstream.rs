#[tokio::main]
async fn main() {
  let jetstream = aerostream2::Jetstream::default();
  let mut rx = jetstream.add_token_receiver().await;
  jetstream.connect().await.unwrap();
  loop {
    match rx.recv().await {
      Some((event, token)) => {
        println!("{event:?}\n{token:?}");
      }
      None => {
        break;
      }
    }
  }
}
