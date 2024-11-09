pub struct AtUri(pub String);

impl std::fmt::Display for AtUri {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl AtUri {
  pub fn repo(&self) -> Option<String> {
    self.0.split("/").nth(2).map(|t| t.to_string())
  }

  pub fn collection(&self) -> Option<String> {
    self.0.split("/").nth(3).map(|t| t.to_string())
  }

  pub fn rkey(&self) -> Option<String> {
    self.0.split("/").nth(4).map(|t| t.to_string())
  }

  pub fn is_profile(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.actor.profile")
      .unwrap_or(false)
  }

  pub fn is_feed_generator(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.feed.generator")
      .unwrap_or(false)
  }

  pub fn is_like(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.feed.like")
      .unwrap_or(false)
  }

  pub fn is_post(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.feed.post")
      .unwrap_or(false)
  }

  pub fn is_postgate(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.feed.postgate")
      .unwrap_or(false)
  }

  pub fn is_repost(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.feed.repost")
      .unwrap_or(false)
  }

  pub fn is_threadgate(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.feed.threadgate")
      .unwrap_or(false)
  }

  pub fn is_block(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.graph.block")
      .unwrap_or(false)
  }

  pub fn is_follow(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.graph.follow")
      .unwrap_or(false)
  }

  pub fn is_list(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.graph.list")
      .unwrap_or(false)
  }

  pub fn is_listblock(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.graph.listblock")
      .unwrap_or(false)
  }

  pub fn is_listitem(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.graph.listitem")
      .unwrap_or(false)
  }

  pub fn is_starterpack(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.graph.starterpack")
      .unwrap_or(false)
  }

  pub fn is_labeler_service(&self) -> bool {
    self
      .collection()
      .map(|c| c == "app.bsky.labeler.service")
      .unwrap_or(false)
  }

  pub fn is_actor_declaration(&self) -> bool {
    self
      .collection()
      .map(|c| c == "chat.bsky.actor.declaration")
      .unwrap_or(false)
  }
}
