use crate::*;

pub async fn firehose_thread(
  hostname: String,
  tx: tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, Record)>,
) {
  let atproto = Atproto::new(None, Some(&hostname));
  let mut cursor = None;
  let mut counter: u64 = 0;
  loop {
    tracing::warn!(
      "FIREHOSE : {hostname} : try connect websocket from {}",
      cursor.unwrap_or_default()
    );
    let mut ws = match atproto.com_atproto_sync_subscribe_repos(cursor).await {
      Ok(ws) => ws,
      Err(e) => {
        tracing::warn!("FIREHOSE : {hostname} : connect websocket error : {e}");
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        continue;
      }
    };
    loop {
      let message = match futures_util::StreamExt::next(&mut ws).await {
        Some(Ok(o)) => o,
        Some(Err(e)) => {
          tracing::warn!("FIREHOSE : {hostname} : websocket receive error : {e}");
          tokio::time::sleep(std::time::Duration::from_secs(1)).await;
          break;
        }
        None => continue,
      };
      counter += 1;
      if counter % 1000 == 0 {
        tracing::info!("FIREHOSE : {hostname} : received {counter}");
      }
      let object = match Object::try_from(&message) {
        Ok(o) => o,
        Err(_) => continue,
      };
      let commit = match object.as_commit() {
        Some(c) => c,
        None => continue,
      };
      cursor = Some(commit.seq);
      for record in commit.to_records().await.into_iter() {
        if let Err(e) = tx.try_send((commit.clone(), record)) {
          tracing::warn!("FIREHOSE : {hostname} : send record error {e}");
          continue;
        }
      }
    }
  }
}

pub async fn receiver_thread(
  mut servers: tokio::sync::mpsc::Receiver<(ComAtprotoSyncSubscribeReposCommit, Record)>,
  receivers: std::sync::Arc<
    tokio::sync::RwLock<
      Vec<tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, Record)>>,
    >,
  >,
) {
  let mut counter: u64 = 0;
  loop {
    let payload = match servers.recv().await {
      Some(p) => p,
      None => continue,
    };
    counter += 1;
    if counter % 10000 == 0 {
      tracing::info!("RECEIVER : received {counter}");
    }
    for tx in receivers.read().await.iter() {
      if let Err(e) = tx.send(payload.clone()).await {
        tracing::warn!("RECEIVER : send record error {e}");
      }
    }
  }
}

pub async fn post_thread(
  mut receiver: tokio::sync::mpsc::Receiver<(ComAtprotoSyncSubscribeReposCommit, Record)>,
  post_receivers: std::sync::Arc<
    tokio::sync::RwLock<
      Vec<tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, AppBskyFeedPost)>>,
    >,
  >,
) {
  let mut counter: u64 = 0;
  loop {
    let (commit, record) = match receiver.recv().await {
      Some(p) => p,
      None => continue,
    };
    let post = match record.as_app_bsky_feed_post() {
      Some(p) => p,
      None => continue,
    };
    counter += 1;
    if counter % 10000 == 0 {
      tracing::info!("POST_RECEIVER : received {counter}");
    }
    for tx in post_receivers.read().await.iter() {
      if let Err(e) = tx.send((commit.clone(), post.clone())).await {
        tracing::warn!("POST_RECEIVER : send record error {e}");
      }
    }
  }
}

pub async fn japanese_thread(
  mut receiver: tokio::sync::mpsc::Receiver<(ComAtprotoSyncSubscribeReposCommit, AppBskyFeedPost)>,
  ja_receivers: std::sync::Arc<
    tokio::sync::RwLock<
      Vec<tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, AppBskyFeedPost)>>,
    >,
  >,
) {
  let mut counter: u64 = 0;
  loop {
    let (commit, post) = match receiver.recv().await {
      Some(p) => p,
      None => continue,
    };
    if post
      .langs
      .as_ref()
      .map(|l| l.contains(&String::from("ja")))
      .unwrap_or(false)
    {
      counter += 1;
      if counter % 1000 == 0 {
        tracing::info!("JA_RECEIVER : received {counter}");
      }
      for tx in ja_receivers.read().await.iter() {
        if let Err(e) = tx.send((commit.clone(), post.clone())).await {
          tracing::warn!("JA_RECEIVER : send record error {e}");
        }
      }
    }
  }
}

