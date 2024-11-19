use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Object {
  Commit(ComAtprotoSyncSubscribeReposCommit),
  Identity(ComAtprotoSyncSubscribeReposIdentity),
  Account(ComAtprotoSyncSubscribeReposAccount),
  Handle(ComAtprotoSyncSubscribeReposHandle),
  Migrate(ComAtprotoSyncSubscribeReposMigrate),
  Tombstone(ComAtprotoSyncSubscribeReposTombstone),
  Info(ComAtprotoSyncSubscribeReposInfo),
}

impl From<ComAtprotoSyncSubscribeReposCommit> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposCommit) -> Self {
    Self::Commit(value)
  }
}

impl From<ComAtprotoSyncSubscribeReposIdentity> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposIdentity) -> Self {
    Self::Identity(value)
  }
}

impl From<ComAtprotoSyncSubscribeReposAccount> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposAccount) -> Self {
    Self::Account(value)
  }
}

impl From<ComAtprotoSyncSubscribeReposHandle> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposHandle) -> Self {
    Self::Handle(value)
  }
}

impl From<ComAtprotoSyncSubscribeReposMigrate> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposMigrate) -> Self {
    Self::Migrate(value)
  }
}

impl From<ComAtprotoSyncSubscribeReposTombstone> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposTombstone) -> Self {
    Self::Tombstone(value)
  }
}

impl From<ComAtprotoSyncSubscribeReposInfo> for Object {
  fn from(value: ComAtprotoSyncSubscribeReposInfo) -> Self {
    Self::Info(value)
  }
}

impl TryFrom<&reqwest_websocket::Message> for Object {
  type Error = crate::Error;
  fn try_from(value: &reqwest_websocket::Message) -> std::result::Result<Self, Self::Error> {
    if let reqwest_websocket::Message::Binary(bin) = value {
      let header = ciborium::from_reader::<ciborium::Value, _>(bin.as_slice())?;
      let mut buf = Vec::new();
      ciborium::into_writer(&header, &mut buf)?;
      if let Ok(commit) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposCommit, _>(&bin[buf.len()..])
      {
        return Ok(commit.into());
      } else if let Ok(identity) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposIdentity, _>(&bin[buf.len()..])
      {
        return Ok(identity.into());
      } else if let Ok(account) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposAccount, _>(&bin[buf.len()..])
      {
        return Ok(account.into());
      } else if let Ok(handle) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposHandle, _>(&bin[buf.len()..])
      {
        return Ok(handle.into());
      } else if let Ok(migrate) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposMigrate, _>(&bin[buf.len()..])
      {
        return Ok(migrate.into());
      } else if let Ok(tombstone) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposTombstone, _>(&bin[buf.len()..])
      {
        return Ok(tombstone.into());
      } else if let Ok(info) =
        ciborium::from_reader::<ComAtprotoSyncSubscribeReposInfo, _>(&bin[buf.len()..])
      {
        return Ok(info.into());
      } else {
        return Err(Error::Other(String::from("unknown data type")));
      }
    }
    return Err(Error::Other(String::from("not binary message")));
  }
}

impl Object {
  pub fn as_commit(&self) -> Option<&ComAtprotoSyncSubscribeReposCommit> {
    match self {
      Self::Commit(c) => Some(c),
      _ => None,
    }
  }

  pub fn as_identity(&self) -> Option<&ComAtprotoSyncSubscribeReposIdentity> {
    match self {
      Self::Identity(i) => Some(i),
      _ => None,
    }
  }

  pub fn as_account(&self) -> Option<&ComAtprotoSyncSubscribeReposAccount> {
    match self {
      Self::Account(a) => Some(a),
      _ => None,
    }
  }

  pub fn as_handle(&self) -> Option<&ComAtprotoSyncSubscribeReposHandle> {
    match self {
      Self::Handle(h) => Some(h),
      _ => None,
    }
  }

