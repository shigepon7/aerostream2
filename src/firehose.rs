use crate::*;

pub async fn firehose_thread(
  hostname: String,
  tx: tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, Record)>,
) {
  let atproto = Atproto::new(None, Some(&hostname));
  let mut cursor = None;
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
  loop {
    let payload = match servers.recv().await {
      Some(p) => p,
      None => continue,
    };
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
  loop {
    let (commit, record) = match receiver.recv().await {
      Some(p) => p,
      None => continue,
    };
    let post = match record.as_app_bsky_feed_post() {
      Some(p) => p,
      None => continue,
    };
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
      for tx in ja_receivers.read().await.iter() {
        if let Err(e) = tx.send((commit.clone(), post.clone())).await {
          tracing::warn!("JA_RECEIVER : send record error {e}");
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
}

impl Firehose {
  pub fn new(size: usize) -> Self {
    let ja_receivers = std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new()));
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
      receivers,
      post_receivers,
      ja_receivers,
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
}
