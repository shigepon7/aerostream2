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

pub struct Firehose {
  pub handles: indexmap::IndexMap<String, tokio::task::JoinHandle<()>>,
  pub tx: tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, Record)>,
  pub rx_hd: tokio::task::JoinHandle<()>,
  pub receivers: std::sync::Arc<
    tokio::sync::RwLock<
      Vec<tokio::sync::mpsc::Sender<(ComAtprotoSyncSubscribeReposCommit, Record)>>,
    >,
  >,
}

impl Firehose {
  pub fn new(size: usize) -> Self {
    let (tx, rx) = tokio::sync::mpsc::channel(size);
    let receivers = std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new()));
    let rx_hd = tokio::spawn(receiver_thread(rx, receivers.clone()));
    Self {
      handles: indexmap::IndexMap::new(),
      tx,
      rx_hd,
      receivers,
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
}