  pub fn as_migrate(&self) -> Option<&ComAtprotoSyncSubscribeReposMigrate> {
    match self {
      Self::Migrate(m) => Some(m),
      _ => None,
    }
  }

  pub fn as_tombstone(&self) -> Option<&ComAtprotoSyncSubscribeReposTombstone> {
    match self {
      Self::Tombstone(t) => Some(t),
      _ => None,
    }
  }

  pub fn as_info(&self) -> Option<&ComAtprotoSyncSubscribeReposInfo> {
    match self {
      Self::Info(i) => Some(i),
      _ => None,
    }
  }
}

impl ComAtprotoSyncSubscribeReposCommit {
  pub async fn to_records(&self) -> Vec<Record> {
    let Ok((blocks, _)) = rs_car::car_read_all(&mut self.blocks.as_slice(), true).await else {
      return Vec::new();
    };
    blocks
      .iter()
      .filter_map(|(_, block)| {
        let value = ciborium::from_reader::<ciborium::Value, _>(block.as_slice()).ok()?;
        value.into_map().ok().and_then(|map| {
          match map.iter().find_map(|(k, v)| {
            k.as_text()
              .and_then(|t| (t == "$type").then(|| v).and_then(|v| v.as_text()))
          }) {
            Some("app.bsky.actor.profile") => {
              ciborium::from_reader::<AppBskyActorProfile, _>(block.as_slice())
                .map(|v| Record::AppBskyActorProfile(v))
                .ok()
            }
            Some("app.bsky.feed.generator") => {
              ciborium::from_reader::<AppBskyFeedGenerator, _>(block.as_slice())
                .map(|v| Record::AppBskyFeedGenerator(v))
                .ok()
            }
            Some("app.bsky.feed.like") => {
              ciborium::from_reader::<AppBskyFeedLike, _>(block.as_slice())
                .map(|v| Record::AppBskyFeedLike(v))
                .ok()
            }
            Some("app.bsky.feed.post") => {
              ciborium::from_reader::<AppBskyFeedPost, _>(block.as_slice())
                .map(|v| Record::AppBskyFeedPost(v))
                .ok()
            }
            Some("app.bsky.feed.postgate") => {
              ciborium::from_reader::<AppBskyFeedPostgate, _>(block.as_slice())
                .map(|v| Record::AppBskyFeedPostgate(v))
                .ok()
            }
            Some("app.bsky.feed.repost") => {
              ciborium::from_reader::<AppBskyFeedRepost, _>(block.as_slice())
                .map(|v| Record::AppBskyFeedRepost(v))
                .ok()
            }
            Some("app.bsky.feed.threadgate") => {
              ciborium::from_reader::<AppBskyFeedThreadgate, _>(block.as_slice())
                .map(|v| Record::AppBskyFeedThreadgate(v))
                .ok()
            }
            Some("app.bsky.graph.block") => {
              ciborium::from_reader::<AppBskyGraphBlock, _>(block.as_slice())
                .map(|v| Record::AppBskyGraphBlock(v))
                .ok()
            }
            Some("app.bsky.graph.follow") => {
              ciborium::from_reader::<AppBskyGraphFollow, _>(block.as_slice())
                .map(|v| Record::AppBskyGraphFollow(v))
                .ok()
            }
            Some("app.bsky.graph.list") => {
              ciborium::from_reader::<AppBskyGraphList, _>(block.as_slice())
                .map(|v| Record::AppBskyGraphList(v))
                .ok()
            }
            Some("app.bsky.graph.listblock") => {
              ciborium::from_reader::<AppBskyGraphListblock, _>(block.as_slice())
                .map(|v| Record::AppBskyGraphListblock(v))
                .ok()
            }
            Some("app.bsky.graph.listitem") => {
              ciborium::from_reader::<AppBskyGraphListitem, _>(block.as_slice())
                .map(|v| Record::AppBskyGraphListitem(v))
                .ok()
            }
            Some("app.bsky.graph.starterpack") => {
              ciborium::from_reader::<AppBskyGraphStarterpack, _>(block.as_slice())
                .map(|v| Record::AppBskyGraphStarterpack(v))
                .ok()
            }
            Some("app.bsky.labeler.service") => {
              ciborium::from_reader::<AppBskyLabelerService, _>(block.as_slice())
                .map(|v| Record::AppBskyLabelerService(v))
                .ok()
            }
            Some("chat.bsky.actor.declaration") => {
              ciborium::from_reader::<ChatBskyActorDeclaration, _>(block.as_slice())
                .map(|v| Record::ChatBskyActorDeclaration(v))
                .ok()
            }
            _ => None,
          }
        })
      })
      .collect::<Vec<_>>()
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Record {
  AppBskyActorProfile(AppBskyActorProfile),
  AppBskyFeedGenerator(AppBskyFeedGenerator),
  AppBskyFeedLike(AppBskyFeedLike),
  AppBskyFeedPost(AppBskyFeedPost),
  AppBskyFeedPostgate(AppBskyFeedPostgate),
  AppBskyFeedRepost(AppBskyFeedRepost),
  AppBskyFeedThreadgate(AppBskyFeedThreadgate),
  AppBskyGraphBlock(AppBskyGraphBlock),
  AppBskyGraphFollow(AppBskyGraphFollow),
  AppBskyGraphList(AppBskyGraphList),
  AppBskyGraphListblock(AppBskyGraphListblock),
  AppBskyGraphListitem(AppBskyGraphListitem),
  AppBskyGraphStarterpack(AppBskyGraphStarterpack),
  AppBskyLabelerService(AppBskyLabelerService),
  ChatBskyActorDeclaration(ChatBskyActorDeclaration),
}

impl Record {
  pub fn as_app_bsky_actor_profile(&self) -> Option<&AppBskyActorProfile> {
    match self {
      Self::AppBskyActorProfile(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_generator(&self) -> Option<&AppBskyFeedGenerator> {
    match self {
      Self::AppBskyFeedGenerator(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_like(&self) -> Option<&AppBskyFeedLike> {
    match self {
      Self::AppBskyFeedLike(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_post(&self) -> Option<&AppBskyFeedPost> {
    match self {
      Self::AppBskyFeedPost(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_postgate(&self) -> Option<&AppBskyFeedPostgate> {
    match self {
      Self::AppBskyFeedPostgate(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_repost(&self) -> Option<&AppBskyFeedRepost> {
    match self {
      Self::AppBskyFeedRepost(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_feed_threadgate(&self) -> Option<&AppBskyFeedThreadgate> {
    match self {
      Self::AppBskyFeedThreadgate(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_block(&self) -> Option<&AppBskyGraphBlock> {
    match self {
      Self::AppBskyGraphBlock(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_follow(&self) -> Option<&AppBskyGraphFollow> {
    match self {
      Self::AppBskyGraphFollow(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_list(&self) -> Option<&AppBskyGraphList> {
    match self {
      Self::AppBskyGraphList(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_listblock(&self) -> Option<&AppBskyGraphListblock> {
    match self {
      Self::AppBskyGraphListblock(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_listitem(&self) -> Option<&AppBskyGraphListitem> {
    match self {
      Self::AppBskyGraphListitem(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_graph_starterpack(&self) -> Option<&AppBskyGraphStarterpack> {
    match self {
      Self::AppBskyGraphStarterpack(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_app_bsky_labeler_service(&self) -> Option<&AppBskyLabelerService> {
    match self {
      Self::AppBskyLabelerService(v) => Some(v),
      _ => None,
    }
  }

  pub fn as_chat_bsky_actor_declaration(&self) -> Option<&ChatBskyActorDeclaration> {
    match self {
      Self::ChatBskyActorDeclaration(v) => Some(v),
      _ => None,
    }
  }
}

pub enum TextFacet {
  Mention(String),
  Link(String),
  Tag(String),
}

impl TextFacet {
  pub fn mention(did: &str) -> Self {
    Self::Mention(did.to_string())
  }

  pub fn link(url: &str) -> Self {
    Self::Link(url.to_string())
  }

  pub fn tag(tag: &str) -> Self {
    Self::Tag(tag.to_string())
  }

  pub fn to_atproto(&self) -> AppBskyRichtextFacetFeaturesUnion {
    match self {
      Self::Mention(v) => AppBskyRichtextFacetFeaturesUnion::AppBskyRichtextFacetMention(Box::new(
        AppBskyRichtextFacetMention {
          did: v.clone(),
          extra: std::collections::HashMap::new(),
        },
      )),
      Self::Link(v) => AppBskyRichtextFacetFeaturesUnion::AppBskyRichtextFacetLink(Box::new(
        AppBskyRichtextFacetLink {
          uri: v.clone(),
          extra: std::collections::HashMap::new(),
        },
      )),
      Self::Tag(v) => AppBskyRichtextFacetFeaturesUnion::AppBskyRichtextFacetTag(Box::new(
        AppBskyRichtextFacetTag {
          tag: v.clone(),
          extra: std::collections::HashMap::new(),
        },
      )),
    }
  }
}

pub struct TextDecoration {
  pub text: String,
  pub facet: TextFacet,
}

impl TextDecoration {
  pub fn new_mention(text: &str, did: &str) -> Self {
    Self {
      text: text.to_string(),
      facet: TextFacet::mention(did),
    }
  }

  pub fn new_link(text: &str, url: &str) -> Self {
    Self {
      text: text.to_string(),
      facet: TextFacet::link(url),
    }
  }

  pub fn new_tag(text: &str, tag: &str) -> Self {
    Self {
      text: text.to_string(),
      facet: TextFacet::tag(tag),
    }
  }

  pub fn to_atproto(&self, description: &str) -> Vec<AppBskyRichtextFacet> {
    let mut ret = Vec::new();
    let mut index = 0;
    while index < description.len() {
      match self.text.get(index..).and_then(|t| description.find(t)) {
        Some(start) => {
          let end = start + self.text.len();
          ret.push(AppBskyRichtextFacet {
            index: AppBskyRichtextFacetByteSlice {
              byte_start: start as i64,
              byte_end: end as i64,
              extra: std::collections::HashMap::new(),
            },
            features: vec![self.facet.to_atproto()],
            extra: std::collections::HashMap::new(),
          });
          index = end;
        }
        None => break,
      }
    }
    ret
  }
}

impl ComAtprotoSyncSubscribeReposCommit {
  pub fn to_aturi(&self) -> Option<String> {
    self
      .ops
      .first()
      .map(|op| format!("at://{}/{}", self.repo, op.path))
  }
}

pub fn get_did_from_request_header(headers: &axum::http::HeaderMap) -> Result<String> {
  let authorization = headers
    .get("Authorization")
    .ok_or_else(|| Error::Other(String::from("no authorization header")))?;
  let authorization = authorization
    .to_str()
    .map_err(|e| Error::Other(e.to_string()))?;
  let body = authorization
    .split(".")
    .nth(1)
    .ok_or_else(|| Error::Other(String::from("invalid jwt format")))?;
  let data = base64::Engine::decode(&base64::prelude::BASE64_STANDARD, &body)
    .map_err(|e| Error::Other(e.to_string()))?;
  let data = String::from_utf8(data).map_err(|e| Error::Other(e.to_string()))?;
  let object =
    serde_json::from_str::<serde_json::Value>(&data).map_err(|e| Error::Other(e.to_string()))?;
  let iss = object
    .get("iss")
    .and_then(|iss| iss.as_str())
    .ok_or_else(|| Error::Other(String::from("no iss entry")))?;
  Ok(iss.to_string())
}