pub async fn token_thread(
  mut receiver: tokio::sync::mpsc::Receiver<(ComAtprotoSyncSubscribeReposCommit, AppBskyFeedPost)>,
  ja_receivers: std::sync::Arc<
    tokio::sync::RwLock<
      Vec<
        tokio::sync::mpsc::Sender<(
          ComAtprotoSyncSubscribeReposCommit,
          AppBskyFeedPost,
          Vec<Vec<String>>,
        )>,
      >,
    >,
  >,
) {
  let mut counter: u64 = 0;
  let dictionary = lindera::DictionaryConfig {
    kind: Some(lindera::DictionaryKind::IPADIC),
    path: None,
  };
  let config = lindera::TokenizerConfig {
    dictionary,
    user_dictionary: None,
    mode: lindera::Mode::Normal,
  };
  let tokenizer = lindera::Tokenizer::from_config(config).unwrap();
  loop {
    let (commit, post) = match receiver.recv().await {
      Some(p) => p,
      None => continue,
    };
    if let Ok(tokens) = tokenizer
      .tokenize(&post.text)
      .map(|mut tokens| {
        tokens
          .iter_mut()
          .filter_map(|t| t.get_details())
          .map(|t| t.iter().map(|t| t.to_string()).collect::<Vec<_>>())
          .collect::<Vec<_>>()
      })
      .as_ref()
    {
      counter += 1;
      if counter % 1000 == 0 {
        tracing::info!("TOKEN_RECEIVER : received {counter}");
      }
      for tx in ja_receivers.read().await.iter() {
        if let Err(e) = tx
          .send((commit.clone(), post.clone(), tokens.to_vec()))
          .await
        {
          tracing::warn!("TOKEN_RECEIVER : send record error {e}");
        }
      }
    }
  }
}

pub struct Firehose {
  pub handles: indexmap::IndexMap<String, tokio::task::JoinHandle<()>>,
  pub tx: tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, Record)>,
  pub rx_hd: tokio::task::JoinHandle<()>,
  pub post_rx_hd: tokio::task::JoinHandle<()>,
  pub ja_rx_hd: tokio::task::JoinHandle<()>,
  pub token_rx_hd: tokio::task::JoinHandle<()>,
  pub receivers: std::sync::Arc<
    tokio::sync::RwLock<
      Vec<tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, Record)>>,
    >,
  >,
  pub post_receivers: std::sync::Arc<
    tokio::sync::RwLock<
      Vec<tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, AppBskyFeedPost)>>,
    >,
  >,
  pub ja_receivers: std::sync::Arc<
    tokio::sync::RwLock<
      Vec<tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, AppBskyFeedPost)>>,
    >,
  >,
  pub token_receivers: std::sync::Arc<
    tokio::sync::RwLock<
      Vec<
        tokio::sync::mpsc::Sender<(
          ComAtprotoSyncSubscribeReposCommit,
          AppBskyFeedPost,
          Vec<Vec<String>>,
        )>,
      >,
    >,
  >,
}

impl Firehose {
  pub fn new(size: usize) -> Self {
    let token_receivers = std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new()));
    let (token_tx, token_rx) = tokio::sync::mpsc::channel(size);
    let token_rx_hd = tokio::spawn(token_thread(token_rx, token_receivers.clone()));

    let ja_receivers = std::sync::Arc::new(tokio::sync::RwLock::new(vec![token_tx]));
    let (ja_tx, ja_rx) = tokio::sync::mpsc::channel(size);
    let ja_rx_hd = tokio::spawn(japanese_thread(ja_rx, ja_receivers.clone()));

    let post_receivers = std::sync::Arc::new(tokio::sync::RwLock::new(vec![ja_tx]));
    let (post_tx, post_rx) = tokio::sync::mpsc::channel(size);
    let post_rx_hd = tokio::spawn(post_thread(post_rx, post_receivers.clone()));

    let (tx, rx) = tokio::sync::mpsc::channel(size);
    let receivers = std::sync::Arc::new(tokio::sync::RwLock::new(vec![post_tx]));
    let rx_hd = tokio::spawn(receiver_thread(rx, receivers.clone()));

    Self {
      handles: indexmap::IndexMap::new(),
      tx,
      rx_hd,
      post_rx_hd,
      ja_rx_hd,
      token_rx_hd,
      receivers,
      post_receivers,
      ja_receivers,
      token_receivers,
    }
  }

  pub fn add_server(&mut self, hostname: &str) {
    self.handles.insert(
      hostname.to_string(),
      tokio::spawn(firehose_thread(hostname.to_string(), self.tx.clone())),
    );
  }

  pub async fn add_receiver(
    &mut self,
    size: usize,
  ) -> tokio::sync::mpsc::Receiver<(ComAtprotoSyncSubscribeReposCommit, Record)> {
    let (sender, receiver) = tokio::sync::mpsc::channel(size);
    self.receivers.write().await.push(sender);
    receiver
  }

  pub async fn add_post_receiver(
    &mut self,
    size: usize,
  ) -> tokio::sync::mpsc::Receiver<(ComAtprotoSyncSubscribeReposCommit, AppBskyFeedPost)> {
    let (sender, receiver) = tokio::sync::mpsc::channel(size);
    self.post_receivers.write().await.push(sender);
    receiver
  }

  pub async fn add_ja_receiver(
    &mut self,
    size: usize,
  ) -> tokio::sync::mpsc::Receiver<(ComAtprotoSyncSubscribeReposCommit, AppBskyFeedPost)> {
    let (sender, receiver) = tokio::sync::mpsc::channel(size);
    self.ja_receivers.write().await.push(sender);
    receiver
  }

  pub async fn add_token_receiver(
    &mut self,
    size: usize,
  ) -> tokio::sync::mpsc::Receiver<(
    ComAtprotoSyncSubscribeReposCommit,
    AppBskyFeedPost,
    Vec<Vec<String>>,
  )> {
    let (sender, receiver) = tokio::sync::mpsc::channel(size);
    self.token_receivers.write().await.push(sender);
    receiver
  }
}
