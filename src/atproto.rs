pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Reqwest(reqwest::Error),
  WebSocket(reqwest_websocket::Error),
  Parse((serde_json::Error, String)),
  Io(std::io::Error),
  CarDecode(String),
  CarEncode(String),
  Rate((i64, i64, i64, String)),
  Other(String),
}

impl From<reqwest::Error> for Error {
  fn from(value: reqwest::Error) -> Self {
    Self::Reqwest(value)
  }
}

impl From<reqwest_websocket::Error> for Error {
  fn from(value: reqwest_websocket::Error) -> Self {
    Self::WebSocket(value)
  }
}

impl From<std::io::Error> for Error {
  fn from(value: std::io::Error) -> Self {
    Self::Io(value)
  }
}

impl<T: std::fmt::Debug> From<ciborium::de::Error<T>> for Error {
  fn from(value: ciborium::de::Error<T>) -> Self {
    Self::CarDecode(format!("{:?}", value))
  }
}

impl<T: std::fmt::Debug> From<ciborium::ser::Error<T>> for Error {
  fn from(value: ciborium::ser::Error<T>) -> Self {
    Self::CarEncode(format!("{:?}", value))
  }
}

impl From<(serde_json::Error, String)> for Error {
  fn from(value: (serde_json::Error, String)) -> Self {
    Self::Parse(value)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tid(pub String);

impl Tid {
  pub fn encoding() -> Result<data_encoding::Encoding> {
    let mut spec = data_encoding::Specification::new();
    spec.symbols.push_str("234567abcdefghijklmnopqrstuvwxyz");
    spec.bit_order = data_encoding::BitOrder::MostSignificantFirst;
    spec.check_trailing_bits = true;
    spec.padding = None;
    Ok(spec.encoding().map_err(|e| Error::Other(format!("{e}")))?)
  }
}

impl From<String> for Tid {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl From<&str> for Tid {
  fn from(value: &str) -> Self {
    Self(value.to_string())
  }
}

impl Tid {
  pub fn new(datetime: chrono::DateTime<chrono::Utc>, clock_id: u16) -> Self {
    let ts = ((datetime.timestamp_micros() as i128) << 73) & 0x7ffffffffffffe000000000000000000;
    let id = ((clock_id as i128) << 63) & 0x00000000000001ff8000000000000000;
    let data = ts | id;
    Self(
      Tid::encoding()
        .unwrap()
        .encode(&data.to_be_bytes())
        .chars()
        .take(13)
        .collect(),
    )
  }

  pub fn get_datetime(&self) -> Result<chrono::DateTime<chrono::Utc>> {
    let bytes = Tid::encoding()?
      .decode(format!("{}2222222222222", &self.0).as_bytes())
      .map_err(|e| Error::Other(format!("{e}")))?;
    let mut array = [0u8; 16];
    array.copy_from_slice(&bytes);
    let ts = (i128::from_be_bytes(array) & 0x7ffffffffffffe000000000000000000) >> 73;
    Ok(
      chrono::TimeZone::timestamp_micros(&chrono::Utc, ts as i64)
        .earliest()
        .ok_or_else(|| Error::Other(format!("cannot convert to datetime {}", &self.0)))?,
    )
  }

  pub fn get_clock_id(&self) -> Result<u16> {
    let bytes = Tid::encoding()?
      .decode(format!("{}2222222222222", self.0).as_bytes())
      .map_err(|e| Error::Other(format!("{e}")))?;
    let mut array = [0u8; 16];
    array.copy_from_slice(&bytes);
    let ts = (i128::from_be_bytes(array) & 0x70000000000001ff8000000000000000) >> 63;
    Ok(ts as u16)
  }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Link {
  #[serde(rename = "$link")]
  pub link: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
  #[serde(rename = "$type")]
  pub type_: String,
  #[serde(rename = "ref")]
  pub ref_: Option<Link>,
  pub mime_type: String,
  pub size: Option<i64>,
  pub cid: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsProfileViewBasic {
  pub did: String,
  pub handle: String,
  pub display_name: Option<String>,
  pub avatar: Option<String>,
  pub associated: Option<AppBskyActorDefsProfileAssociated>,
  pub viewer: Option<AppBskyActorDefsViewerState>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsProfileView {
  pub did: String,
  pub handle: String,
  pub display_name: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>,
  pub associated: Option<AppBskyActorDefsProfileAssociated>,
  pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  pub viewer: Option<AppBskyActorDefsViewerState>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsProfileViewDetailed {
  pub did: String,
  pub handle: String,
  pub display_name: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>,
  pub banner: Option<String>,
  pub followers_count: Option<i64>,
  pub follows_count: Option<i64>,
  pub posts_count: Option<i64>,
  pub associated: Option<AppBskyActorDefsProfileAssociated>,
  pub joined_via_starter_pack: Option<AppBskyGraphDefsStarterPackViewBasic>,
  pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  pub viewer: Option<AppBskyActorDefsViewerState>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsProfileAssociated {
  pub lists: Option<i64>,
  pub feedgens: Option<i64>,
  pub starter_packs: Option<i64>,
  pub labeler: Option<bool>,
  pub chat: Option<AppBskyActorDefsProfileAssociatedChat>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsProfileAssociatedChat {
  pub allow_incoming: String,
}

/// Metadata about the requesting account's relationship with the subject account. Only has meaningful content for authed requests.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsViewerState {
  pub muted: Option<bool>,
  pub muted_by_list: Option<AppBskyGraphDefsListViewBasic>,
  pub blocked_by: Option<bool>,
  pub blocking: Option<String>,
  pub blocking_by_list: Option<AppBskyGraphDefsListViewBasic>,
  pub following: Option<String>,
  pub followed_by: Option<String>,
  pub known_followers: Option<AppBskyActorDefsKnownFollowers>,
}

/// The subject's followers whom you also follow
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsKnownFollowers {
  pub count: i64,
  pub followers: Vec<AppBskyActorDefsProfileViewBasic>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyActorDefsPreferencesUnion {
  #[serde(rename = "app.bsky.actor.defs#adultContentPref")]
  AppBskyActorDefsAdultContentPref(Box<AppBskyActorDefsAdultContentPref>),
  #[serde(rename = "app.bsky.actor.defs#contentLabelPref")]
  AppBskyActorDefsContentLabelPref(Box<AppBskyActorDefsContentLabelPref>),
  #[serde(rename = "app.bsky.actor.defs#savedFeedsPref")]
  AppBskyActorDefsSavedFeedsPref(Box<AppBskyActorDefsSavedFeedsPref>),
  #[serde(rename = "app.bsky.actor.defs#savedFeedsPrefV2")]
  AppBskyActorDefsSavedFeedsPrefV2(Box<AppBskyActorDefsSavedFeedsPrefV2>),
  #[serde(rename = "app.bsky.actor.defs#personalDetailsPref")]
  AppBskyActorDefsPersonalDetailsPref(Box<AppBskyActorDefsPersonalDetailsPref>),
  #[serde(rename = "app.bsky.actor.defs#feedViewPref")]
  AppBskyActorDefsFeedViewPref(Box<AppBskyActorDefsFeedViewPref>),
  #[serde(rename = "app.bsky.actor.defs#threadViewPref")]
  AppBskyActorDefsThreadViewPref(Box<AppBskyActorDefsThreadViewPref>),
  #[serde(rename = "app.bsky.actor.defs#interestsPref")]
  AppBskyActorDefsInterestsPref(Box<AppBskyActorDefsInterestsPref>),
  #[serde(rename = "app.bsky.actor.defs#mutedWordsPref")]
  AppBskyActorDefsMutedWordsPref(Box<AppBskyActorDefsMutedWordsPref>),
  #[serde(rename = "app.bsky.actor.defs#hiddenPostsPref")]
  AppBskyActorDefsHiddenPostsPref(Box<AppBskyActorDefsHiddenPostsPref>),
  #[serde(rename = "app.bsky.actor.defs#bskyAppStatePref")]
  AppBskyActorDefsBskyAppStatePref(Box<AppBskyActorDefsBskyAppStatePref>),
  #[serde(rename = "app.bsky.actor.defs#labelersPref")]
  AppBskyActorDefsLabelersPref(Box<AppBskyActorDefsLabelersPref>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyActorDefsPreferences(pub Vec<AppBskyActorDefsPreferencesUnion>);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsAdultContentPref {
  pub enabled: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsContentLabelPref {
  /// Which labeler does this preference apply to? If undefined, applies globally.,
  pub labeler_did: Option<String>,
  pub label: String,
  pub visibility: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsSavedFeed {
  pub id: String,
  pub type_: String,
  pub value: String,
  pub pinned: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsSavedFeedsPrefV2 {
  pub items: Vec<AppBskyActorDefsSavedFeed>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsSavedFeedsPref {
  pub pinned: Vec<String>,
  pub saved: Vec<String>,
  pub timeline_index: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsPersonalDetailsPref {
  /// The birth date of account owner.,
  pub birth_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsFeedViewPref {
  /// The URI of the feed, or an identifier which describes the feed.,
  pub feed: String,
  /// Hide replies in the feed.,
  pub hide_replies: Option<bool>,
  /// Hide replies in the feed if they are not by followed users.,
  pub hide_replies_by_unfollowed: Option<bool>,
  /// Hide replies in the feed if they do not have this number of likes.,
  pub hide_replies_by_like_count: Option<i64>,
  /// Hide reposts in the feed.,
  pub hide_reposts: Option<bool>,
  /// Hide quote posts in the feed.,
  pub hide_quote_posts: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsThreadViewPref {
  /// Sorting mode for threads.,
  pub sort: Option<String>,
  /// Show followed users at the top of all replies.,
  pub prioritize_followed_users: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsInterestsPref {
  /// A list of tags which describe the account owner's interests gathered during onboarding.,
  pub tags: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyActorDefsMutedWordTarget(pub String);

/// A word that the account owner has muted.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsMutedWord {
  pub id: Option<String>,
  /// The muted word itself.,
  pub value: String,
  /// The intended targets of the muted word.,
  pub targets: Vec<AppBskyActorDefsMutedWordTarget>,
  /// Groups of users to apply the muted word to. If undefined, applies to all users.,
  pub actor_target: Option<String>,
  /// The date and time at which the muted word will expire and no longer be applied.,
  pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsMutedWordsPref {
  /// A list of words the account owner has muted.,
  pub items: Vec<AppBskyActorDefsMutedWord>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsHiddenPostsPref {
  /// A list of URIs of posts the account owner has hidden.,
  pub items: Vec<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsLabelersPref {
  pub labelers: Vec<AppBskyActorDefsLabelerPrefItem>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsLabelerPrefItem {
  pub did: String,
}

/// A grab bag of state that's specific to the bsky.app program. Third-party apps shouldn't use this.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsBskyAppStatePref {
  pub active_progress_guide: Option<AppBskyActorDefsBskyAppProgressGuide>,
  /// An array of tokens which identify nudges (modals, popups, tours, highlight dots) that should be shown to the user.,
  pub queued_nudges: Option<Vec<String>>,
  /// Storage for NUXs the user has encountered.,
  pub nuxs: Option<Vec<AppBskyActorDefsNux>>,
}

/// If set, an active progress guide. Once completed, can be set to undefined. Should have unspecced fields tracking progress.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsBskyAppProgressGuide {
  pub guide: String,
}

/// A new user experiences (NUX) storage object
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorDefsNux {
  pub id: String,
  pub completed: bool,
  /// Arbitrary data for the NUX. The structure is defined by the NUX itself. Limited to 300 characters.,
  pub data: Option<String>,
  /// The date and time at which the NUX will expire and should be considered completed.,
  pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorGetPreferencesOutput {
  pub preferences: AppBskyActorDefsPreferences,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorGetProfilesOutput {
  pub profiles: Vec<AppBskyActorDefsProfileViewDetailed>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorGetSuggestionsOutput {
  pub cursor: Option<String>,
  pub actors: Vec<AppBskyActorDefsProfileView>,
}

/// Self-label values, specific to the Bluesky application, on the overall account.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyActorProfileLabelsUnion {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelfLabels(Box<ComAtprotoLabelDefsSelfLabels>),
}

/// A declaration of a Bluesky account profile.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorProfile {
  pub display_name: Option<String>,
  /// Free-form profile description text.,
  pub description: Option<String>,
  /// Small image to be displayed next to posts from account. AKA, 'profile picture',
  pub avatar: Option<Blob>,
  /// Larger horizontal image to display behind profile view.,
  pub banner: Option<Blob>,
  /// Self-label values, specific to the Bluesky application, on the overall account.,
  pub labels: Option<AppBskyActorProfileLabelsUnion>,
  pub joined_via_starter_pack: Option<ComAtprotoRepoStrongRef>,
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorPutPreferencesInput {
  pub preferences: AppBskyActorDefsPreferences,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorSearchActorsOutput {
  pub cursor: Option<String>,
  pub actors: Vec<AppBskyActorDefsProfileView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyActorSearchActorsTypeaheadOutput {
  pub actors: Vec<AppBskyActorDefsProfileViewBasic>,
}

/// width:height represents an aspect ratio. It may be approximate, and may not correspond to absolute dimensions in any given unit.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedDefsAspectRatio {
  pub width: i64,
  pub height: i64,
}

/// A representation of some externally linked content (eg, a URL and 'card'), embedded in a Bluesky record (eg, a post).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedExternal {
  pub external: AppBskyEmbedExternalExternal,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedExternalExternal {
  pub uri: String,
  pub title: String,
  pub description: String,
  pub thumb: Option<Blob>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedExternalView {
  pub external: AppBskyEmbedExternalViewExternal,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedExternalViewExternal {
  pub uri: String,
  pub title: String,
  pub description: String,
  pub thumb: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedImages {
  pub images: Vec<AppBskyEmbedImagesImage>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedImagesImage {
  pub image: Blob,
  /// Alt text description of the image, for accessibility.,
  pub alt: String,
  pub aspect_ratio: Option<AppBskyEmbedDefsAspectRatio>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedImagesView {
  pub images: Vec<AppBskyEmbedImagesViewImage>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedImagesViewImage {
  /// Fully-qualified URL where a thumbnail of the image can be fetched. For example, CDN location provided by the App View.,
  pub thumb: String,
  /// Fully-qualified URL where a large version of the image can be fetched. May or may not be the exact original blob. For example, CDN location provided by the App View.,
  pub fullsize: String,
  /// Alt text description of the image, for accessibility.,
  pub alt: String,
  pub aspect_ratio: Option<AppBskyEmbedDefsAspectRatio>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecord {
  pub record: ComAtprotoRepoStrongRef,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordViewRecordUnion {
  #[serde(rename = "app.bsky.embed.record#viewRecord")]
  AppBskyEmbedRecordViewRecord(Box<AppBskyEmbedRecordViewRecord>),
  #[serde(rename = "app.bsky.embed.record#viewNotFound")]
  AppBskyEmbedRecordViewNotFound(Box<AppBskyEmbedRecordViewNotFound>),
  #[serde(rename = "app.bsky.embed.record#viewBlocked")]
  AppBskyEmbedRecordViewBlocked(Box<AppBskyEmbedRecordViewBlocked>),
  #[serde(rename = "app.bsky.embed.record#viewDetached")]
  AppBskyEmbedRecordViewDetached(Box<AppBskyEmbedRecordViewDetached>),
  #[serde(rename = "app.bsky.feed.defs#generatorView")]
  AppBskyFeedDefsGeneratorView(Box<AppBskyFeedDefsGeneratorView>),
  #[serde(rename = "app.bsky.graph.defs#listView")]
  AppBskyGraphDefsListView(Box<AppBskyGraphDefsListView>),
  #[serde(rename = "app.bsky.labeler.defs#labelerView")]
  AppBskyLabelerDefsLabelerView(Box<AppBskyLabelerDefsLabelerView>),
  #[serde(rename = "app.bsky.graph.defs#starterPackViewBasic")]
  AppBskyGraphDefsStarterPackViewBasic(Box<AppBskyGraphDefsStarterPackViewBasic>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordView {
  pub record: AppBskyEmbedRecordViewRecordUnion,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordViewRecordEmbedsUnion {
  #[serde(rename = "app.bsky.embed.images#view")]
  AppBskyEmbedImagesView(Box<AppBskyEmbedImagesView>),
  #[serde(rename = "app.bsky.embed.video#view")]
  AppBskyEmbedVideoView(Box<AppBskyEmbedVideoView>),
  #[serde(rename = "app.bsky.embed.external#view")]
  AppBskyEmbedExternalView(Box<AppBskyEmbedExternalView>),
  #[serde(rename = "app.bsky.embed.record#view")]
  AppBskyEmbedRecordView(Box<AppBskyEmbedRecordView>),
  #[serde(rename = "app.bsky.embed.recordWithMedia#view")]
  AppBskyEmbedRecordWithMediaView(Box<AppBskyEmbedRecordWithMediaView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordViewRecord {
  pub uri: String,
  pub cid: String,
  pub author: AppBskyActorDefsProfileViewBasic,
  /// The record data itself.,
  pub value: serde_json::Value,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub reply_count: Option<i64>,
  pub repost_count: Option<i64>,
  pub like_count: Option<i64>,
  pub quote_count: Option<i64>,
  pub embeds: Option<Vec<AppBskyEmbedRecordViewRecordEmbedsUnion>>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordViewNotFound {
  pub uri: String,
  pub not_found: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordViewBlocked {
  pub uri: String,
  pub blocked: bool,
  pub author: AppBskyFeedDefsBlockedAuthor,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordViewDetached {
  pub uri: String,
  pub detached: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordWithMediaMediaUnion {
  #[serde(rename = "app.bsky.embed.images")]
  AppBskyEmbedImages(Box<AppBskyEmbedImages>),
  #[serde(rename = "app.bsky.embed.video")]
  AppBskyEmbedVideo(Box<AppBskyEmbedVideo>),
  #[serde(rename = "app.bsky.embed.external")]
  AppBskyEmbedExternal(Box<AppBskyEmbedExternal>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordWithMedia {
  pub record: AppBskyEmbedRecord,
  pub media: AppBskyEmbedRecordWithMediaMediaUnion,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyEmbedRecordWithMediaViewMediaUnion {
  #[serde(rename = "app.bsky.embed.images#view")]
  AppBskyEmbedImagesView(Box<AppBskyEmbedImagesView>),
  #[serde(rename = "app.bsky.embed.video#view")]
  AppBskyEmbedVideoView(Box<AppBskyEmbedVideoView>),
  #[serde(rename = "app.bsky.embed.external#view")]
  AppBskyEmbedExternalView(Box<AppBskyEmbedExternalView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedRecordWithMediaView {
  pub record: AppBskyEmbedRecordView,
  pub media: AppBskyEmbedRecordWithMediaViewMediaUnion,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedVideo {
  pub video: Blob,
  pub captions: Option<Vec<AppBskyEmbedVideoCaption>>,
  /// Alt text description of the video, for accessibility.,
  pub alt: Option<String>,
  pub aspect_ratio: Option<AppBskyEmbedDefsAspectRatio>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedVideoCaption {
  pub lang: String,
  pub file: Blob,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyEmbedVideoView {
  pub cid: String,
  pub playlist: String,
  pub thumbnail: Option<String>,
  pub alt: Option<String>,
  pub aspect_ratio: Option<AppBskyEmbedDefsAspectRatio>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsPostViewEmbedUnion {
  #[serde(rename = "app.bsky.embed.images#view")]
  AppBskyEmbedImagesView(Box<AppBskyEmbedImagesView>),
  #[serde(rename = "app.bsky.embed.video#view")]
  AppBskyEmbedVideoView(Box<AppBskyEmbedVideoView>),
  #[serde(rename = "app.bsky.embed.external#view")]
  AppBskyEmbedExternalView(Box<AppBskyEmbedExternalView>),
  #[serde(rename = "app.bsky.embed.record#view")]
  AppBskyEmbedRecordView(Box<AppBskyEmbedRecordView>),
  #[serde(rename = "app.bsky.embed.recordWithMedia#view")]
  AppBskyEmbedRecordWithMediaView(Box<AppBskyEmbedRecordWithMediaView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsPostView {
  pub uri: String,
  pub cid: String,
  pub author: AppBskyActorDefsProfileViewBasic,
  pub record: serde_json::Value,
  pub embed: Option<AppBskyFeedDefsPostViewEmbedUnion>,
  pub reply_count: Option<i64>,
  pub repost_count: Option<i64>,
  pub like_count: Option<i64>,
  pub quote_count: Option<i64>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub viewer: Option<AppBskyFeedDefsViewerState>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub threadgate: Option<AppBskyFeedDefsThreadgateView>,
}

/// Metadata about the requesting account's relationship with the subject content. Only has meaningful content for authed requests.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsViewerState {
  pub repost: Option<String>,
  pub like: Option<String>,
  pub thread_muted: Option<bool>,
  pub reply_disabled: Option<bool>,
  pub embedding_disabled: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsFeedViewPostReasonUnion {
  #[serde(rename = "app.bsky.feed.defs#reasonRepost")]
  AppBskyFeedDefsReasonRepost(Box<AppBskyFeedDefsReasonRepost>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsFeedViewPost {
  pub post: AppBskyFeedDefsPostView,
  pub reply: Option<AppBskyFeedDefsReplyRef>,
  pub reason: Option<AppBskyFeedDefsFeedViewPostReasonUnion>,
  /// Context provided by feed generator that may be passed back alongside interactions.,
  pub feed_context: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsReplyRefRootUnion {
  #[serde(rename = "app.bsky.feed.defs#postView")]
  AppBskyFeedDefsPostView(Box<AppBskyFeedDefsPostView>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotFoundPost(Box<AppBskyFeedDefsNotFoundPost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedPost(Box<AppBskyFeedDefsBlockedPost>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsReplyRefParentUnion {
  #[serde(rename = "app.bsky.feed.defs#postView")]
  AppBskyFeedDefsPostView(Box<AppBskyFeedDefsPostView>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotFoundPost(Box<AppBskyFeedDefsNotFoundPost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedPost(Box<AppBskyFeedDefsBlockedPost>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsReplyRef {
  pub root: AppBskyFeedDefsReplyRefRootUnion,
  pub parent: AppBskyFeedDefsReplyRefParentUnion,
  /// When parent is a reply to another post, this is the author of that post.,
  pub grandparent_author: Option<AppBskyActorDefsProfileViewBasic>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsReasonRepost {
  pub by: AppBskyActorDefsProfileViewBasic,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsThreadViewPostParentUnion {
  #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
  AppBskyFeedDefsThreadViewPost(Box<AppBskyFeedDefsThreadViewPost>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotFoundPost(Box<AppBskyFeedDefsNotFoundPost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedPost(Box<AppBskyFeedDefsBlockedPost>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsThreadViewPostRepliesUnion {
  #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
  AppBskyFeedDefsThreadViewPost(Box<AppBskyFeedDefsThreadViewPost>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotFoundPost(Box<AppBskyFeedDefsNotFoundPost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedPost(Box<AppBskyFeedDefsBlockedPost>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsThreadViewPost {
  pub post: AppBskyFeedDefsPostView,
  pub parent: Option<AppBskyFeedDefsThreadViewPostParentUnion>,
  pub replies: Option<Vec<AppBskyFeedDefsThreadViewPostRepliesUnion>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsNotFoundPost {
  pub uri: String,
  pub not_found: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsBlockedPost {
  pub uri: String,
  pub blocked: bool,
  pub author: AppBskyFeedDefsBlockedAuthor,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsBlockedAuthor {
  pub did: String,
  pub viewer: Option<AppBskyActorDefsViewerState>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsGeneratorView {
  pub uri: String,
  pub cid: String,
  pub did: String,
  pub creator: AppBskyActorDefsProfileView,
  pub display_name: String,
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<String>,
  pub like_count: Option<i64>,
  pub accepts_interactions: Option<bool>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub viewer: Option<AppBskyFeedDefsGeneratorViewerState>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsGeneratorViewerState {
  pub like: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedDefsSkeletonFeedPostReasonUnion {
  #[serde(rename = "app.bsky.feed.defs#skeletonReasonRepost")]
  AppBskyFeedDefsSkeletonReasonRepost(Box<AppBskyFeedDefsSkeletonReasonRepost>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsSkeletonFeedPost {
  pub post: String,
  pub reason: Option<AppBskyFeedDefsSkeletonFeedPostReasonUnion>,
  /// Context that will be passed through to client and may be passed to feed generator back alongside interactions.,
  pub feed_context: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsSkeletonReasonRepost {
  pub repost: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsThreadgateView {
  pub uri: Option<String>,
  pub cid: Option<String>,
  pub record: Option<serde_json::Value>,
  pub lists: Option<Vec<AppBskyGraphDefsListViewBasic>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDefsInteraction {
  pub item: Option<String>,
  pub event: Option<String>,
  /// Context on a feed item that was originally supplied by the feed generator on getFeedSkeleton.,
  pub feed_context: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDescribeFeedGeneratorOutput {
  pub did: String,
  pub feeds: Vec<AppBskyFeedDescribeFeedGeneratorFeed>,
  pub links: Option<AppBskyFeedDescribeFeedGeneratorLinks>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDescribeFeedGeneratorFeed {
  pub uri: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedDescribeFeedGeneratorLinks {
  pub privacy_policy: Option<String>,
  pub terms_of_service: Option<String>,
}

/// Self-label values

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedGeneratorLabelsUnion {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelfLabels(Box<ComAtprotoLabelDefsSelfLabels>),
}

/// Record declaring of the existence of a feed generator, and containing metadata about it. The record can exist in any repository.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGenerator {
  pub did: String,
  pub display_name: String,
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<Blob>,
  /// Declaration that a feed accepts feedback interactions from a client through app.bsky.feed.sendInteractions,
  pub accepts_interactions: Option<bool>,
  /// Self-label values,
  pub labels: Option<AppBskyFeedGeneratorLabelsUnion>,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetActorFeedsOutput {
  pub cursor: Option<String>,
  pub feeds: Vec<AppBskyFeedDefsGeneratorView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetActorLikesOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsFeedViewPost>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetAuthorFeedOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsFeedViewPost>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetFeedOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsFeedViewPost>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetFeedGeneratorOutput {
  pub view: AppBskyFeedDefsGeneratorView,
  /// Indicates whether the feed generator service has been online recently, or else seems to be inactive.,
  pub is_online: bool,
  /// Indicates whether the feed generator service is compatible with the record declaration.,
  pub is_valid: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetFeedGeneratorsOutput {
  pub feeds: Vec<AppBskyFeedDefsGeneratorView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetFeedSkeletonOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsSkeletonFeedPost>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetLikesOutput {
  pub uri: String,
  pub cid: Option<String>,
  pub cursor: Option<String>,
  pub likes: Vec<AppBskyFeedGetLikesLike>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetLikesLike {
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub actor: AppBskyActorDefsProfileView,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetListFeedOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsFeedViewPost>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedGetPostThreadOutputThreadUnion {
  #[serde(rename = "app.bsky.feed.defs#threadViewPost")]
  AppBskyFeedDefsThreadViewPost(Box<AppBskyFeedDefsThreadViewPost>),
  #[serde(rename = "app.bsky.feed.defs#notFoundPost")]
  AppBskyFeedDefsNotFoundPost(Box<AppBskyFeedDefsNotFoundPost>),
  #[serde(rename = "app.bsky.feed.defs#blockedPost")]
  AppBskyFeedDefsBlockedPost(Box<AppBskyFeedDefsBlockedPost>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetPostThreadOutput {
  pub thread: AppBskyFeedGetPostThreadOutputThreadUnion,
  pub threadgate: Option<AppBskyFeedDefsThreadgateView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetPostsOutput {
  pub posts: Vec<AppBskyFeedDefsPostView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetQuotesOutput {
  pub uri: String,
  pub cid: Option<String>,
  pub cursor: Option<String>,
  pub posts: Vec<AppBskyFeedDefsPostView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetRepostedByOutput {
  pub uri: String,
  pub cid: Option<String>,
  pub cursor: Option<String>,
  pub reposted_by: Vec<AppBskyActorDefsProfileView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetSuggestedFeedsOutput {
  pub cursor: Option<String>,
  pub feeds: Vec<AppBskyFeedDefsGeneratorView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedGetTimelineOutput {
  pub cursor: Option<String>,
  pub feed: Vec<AppBskyFeedDefsFeedViewPost>,
}

/// Record declaring a 'like' of a piece of subject content.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedLike {
  pub subject: ComAtprotoRepoStrongRef,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedPostEmbedUnion {
  #[serde(rename = "app.bsky.embed.images")]
  AppBskyEmbedImages(Box<AppBskyEmbedImages>),
  #[serde(rename = "app.bsky.embed.video")]
  AppBskyEmbedVideo(Box<AppBskyEmbedVideo>),
  #[serde(rename = "app.bsky.embed.external")]
  AppBskyEmbedExternal(Box<AppBskyEmbedExternal>),
  #[serde(rename = "app.bsky.embed.record")]
  AppBskyEmbedRecord(Box<AppBskyEmbedRecord>),
  #[serde(rename = "app.bsky.embed.recordWithMedia")]
  AppBskyEmbedRecordWithMedia(Box<AppBskyEmbedRecordWithMedia>),
}

/// Self-label values for this post. Effectively content warnings.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedPostLabelsUnion {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelfLabels(Box<ComAtprotoLabelDefsSelfLabels>),
}

/// Record containing a Bluesky post.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedPost {
  /// The primary post content. May be an empty string, if there are embeds.,
  pub text: String,
  /// DEPRECATED: replaced by app.bsky.richtext.facet.,
  pub entities: Option<Vec<AppBskyFeedPostEntity>>,
  /// Annotations of text (mentions, URLs, hashtags, etc),
  pub facets: Option<Vec<AppBskyRichtextFacet>>,
  pub reply: Option<AppBskyFeedPostReplyRef>,
  pub embed: Option<AppBskyFeedPostEmbedUnion>,
  /// Indicates human language of post primary text content.,
  pub langs: Option<Vec<String>>,
  /// Self-label values for this post. Effectively content warnings.,
  pub labels: Option<AppBskyFeedPostLabelsUnion>,
  /// Additional hashtags, in addition to any included in post text and facets.,
  pub tags: Option<Vec<String>>,
  /// Client-declared timestamp when this post was originally created.,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedPostReplyRef {
  pub root: ComAtprotoRepoStrongRef,
  pub parent: ComAtprotoRepoStrongRef,
}

/// Deprecated: use facets instead.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedPostEntity {
  pub index: AppBskyFeedPostTextSlice,
  /// Expected values are 'mention' and 'link'.,
  pub type_: String,
  pub value: String,
}

/// Deprecated. Use app.bsky.richtext instead -- A text segment. Start is inclusive, end is exclusive. Indices are for utf16-encoded strings.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedPostTextSlice {
  pub start: i64,
  pub end: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedPostgateEmbeddingRulesUnion {
  #[serde(rename = "app.bsky.feed.postgate#disableRule")]
  AppBskyFeedPostgateDisableRule(Box<AppBskyFeedPostgateDisableRule>),
}

/// Record defining interaction rules for a post. The record key (rkey) of the postgate record must match the record key of the post, and that record must be in the same repository.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedPostgate {
  pub created_at: chrono::DateTime<chrono::Utc>,
  /// Reference (AT-URI) to the post record.,
  pub post: String,
  /// List of AT-URIs embedding this post that the author has detached from.,
  pub detached_embedding_uris: Option<Vec<String>>,
  pub embedding_rules: Option<Vec<AppBskyFeedPostgateEmbeddingRulesUnion>>,
}

/// Disables embedding of this post.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyFeedPostgateDisableRule(pub serde_json::Value);

/// Record representing a 'repost' of an existing Bluesky post.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedRepost {
  pub subject: ComAtprotoRepoStrongRef,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedSearchPostsOutput {
  pub cursor: Option<String>,
  /// Count of search hits. Optional, may be rounded/truncated, and may not be possible to paginate through all hits.,
  pub hits_total: Option<i64>,
  pub posts: Vec<AppBskyFeedDefsPostView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedSendInteractionsInput {
  pub interactions: Vec<AppBskyFeedDefsInteraction>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyFeedSendInteractionsOutput(pub serde_json::Value);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyFeedThreadgateAllowUnion {
  #[serde(rename = "app.bsky.feed.threadgate#mentionRule")]
  AppBskyFeedThreadgateMentionRule(Box<AppBskyFeedThreadgateMentionRule>),
  #[serde(rename = "app.bsky.feed.threadgate#followingRule")]
  AppBskyFeedThreadgateFollowingRule(Box<AppBskyFeedThreadgateFollowingRule>),
  #[serde(rename = "app.bsky.feed.threadgate#listRule")]
  AppBskyFeedThreadgateListRule(Box<AppBskyFeedThreadgateListRule>),
}

/// Record defining interaction gating rules for a thread (aka, reply controls). The record key (rkey) of the threadgate record must match the record key of the thread's root post, and that record must be in the same repository..
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedThreadgate {
  /// Reference (AT-URI) to the post record.,
  pub post: String,
  pub allow: Option<Vec<AppBskyFeedThreadgateAllowUnion>>,
  pub created_at: chrono::DateTime<chrono::Utc>,
  /// List of hidden reply URIs.,
  pub hidden_replies: Option<Vec<String>>,
}

/// Allow replies from actors mentioned in your post.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyFeedThreadgateMentionRule(pub serde_json::Value);

/// Allow replies from actors you follow.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyFeedThreadgateFollowingRule(pub serde_json::Value);

/// Allow replies from actors on a list.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyFeedThreadgateListRule {
  pub list: String,
}

/// Record declaring a 'block' relationship against another account. NOTE: blocks are public in Bluesky; see blog posts for details.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphBlock {
  /// DID of the account to be blocked.,
  pub subject: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsListViewBasic {
  pub uri: String,
  pub cid: String,
  pub name: String,
  pub purpose: AppBskyGraphDefsListPurpose,
  pub avatar: Option<String>,
  pub list_item_count: Option<i64>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub viewer: Option<AppBskyGraphDefsListViewerState>,
  pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsListView {
  pub uri: String,
  pub cid: String,
  pub creator: AppBskyActorDefsProfileView,
  pub name: String,
  pub purpose: AppBskyGraphDefsListPurpose,
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<String>,
  pub list_item_count: Option<i64>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub viewer: Option<AppBskyGraphDefsListViewerState>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsListItemView {
  pub uri: String,
  pub subject: AppBskyActorDefsProfileView,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsStarterPackView {
  pub uri: String,
  pub cid: String,
  pub record: serde_json::Value,
  pub creator: AppBskyActorDefsProfileViewBasic,
  pub list: Option<AppBskyGraphDefsListViewBasic>,
  pub list_items_sample: Option<Vec<AppBskyGraphDefsListItemView>>,
  pub feeds: Option<Vec<AppBskyFeedDefsGeneratorView>>,
  pub joined_week_count: Option<i64>,
  pub joined_all_time_count: Option<i64>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsStarterPackViewBasic {
  pub uri: String,
  pub cid: String,
  pub record: serde_json::Value,
  pub creator: AppBskyActorDefsProfileViewBasic,
  pub list_item_count: Option<i64>,
  pub joined_week_count: Option<i64>,
  pub joined_all_time_count: Option<i64>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppBskyGraphDefsListPurpose(pub String);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsListViewerState {
  pub muted: Option<bool>,
  pub blocked: Option<String>,
}

/// indicates that a handle or DID could not be resolved
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsNotFoundActor {
  pub actor: String,
  pub not_found: bool,
}

/// lists the bi-directional graph relationships between one actor (not indicated in the object), and the target actors (the DID included in the object)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphDefsRelationship {
  pub did: String,
  /// if the actor follows this DID, this is the AT-URI of the follow record,
  pub following: Option<String>,
  /// if the actor is followed by this DID, contains the AT-URI of the follow record,
  pub followed_by: Option<String>,
}

/// Record declaring a social 'follow' relationship of another account. Duplicate follows will be ignored by the AppView.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphFollow {
  pub subject: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetActorStarterPacksOutput {
  pub cursor: Option<String>,
  pub starter_packs: Vec<AppBskyGraphDefsStarterPackViewBasic>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetBlocksOutput {
  pub cursor: Option<String>,
  pub blocks: Vec<AppBskyActorDefsProfileView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetFollowersOutput {
  pub subject: AppBskyActorDefsProfileView,
  pub cursor: Option<String>,
  pub followers: Vec<AppBskyActorDefsProfileView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetFollowsOutput {
  pub subject: AppBskyActorDefsProfileView,
  pub cursor: Option<String>,
  pub follows: Vec<AppBskyActorDefsProfileView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetKnownFollowersOutput {
  pub subject: AppBskyActorDefsProfileView,
  pub cursor: Option<String>,
  pub followers: Vec<AppBskyActorDefsProfileView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetListOutput {
  pub cursor: Option<String>,
  pub list: AppBskyGraphDefsListView,
  pub items: Vec<AppBskyGraphDefsListItemView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetListBlocksOutput {
  pub cursor: Option<String>,
  pub lists: Vec<AppBskyGraphDefsListView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetListMutesOutput {
  pub cursor: Option<String>,
  pub lists: Vec<AppBskyGraphDefsListView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetListsOutput {
  pub cursor: Option<String>,
  pub lists: Vec<AppBskyGraphDefsListView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetMutesOutput {
  pub cursor: Option<String>,
  pub mutes: Vec<AppBskyActorDefsProfileView>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyGraphGetRelationshipsOutputRelationshipsUnion {
  #[serde(rename = "app.bsky.graph.defs#relationship")]
  AppBskyGraphDefsRelationship(Box<AppBskyGraphDefsRelationship>),
  #[serde(rename = "app.bsky.graph.defs#notFoundActor")]
  AppBskyGraphDefsNotFoundActor(Box<AppBskyGraphDefsNotFoundActor>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetRelationshipsOutput {
  pub actor: Option<String>,
  pub relationships: Vec<AppBskyGraphGetRelationshipsOutputRelationshipsUnion>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetStarterPackOutput {
  pub starter_pack: AppBskyGraphDefsStarterPackView,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetStarterPacksOutput {
  pub starter_packs: Vec<AppBskyGraphDefsStarterPackViewBasic>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphGetSuggestedFollowsByActorOutput {
  pub suggestions: Vec<AppBskyActorDefsProfileView>,
  /// If true, response has fallen-back to generic results, and is not scoped using relativeToDid,
  pub is_fallback: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyGraphListLabelsUnion {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelfLabels(Box<ComAtprotoLabelDefsSelfLabels>),
}

/// Record representing a list of accounts (actors). Scope includes both moderation-oriented lists and curration-oriented lists.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphList {
  /// Defines the purpose of the list (aka, moderation-oriented or curration-oriented),
  pub purpose: AppBskyGraphDefsListPurpose,
  /// Display name for list; can not be empty.,
  pub name: String,
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  pub avatar: Option<Blob>,
  pub labels: Option<AppBskyGraphListLabelsUnion>,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Record representing a block relationship against an entire an entire list of accounts (actors).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphListblock {
  /// Reference (AT-URI) to the mod list record.,
  pub subject: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Record representing an account's inclusion on a specific list. The AppView will ignore duplicate listitem records.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphListitem {
  /// The account which is included on the list.,
  pub subject: String,
  /// Reference (AT-URI) to the list record (app.bsky.graph.list).,
  pub list: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphMuteActorInput {
  pub actor: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphMuteActorListInput {
  pub list: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphMuteThreadInput {
  pub root: String,
}

/// Record defining a starter pack of actors and feeds for new users.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphStarterpack {
  /// Display name for starter pack; can not be empty.,
  pub name: String,
  pub description: Option<String>,
  pub description_facets: Option<Vec<AppBskyRichtextFacet>>,
  /// Reference (AT-URI) to the list record.,
  pub list: String,
  pub feeds: Option<Vec<AppBskyGraphStarterpackFeedItem>>,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphStarterpackFeedItem {
  pub uri: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphUnmuteActorInput {
  pub actor: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphUnmuteActorListInput {
  pub list: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyGraphUnmuteThreadInput {
  pub root: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerDefsLabelerView {
  pub uri: String,
  pub cid: String,
  pub creator: AppBskyActorDefsProfileView,
  pub like_count: Option<i64>,
  pub viewer: Option<AppBskyLabelerDefsLabelerViewerState>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerDefsLabelerViewDetailed {
  pub uri: String,
  pub cid: String,
  pub creator: AppBskyActorDefsProfileView,
  pub policies: AppBskyLabelerDefsLabelerPolicies,
  pub like_count: Option<i64>,
  pub viewer: Option<AppBskyLabelerDefsLabelerViewerState>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerDefsLabelerViewerState {
  pub like: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerDefsLabelerPolicies {
  /// The label values which this labeler publishes. May include global or custom labels.,
  pub label_values: Vec<ComAtprotoLabelDefsLabelValue>,
  /// Label values created by this labeler and scoped exclusively to it. Labels defined here will override global label definitions for this labeler.,
  pub label_value_definitions: Option<Vec<ComAtprotoLabelDefsLabelValueDefinition>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyLabelerGetServicesOutputViewsUnion {
  #[serde(rename = "app.bsky.labeler.defs#labelerView")]
  AppBskyLabelerDefsLabelerView(Box<AppBskyLabelerDefsLabelerView>),
  #[serde(rename = "app.bsky.labeler.defs#labelerViewDetailed")]
  AppBskyLabelerDefsLabelerViewDetailed(Box<AppBskyLabelerDefsLabelerViewDetailed>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerGetServicesOutput {
  pub views: Vec<AppBskyLabelerGetServicesOutputViewsUnion>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyLabelerServiceLabelsUnion {
  #[serde(rename = "com.atproto.label.defs#selfLabels")]
  ComAtprotoLabelDefsSelfLabels(Box<ComAtprotoLabelDefsSelfLabels>),
}

/// A declaration of the existence of labeler service.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyLabelerService {
  pub policies: AppBskyLabelerDefsLabelerPolicies,
  pub labels: Option<AppBskyLabelerServiceLabelsUnion>,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationGetUnreadCountOutput {
  pub count: i64,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationListNotificationsOutput {
  pub cursor: Option<String>,
  pub notifications: Vec<AppBskyNotificationListNotificationsNotification>,
  pub priority: Option<bool>,
  pub seen_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationListNotificationsNotification {
  pub uri: String,
  pub cid: String,
  pub author: AppBskyActorDefsProfileView,
  /// Expected values are 'like', 'repost', 'follow', 'mention', 'reply', 'quote', and 'starterpack-joined'.,
  pub reason: String,
  pub reason_subject: Option<String>,
  pub record: serde_json::Value,
  pub is_read: bool,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationPutPreferencesInput {
  pub priority: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationRegisterPushInput {
  pub service_did: String,
  pub token: String,
  pub platform: String,
  pub app_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyNotificationUpdateSeenInput {
  pub seen_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum AppBskyRichtextFacetFeaturesUnion {
  #[serde(rename = "app.bsky.richtext.facet#mention")]
  AppBskyRichtextFacetMention(Box<AppBskyRichtextFacetMention>),
  #[serde(rename = "app.bsky.richtext.facet#link")]
  AppBskyRichtextFacetLink(Box<AppBskyRichtextFacetLink>),
  #[serde(rename = "app.bsky.richtext.facet#tag")]
  AppBskyRichtextFacetTag(Box<AppBskyRichtextFacetTag>),
}

/// Annotation of a sub-string within rich text.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyRichtextFacet {
  pub index: AppBskyRichtextFacetByteSlice,
  pub features: Vec<AppBskyRichtextFacetFeaturesUnion>,
}

/// Facet feature for mention of another account. The text is usually a handle, including a '@' prefix, but the facet reference is a DID.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyRichtextFacetMention {
  pub did: String,
}

/// Facet feature for a URL. The text URL may have been simplified or truncated, but the facet reference should be a complete URL.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyRichtextFacetLink {
  pub uri: String,
}

/// Facet feature for a hashtag. The text usually includes a '#' prefix, but the facet reference should not (except in the case of 'double hash tags').
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyRichtextFacetTag {
  pub tag: String,
}

/// Specifies the sub-string range a facet feature applies to. Start index is inclusive, end index is exclusive. Indices are zero-indexed, counting bytes of the UTF-8 encoded text. NOTE: some languages, like Javascript, use UTF-16 or Unicode codepoints for string slice indexing; in these languages, convert to byte arrays before working with facets.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyRichtextFacetByteSlice {
  pub byte_start: i64,
  pub byte_end: i64,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedDefsSkeletonSearchPost {
  pub uri: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedDefsSkeletonSearchActor {
  pub did: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedGetPopularFeedGeneratorsOutput {
  pub cursor: Option<String>,
  pub feeds: Vec<AppBskyFeedDefsGeneratorView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedGetSuggestionsSkeletonOutput {
  pub cursor: Option<String>,
  pub actors: Vec<AppBskyUnspeccedDefsSkeletonSearchActor>,
  /// DID of the account these suggestions are relative to. If this is returned undefined, suggestions are based on the viewer.,
  pub relative_to_did: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedGetTaggedSuggestionsOutput {
  pub suggestions: Vec<AppBskyUnspeccedGetTaggedSuggestionsSuggestion>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedGetTaggedSuggestionsSuggestion {
  pub tag: String,
  pub subject_type: String,
  pub subject: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedSearchActorsSkeletonOutput {
  pub cursor: Option<String>,
  /// Count of search hits. Optional, may be rounded/truncated, and may not be possible to paginate through all hits.,
  pub hits_total: Option<i64>,
  pub actors: Vec<AppBskyUnspeccedDefsSkeletonSearchActor>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyUnspeccedSearchPostsSkeletonOutput {
  pub cursor: Option<String>,
  /// Count of search hits. Optional, may be rounded/truncated, and may not be possible to paginate through all hits.,
  pub hits_total: Option<i64>,
  pub posts: Vec<AppBskyUnspeccedDefsSkeletonSearchPost>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyVideoDefsJobStatus {
  pub job_id: String,
  pub did: String,
  /// The state of the video processing job. All values not listed as a known value indicate that the job is in process.,
  pub state: String,
  /// Progress within the current processing state.,
  pub progress: Option<i64>,
  pub blob: Option<Blob>,
  pub error: Option<String>,
  pub message: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyVideoGetJobStatusOutput {
  pub job_status: AppBskyVideoDefsJobStatus,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyVideoGetUploadLimitsOutput {
  pub can_upload: bool,
  pub remaining_daily_videos: Option<i64>,
  pub remaining_daily_bytes: Option<i64>,
  pub message: Option<String>,
  pub error: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBskyVideoUploadVideoOutput {
  pub job_status: AppBskyVideoDefsJobStatus,
}

/// A declaration of a Bluesky chat account.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyActorDeclaration {
  pub allow_incoming: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyActorDefsProfileViewBasic {
  pub did: String,
  pub handle: String,
  pub display_name: Option<String>,
  pub avatar: Option<String>,
  pub associated: Option<AppBskyActorDefsProfileAssociated>,
  pub viewer: Option<AppBskyActorDefsViewerState>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  /// Set to true when the actor cannot actively participate in converations,
  pub chat_disabled: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatBskyActorDeleteAccountOutput(pub serde_json::Value);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsMessageRef {
  pub did: String,
  pub convo_id: String,
  pub message_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoDefsMessageInputEmbedUnion {
  #[serde(rename = "app.bsky.embed.record")]
  AppBskyEmbedRecord(Box<AppBskyEmbedRecord>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsMessageInput {
  pub text: String,
  /// Annotations of text (mentions, URLs, hashtags, etc),
  pub facets: Option<Vec<AppBskyRichtextFacet>>,
  pub embed: Option<ChatBskyConvoDefsMessageInputEmbedUnion>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoDefsMessageViewEmbedUnion {
  #[serde(rename = "app.bsky.embed.record#view")]
  AppBskyEmbedRecordView(Box<AppBskyEmbedRecordView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsMessageView {
  pub id: String,
  pub rev: String,
  pub text: String,
  /// Annotations of text (mentions, URLs, hashtags, etc),
  pub facets: Option<Vec<AppBskyRichtextFacet>>,
  pub embed: Option<ChatBskyConvoDefsMessageViewEmbedUnion>,
  pub sender: ChatBskyConvoDefsMessageViewSender,
  pub sent_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsDeletedMessageView {
  pub id: String,
  pub rev: String,
  pub sender: ChatBskyConvoDefsMessageViewSender,
  pub sent_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsMessageViewSender {
  pub did: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoDefsConvoViewLastMessageUnion {
  #[serde(rename = "chat.bsky.convo.defs#messageView")]
  ChatBskyConvoDefsMessageView(Box<ChatBskyConvoDefsMessageView>),
  #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
  ChatBskyConvoDefsDeletedMessageView(Box<ChatBskyConvoDefsDeletedMessageView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsConvoView {
  pub id: String,
  pub rev: String,
  pub members: Vec<ChatBskyActorDefsProfileViewBasic>,
  pub last_message: Option<ChatBskyConvoDefsConvoViewLastMessageUnion>,
  pub muted: bool,
  pub unread_count: i64,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsLogBeginConvo {
  pub rev: String,
  pub convo_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsLogLeaveConvo {
  pub rev: String,
  pub convo_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoDefsLogCreateMessageMessageUnion {
  #[serde(rename = "chat.bsky.convo.defs#messageView")]
  ChatBskyConvoDefsMessageView(Box<ChatBskyConvoDefsMessageView>),
  #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
  ChatBskyConvoDefsDeletedMessageView(Box<ChatBskyConvoDefsDeletedMessageView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsLogCreateMessage {
  pub rev: String,
  pub convo_id: String,
  pub message: ChatBskyConvoDefsLogCreateMessageMessageUnion,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoDefsLogDeleteMessageMessageUnion {
  #[serde(rename = "chat.bsky.convo.defs#messageView")]
  ChatBskyConvoDefsMessageView(Box<ChatBskyConvoDefsMessageView>),
  #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
  ChatBskyConvoDefsDeletedMessageView(Box<ChatBskyConvoDefsDeletedMessageView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDefsLogDeleteMessage {
  pub rev: String,
  pub convo_id: String,
  pub message: ChatBskyConvoDefsLogDeleteMessageMessageUnion,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoDeleteMessageForSelfInput {
  pub convo_id: String,
  pub message_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoGetConvoOutput {
  pub convo: ChatBskyConvoDefsConvoView,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoGetConvoForMembersOutput {
  pub convo: ChatBskyConvoDefsConvoView,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoGetLogOutputLogsUnion {
  #[serde(rename = "chat.bsky.convo.defs#logBeginConvo")]
  ChatBskyConvoDefsLogBeginConvo(Box<ChatBskyConvoDefsLogBeginConvo>),
  #[serde(rename = "chat.bsky.convo.defs#logLeaveConvo")]
  ChatBskyConvoDefsLogLeaveConvo(Box<ChatBskyConvoDefsLogLeaveConvo>),
  #[serde(rename = "chat.bsky.convo.defs#logCreateMessage")]
  ChatBskyConvoDefsLogCreateMessage(Box<ChatBskyConvoDefsLogCreateMessage>),
  #[serde(rename = "chat.bsky.convo.defs#logDeleteMessage")]
  ChatBskyConvoDefsLogDeleteMessage(Box<ChatBskyConvoDefsLogDeleteMessage>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoGetLogOutput {
  pub cursor: Option<String>,
  pub logs: Vec<ChatBskyConvoGetLogOutputLogsUnion>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyConvoGetMessagesOutputMessagesUnion {
  #[serde(rename = "chat.bsky.convo.defs#messageView")]
  ChatBskyConvoDefsMessageView(Box<ChatBskyConvoDefsMessageView>),
  #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
  ChatBskyConvoDefsDeletedMessageView(Box<ChatBskyConvoDefsDeletedMessageView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoGetMessagesOutput {
  pub cursor: Option<String>,
  pub messages: Vec<ChatBskyConvoGetMessagesOutputMessagesUnion>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoLeaveConvoInput {
  pub convo_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoLeaveConvoOutput {
  pub convo_id: String,
  pub rev: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoListConvosOutput {
  pub cursor: Option<String>,
  pub convos: Vec<ChatBskyConvoDefsConvoView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoMuteConvoInput {
  pub convo_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoMuteConvoOutput {
  pub convo: ChatBskyConvoDefsConvoView,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoSendMessageInput {
  pub convo_id: String,
  pub message: ChatBskyConvoDefsMessageInput,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoSendMessageBatchInput {
  pub items: Vec<ChatBskyConvoSendMessageBatchBatchItem>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoSendMessageBatchOutput {
  pub items: Vec<ChatBskyConvoDefsMessageView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoSendMessageBatchBatchItem {
  pub convo_id: String,
  pub message: ChatBskyConvoDefsMessageInput,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoUnmuteConvoInput {
  pub convo_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoUnmuteConvoOutput {
  pub convo: ChatBskyConvoDefsConvoView,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoUpdateReadInput {
  pub convo_id: String,
  pub message_id: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyConvoUpdateReadOutput {
  pub convo: ChatBskyConvoDefsConvoView,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyModerationGetActorMetadataOutput {
  pub day: ChatBskyModerationGetActorMetadataMetadata,
  pub month: ChatBskyModerationGetActorMetadataMetadata,
  pub all: ChatBskyModerationGetActorMetadataMetadata,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyModerationGetActorMetadataMetadata {
  pub messages_sent: i64,
  pub messages_received: i64,
  pub convos: i64,
  pub convos_started: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ChatBskyModerationGetMessageContextOutputMessagesUnion {
  #[serde(rename = "chat.bsky.convo.defs#messageView")]
  ChatBskyConvoDefsMessageView(Box<ChatBskyConvoDefsMessageView>),
  #[serde(rename = "chat.bsky.convo.defs#deletedMessageView")]
  ChatBskyConvoDefsDeletedMessageView(Box<ChatBskyConvoDefsDeletedMessageView>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyModerationGetMessageContextOutput {
  pub messages: Vec<ChatBskyModerationGetMessageContextOutputMessagesUnion>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBskyModerationUpdateActorAccessInput {
  pub actor: String,
  pub allow_access: bool,
  pub ref_: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDefsStatusAttr {
  pub applied: bool,
  pub ref_: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDefsAccountView {
  pub did: String,
  pub handle: String,
  pub email: Option<String>,
  pub related_records: Option<Vec<serde_json::Value>>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub invited_by: Option<ComAtprotoServerDefsInviteCode>,
  pub invites: Option<Vec<ComAtprotoServerDefsInviteCode>>,
  pub invites_disabled: Option<bool>,
  pub email_confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
  pub invite_note: Option<String>,
  pub deactivated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDefsRepoRef {
  pub did: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDefsRepoBlobRef {
  pub did: String,
  pub cid: String,
  pub record_uri: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDeleteAccountInput {
  pub did: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDisableAccountInvitesInput {
  pub account: String,
  /// Optional reason for disabled invites.,
  pub note: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminDisableInviteCodesInput {
  pub codes: Option<Vec<String>>,
  pub accounts: Option<Vec<String>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminEnableAccountInvitesInput {
  pub account: String,
  /// Optional reason for enabled invites.,
  pub note: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminGetAccountInfosOutput {
  pub infos: Vec<ComAtprotoAdminDefsAccountView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminGetInviteCodesOutput {
  pub cursor: Option<String>,
  pub codes: Vec<ComAtprotoServerDefsInviteCode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminGetSubjectStatusOutputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
  #[serde(rename = "com.atproto.admin.defs#repoBlobRef")]
  ComAtprotoAdminDefsRepoBlobRef(Box<ComAtprotoAdminDefsRepoBlobRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminGetSubjectStatusOutput {
  pub subject: ComAtprotoAdminGetSubjectStatusOutputSubjectUnion,
  pub takedown: Option<ComAtprotoAdminDefsStatusAttr>,
  pub deactivated: Option<ComAtprotoAdminDefsStatusAttr>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminSearchAccountsOutput {
  pub cursor: Option<String>,
  pub accounts: Vec<ComAtprotoAdminDefsAccountView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminSendEmailInput {
  pub recipient_did: String,
  pub content: String,
  pub subject: Option<String>,
  pub sender_did: String,
  /// Additional comment by the sender that won't be used in the email itself but helpful to provide more context for moderators/reviewers,
  pub comment: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminSendEmailOutput {
  pub sent: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminUpdateAccountEmailInput {
  /// The handle or DID of the repo.,
  pub account: String,
  pub email: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminUpdateAccountHandleInput {
  pub did: String,
  pub handle: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminUpdateAccountPasswordInput {
  pub did: String,
  pub password: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminUpdateSubjectStatusInputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
  #[serde(rename = "com.atproto.admin.defs#repoBlobRef")]
  ComAtprotoAdminDefsRepoBlobRef(Box<ComAtprotoAdminDefsRepoBlobRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminUpdateSubjectStatusInput {
  pub subject: ComAtprotoAdminUpdateSubjectStatusInputSubjectUnion,
  pub takedown: Option<ComAtprotoAdminDefsStatusAttr>,
  pub deactivated: Option<ComAtprotoAdminDefsStatusAttr>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoAdminUpdateSubjectStatusOutputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
  #[serde(rename = "com.atproto.admin.defs#repoBlobRef")]
  ComAtprotoAdminDefsRepoBlobRef(Box<ComAtprotoAdminDefsRepoBlobRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoAdminUpdateSubjectStatusOutput {
  pub subject: ComAtprotoAdminUpdateSubjectStatusOutputSubjectUnion,
  pub takedown: Option<ComAtprotoAdminDefsStatusAttr>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentityGetRecommendedDidCredentialsOutput {
  /// Recommended rotation keys for PLC dids. Should be undefined (or ignored) for did:webs.,
  pub rotation_keys: Option<Vec<String>>,
  pub also_known_as: Option<Vec<String>>,
  pub verification_methods: Option<serde_json::Value>,
  pub services: Option<serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentityResolveHandleOutput {
  pub did: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentitySignPlcOperationInput {
  /// A token received through com.atproto.identity.requestPlcOperationSignature,
  pub token: Option<String>,
  pub rotation_keys: Option<Vec<String>>,
  pub also_known_as: Option<Vec<String>>,
  pub verification_methods: Option<serde_json::Value>,
  pub services: Option<serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentitySignPlcOperationOutput {
  /// A signed DID PLC operation.,
  pub operation: serde_json::Value,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentitySubmitPlcOperationInput {
  pub operation: serde_json::Value,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoIdentityUpdateHandleInput {
  /// The new handle.,
  pub handle: String,
}

/// Metadata tag on an atproto resource (eg, repo or record).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelDefsLabel {
  /// The AT Protocol version of the label object.,
  pub ver: Option<i64>,
  /// DID of the actor who created this label.,
  pub src: String,
  /// AT URI of the record, repository (account), or other resource that this label applies to.,
  pub uri: String,
  /// Optionally, CID specifying the specific version of 'uri' resource this label applies to.,
  pub cid: Option<String>,
  /// The short string name of the value or type of this label.,
  pub val: String,
  /// If true, this is a negation label, overwriting a previous label.,
  pub neg: Option<bool>,
  /// Timestamp when this label was created.,
  pub cts: chrono::DateTime<chrono::Utc>,
  /// Timestamp at which this label expires (no longer applies).,
  pub exp: Option<chrono::DateTime<chrono::Utc>>,
  pub sig: Option<Vec<u8>>,
}

/// Metadata tags on an atproto record, published by the author within the record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelDefsSelfLabels {
  pub values: Vec<ComAtprotoLabelDefsSelfLabel>,
}

/// Metadata tag on an atproto record, published by the author within the record. Note that schemas should use #selfLabels, not #selfLabel.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelDefsSelfLabel {
  /// The short string name of the value or type of this label.,
  pub val: String,
}

/// Declares a label value and its expected interpretations and behaviors.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelDefsLabelValueDefinition {
  /// The value of the label being defined. Must only include lowercase ascii and the '-' character ([a-z-]+).,
  pub identifier: String,
  /// How should a client visually convey this label? 'inform' means neutral and informational; 'alert' means negative and warning; 'none' means show nothing.,
  pub severity: String,
  /// What should this label hide in the UI, if applied? 'content' hides all of the target; 'media' hides the images/video/audio; 'none' hides nothing.,
  pub blurs: String,
  /// The default setting for this label.,
  pub default_setting: Option<String>,
  /// Does the user need to have adult content enabled in order to configure this label?,
  pub adult_only: Option<bool>,
  pub locales: Vec<ComAtprotoLabelDefsLabelValueDefinitionStrings>,
}

/// Strings which describe the label in the UI, localized into a specific language.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelDefsLabelValueDefinitionStrings {
  /// The code of the language these strings are written in.,
  pub lang: String,
  /// A short human-readable name for the label.,
  pub name: String,
  /// A longer description of what the label means and why it might be applied.,
  pub description: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComAtprotoLabelDefsLabelValue(pub String);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelQueryLabelsOutput {
  pub cursor: Option<String>,
  pub labels: Vec<ComAtprotoLabelDefsLabel>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelSubscribeLabelsLabels {
  pub seq: i64,
  pub labels: Vec<ComAtprotoLabelDefsLabel>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoLabelSubscribeLabelsInfo {
  pub name: String,
  pub message: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoModerationCreateReportInputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoModerationCreateReportInput {
  /// Indicates the broad category of violation the report is for.,
  pub reason_type: ComAtprotoModerationDefsReasonType,
  /// Additional context about the content and violation.,
  pub reason: Option<String>,
  pub subject: ComAtprotoModerationCreateReportInputSubjectUnion,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoModerationCreateReportOutputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoModerationCreateReportOutput {
  pub id: i64,
  pub reason_type: ComAtprotoModerationDefsReasonType,
  pub reason: Option<String>,
  pub subject: ComAtprotoModerationCreateReportOutputSubjectUnion,
  pub reported_by: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComAtprotoModerationDefsReasonType(pub String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoRepoApplyWritesInputWritesUnion {
  #[serde(rename = "com.atproto.repo.applyWrites#create")]
  ComAtprotoRepoApplyWritesCreate(Box<ComAtprotoRepoApplyWritesCreate>),
  #[serde(rename = "com.atproto.repo.applyWrites#update")]
  ComAtprotoRepoApplyWritesUpdate(Box<ComAtprotoRepoApplyWritesUpdate>),
  #[serde(rename = "com.atproto.repo.applyWrites#delete")]
  ComAtprotoRepoApplyWritesDelete(Box<ComAtprotoRepoApplyWritesDelete>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesInput {
  /// The handle or DID of the repo (aka, current account).,
  pub repo: String,
  /// Can be set to 'false' to skip Lexicon schema validation of record data across all operations, 'true' to require it, or leave unset to validate only for known Lexicons.,
  pub validate: Option<bool>,
  pub writes: Vec<ComAtprotoRepoApplyWritesInputWritesUnion>,
  /// If provided, the entire operation will fail if the current repo commit CID does not match this value. Used to prevent conflicting repo mutations.,
  pub swap_commit: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ComAtprotoRepoApplyWritesOutputResultsUnion {
  #[serde(rename = "com.atproto.repo.applyWrites#createResult")]
  ComAtprotoRepoApplyWritesCreateResult(Box<ComAtprotoRepoApplyWritesCreateResult>),
  #[serde(rename = "com.atproto.repo.applyWrites#updateResult")]
  ComAtprotoRepoApplyWritesUpdateResult(Box<ComAtprotoRepoApplyWritesUpdateResult>),
  #[serde(rename = "com.atproto.repo.applyWrites#deleteResult")]
  ComAtprotoRepoApplyWritesDeleteResult(Box<ComAtprotoRepoApplyWritesDeleteResult>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesOutput {
  pub commit: Option<ComAtprotoRepoDefsCommitMeta>,
  pub results: Option<Vec<ComAtprotoRepoApplyWritesOutputResultsUnion>>,
}

/// Operation which creates a new record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesCreate {
  pub collection: String,
  pub rkey: Option<String>,
  pub value: serde_json::Value,
}

/// Operation which updates an existing record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesUpdate {
  pub collection: String,
  pub rkey: String,
  pub value: serde_json::Value,
}

/// Operation which deletes an existing record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesDelete {
  pub collection: String,
  pub rkey: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesCreateResult {
  pub uri: String,
  pub cid: String,
  pub validation_status: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoApplyWritesUpdateResult {
  pub uri: String,
  pub cid: String,
  pub validation_status: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComAtprotoRepoApplyWritesDeleteResult(pub serde_json::Value);

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoCreateRecordInput {
  /// The handle or DID of the repo (aka, current account).,
  pub repo: String,
  /// The NSID of the record collection.,
  pub collection: String,
  /// The Record Key.,
  pub rkey: Option<String>,
  /// Can be set to 'false' to skip Lexicon schema validation of record data, 'true' to require it, or leave unset to validate only for known Lexicons.,
  pub validate: Option<bool>,
  /// The record itself. Must contain a $type field.,
  pub record: serde_json::Value,
  /// Compare and swap with the previous commit by CID.,
  pub swap_commit: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoCreateRecordOutput {
  pub uri: String,
  pub cid: String,
  pub commit: Option<ComAtprotoRepoDefsCommitMeta>,
  pub validation_status: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoDefsCommitMeta {
  pub cid: String,
  pub rev: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoDeleteRecordInput {
  /// The handle or DID of the repo (aka, current account).,
  pub repo: String,
  /// The NSID of the record collection.,
  pub collection: String,
  /// The Record Key.,
  pub rkey: String,
  /// Compare and swap with the previous record by CID.,
  pub swap_record: Option<String>,
  /// Compare and swap with the previous commit by CID.,
  pub swap_commit: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoDeleteRecordOutput {
  pub commit: Option<ComAtprotoRepoDefsCommitMeta>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoDescribeRepoOutput {
  pub handle: String,
  pub did: String,
  /// The complete DID document for this account.,
  pub did_doc: serde_json::Value,
  /// List of all the collections (NSIDs) for which this repo contains at least one record.,
  pub collections: Vec<String>,
  /// Indicates if handle is currently valid (resolves bi-directionally),
  pub handle_is_correct: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoGetRecordOutput {
  pub uri: String,
  pub cid: Option<String>,
  pub value: serde_json::Value,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoListMissingBlobsOutput {
  pub cursor: Option<String>,
  pub blobs: Vec<ComAtprotoRepoListMissingBlobsRecordBlob>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoListMissingBlobsRecordBlob {
  pub cid: String,
  pub record_uri: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoListRecordsOutput {
  pub cursor: Option<String>,
  pub records: Vec<ComAtprotoRepoListRecordsRecord>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoListRecordsRecord {
  pub uri: String,
  pub cid: String,
  pub value: serde_json::Value,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoPutRecordInput {
  /// The handle or DID of the repo (aka, current account).,
  pub repo: String,
  /// The NSID of the record collection.,
  pub collection: String,
  /// The Record Key.,
  pub rkey: String,
  /// Can be set to 'false' to skip Lexicon schema validation of record data, 'true' to require it, or leave unset to validate only for known Lexicons.,
  pub validate: Option<bool>,
  /// The record to write.,
  pub record: serde_json::Value,
  /// Compare and swap with the previous record by CID. WARNING: nullable and optional field; may cause problems with golang implementation,
  pub swap_record: Option<String>,
  /// Compare and swap with the previous commit by CID.,
  pub swap_commit: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoPutRecordOutput {
  pub uri: String,
  pub cid: String,
  pub commit: Option<ComAtprotoRepoDefsCommitMeta>,
  pub validation_status: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoStrongRef {
  pub uri: String,
  pub cid: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoRepoUploadBlobOutput {
  pub blob: Blob,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCheckAccountStatusOutput {
  pub activated: bool,
  pub valid_did: bool,
  pub repo_commit: String,
  pub repo_rev: String,
  pub repo_blocks: i64,
  pub indexed_records: i64,
  pub private_state_values: i64,
  pub expected_blobs: i64,
  pub imported_blobs: i64,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerConfirmEmailInput {
  pub email: String,
  pub token: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateAccountInput {
  pub email: Option<String>,
  /// Requested handle for the account.,
  pub handle: String,
  /// Pre-existing atproto DID, being imported to a new account.,
  pub did: Option<String>,
  pub invite_code: Option<String>,
  pub verification_code: Option<String>,
  pub verification_phone: Option<String>,
  /// Initial account password. May need to meet instance-specific password strength requirements.,
  pub password: Option<String>,
  /// DID PLC rotation key (aka, recovery key) to be included in PLC creation operation.,
  pub recovery_key: Option<String>,
  /// A signed DID PLC operation to be submitted as part of importing an existing account to this instance. NOTE: this optional field may be updated when full account migration is implemented.,
  pub plc_op: Option<serde_json::Value>,
}

/// Account login session returned on successful account creation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateAccountOutput {
  pub access_jwt: String,
  pub refresh_jwt: String,
  pub handle: String,
  /// The DID of the new account.,
  pub did: String,
  /// Complete DID document.,
  pub did_doc: Option<serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateAppPasswordInput {
  /// A short name for the App Password, to help distinguish them.,
  pub name: String,
  /// If an app password has 'privileged' access to possibly sensitive account state. Meant for use with trusted clients.,
  pub privileged: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateAppPasswordAppPassword {
  pub name: String,
  pub password: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub privileged: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateInviteCodeInput {
  pub use_count: i64,
  pub for_account: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateInviteCodeOutput {
  pub code: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateInviteCodesInput {
  pub code_count: i64,
  pub use_count: i64,
  pub for_accounts: Option<Vec<String>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateInviteCodesOutput {
  pub codes: Vec<ComAtprotoServerCreateInviteCodesAccountCodes>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateInviteCodesAccountCodes {
  pub account: String,
  pub codes: Vec<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateSessionInput {
  /// Handle or other identifier supported by the server for the authenticating user.,
  pub identifier: String,
  pub password: String,
  pub auth_factor_token: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerCreateSessionOutput {
  pub access_jwt: String,
  pub refresh_jwt: String,
  pub handle: String,
  pub did: String,
  pub did_doc: Option<serde_json::Value>,
  pub email: Option<String>,
  pub email_confirmed: Option<bool>,
  pub email_auth_factor: Option<bool>,
  pub active: Option<bool>,
  /// If active=false, this optional field indicates a possible reason for why the account is not active. If active=false and no status is supplied, then the host makes no claim for why the repository is no longer being hosted.,
  pub status: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDeactivateAccountInput {
  /// A recommendation to server as to how long they should hold onto the deactivated account before deleting.,
  pub delete_after: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDefsInviteCode {
  pub code: String,
  pub available: i64,
  pub disabled: bool,
  pub for_account: String,
  pub created_by: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub uses: Vec<ComAtprotoServerDefsInviteCodeUse>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDefsInviteCodeUse {
  pub used_by: String,
  pub used_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDeleteAccountInput {
  pub did: String,
  pub password: String,
  pub token: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDescribeServerOutput {
  /// If true, an invite code must be supplied to create an account on this instance.,
  pub invite_code_required: Option<bool>,
  /// If true, a phone verification token must be supplied to create an account on this instance.,
  pub phone_verification_required: Option<bool>,
  /// List of domain suffixes that can be used in account handles.,
  pub available_user_domains: Vec<String>,
  /// URLs of service policy documents.,
  pub links: Option<ComAtprotoServerDescribeServerLinks>,
  /// Contact information,
  pub contact: Option<ComAtprotoServerDescribeServerContact>,
  pub did: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDescribeServerLinks {
  pub privacy_policy: Option<String>,
  pub terms_of_service: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerDescribeServerContact {
  pub email: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerGetAccountInviteCodesOutput {
  pub codes: Vec<ComAtprotoServerDefsInviteCode>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerGetServiceAuthOutput {
  pub token: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerGetSessionOutput {
  pub handle: String,
  pub did: String,
  pub email: Option<String>,
  pub email_confirmed: Option<bool>,
  pub email_auth_factor: Option<bool>,
  pub did_doc: Option<serde_json::Value>,
  pub active: Option<bool>,
  /// If active=false, this optional field indicates a possible reason for why the account is not active. If active=false and no status is supplied, then the host makes no claim for why the repository is no longer being hosted.,
  pub status: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerListAppPasswordsOutput {
  pub passwords: Vec<ComAtprotoServerListAppPasswordsAppPassword>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerListAppPasswordsAppPassword {
  pub name: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub privileged: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerRefreshSessionOutput {
  pub access_jwt: String,
  pub refresh_jwt: String,
  pub handle: String,
  pub did: String,
  pub did_doc: Option<serde_json::Value>,
  pub active: Option<bool>,
  /// Hosting status of the account. If not specified, then assume 'active'.,
  pub status: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerRequestEmailUpdateOutput {
  pub token_required: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerRequestPasswordResetInput {
  pub email: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerReserveSigningKeyInput {
  /// The DID to reserve a key for.,
  pub did: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerReserveSigningKeyOutput {
  /// The public key for the reserved signing key, in did:key serialization.,
  pub signing_key: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerResetPasswordInput {
  pub token: String,
  pub password: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerRevokeAppPasswordInput {
  pub name: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoServerUpdateEmailInput {
  pub email: String,
  pub email_auth_factor: Option<bool>,
  /// Requires a token from com.atproto.sever.requestEmailUpdate if the account's email has been confirmed.,
  pub token: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncGetHeadOutput {
  pub root: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncGetLatestCommitOutput {
  pub cid: String,
  pub rev: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncGetRepoStatusOutput {
  pub did: String,
  pub active: bool,
  /// If active=false, this optional field indicates a possible reason for why the account is not active. If active=false and no status is supplied, then the host makes no claim for why the repository is no longer being hosted.,
  pub status: Option<String>,
  /// Optional field, the current rev of the repo, if active=true,
  pub rev: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncListBlobsOutput {
  pub cursor: Option<String>,
  pub cids: Vec<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncListReposOutput {
  pub cursor: Option<String>,
  pub repos: Vec<ComAtprotoSyncListReposRepo>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncListReposRepo {
  pub did: String,
  /// Current repo commit CID,
  pub head: String,
  pub rev: String,
  pub active: Option<bool>,
  /// If active=false, this optional field indicates a possible reason for why the account is not active. If active=false and no status is supplied, then the host makes no claim for why the repository is no longer being hosted.,
  pub status: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncNotifyOfUpdateInput {
  /// Hostname of the current service (usually a PDS) that is notifying of update.,
  pub hostname: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncRequestCrawlInput {
  /// Hostname of the current service (eg, PDS) that is requesting to be crawled.,
  pub hostname: String,
}

/// Represents an update of repository state. Note that empty commits are allowed, which include no repo data changes, but an update to rev and signature.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposCommit {
  /// The stream sequence number of this message.,
  pub seq: i64,
  /// DEPRECATED -- unused,
  pub rebase: bool,
  /// Indicates that this commit contained too many ops, or data size was too large. Consumers will need to make a separate request to get missing data.,
  pub too_big: bool,
  /// The repo this event comes from.,
  pub repo: String,
  /// Repo commit object CID.,
  pub commit: ciborium::Value,
  /// DEPRECATED -- unused. WARNING -- nullable and optional; stick with optional to ensure golang interoperability.,
  pub prev: Option<ciborium::Value>,
  /// The rev of the emitted commit. Note that this information is also in the commit object included in blocks, unless this is a tooBig event.,
  pub rev: String,
  /// The rev of the last emitted commit from this repo (if any).,
  pub since: Option<String>,
  pub blocks: Vec<u8>,
  pub ops: Vec<ComAtprotoSyncSubscribeReposRepoOp>,
  pub blobs: Vec<ciborium::Value>,
  /// Timestamp of when this message was originally broadcast.,
  pub time: chrono::DateTime<chrono::Utc>,
}

/// Represents a change to an account's identity. Could be an updated handle, signing key, or pds hosting endpoint. Serves as a prod to all downstream services to refresh their identity cache.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposIdentity {
  pub seq: i64,
  pub did: String,
  pub time: chrono::DateTime<chrono::Utc>,
  /// The current handle for the account, or 'handle.invalid' if validation fails. This field is optional, might have been validated or passed-through from an upstream source. Semantics and behaviors for PDS vs Relay may evolve in the future; see atproto specs for more details.,
  pub handle: Option<String>,
}

/// Represents a change to an account's status on a host (eg, PDS or Relay). The semantics of this event are that the status is at the host which emitted the event, not necessarily that at the currently active PDS. Eg, a Relay takedown would emit a takedown with active=false, even if the PDS is still active.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposAccount {
  pub seq: i64,
  pub did: String,
  pub time: chrono::DateTime<chrono::Utc>,
  /// Indicates that the account has a repository which can be fetched from the host that emitted this event.,
  pub active: bool,
  /// If active=false, this optional field indicates a reason for why the account is not active.,
  pub status: Option<String>,
}

/// DEPRECATED -- Use #identity event instead
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposHandle {
  pub seq: i64,
  pub did: String,
  pub handle: String,
  pub time: chrono::DateTime<chrono::Utc>,
}

/// DEPRECATED -- Use #account event instead
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposMigrate {
  pub seq: i64,
  pub did: String,
  pub migrate_to: Option<String>,
  pub time: chrono::DateTime<chrono::Utc>,
}

/// DEPRECATED -- Use #account event instead
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposTombstone {
  pub seq: i64,
  pub did: String,
  pub time: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposInfo {
  pub name: String,
  pub message: Option<String>,
}

/// A repo operation, ie a mutation of a single record.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoSyncSubscribeReposRepoOp {
  pub action: String,
  pub path: String,
  /// For creates and updates, the new record CID. For deletions, null.,
  pub cid: Option<ciborium::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoTempCheckSignupQueueOutput {
  pub activated: bool,
  pub place_in_queue: Option<i64>,
  pub estimated_time_ms: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoTempFetchLabelsOutput {
  pub labels: Vec<ComAtprotoLabelDefsLabel>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComAtprotoTempRequestPhoneVerificationInput {
  pub phone_number: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneCommunicationCreateTemplateInput {
  /// Name of the template.,
  pub name: String,
  /// Content of the template, markdown supported, can contain variable placeholders.,
  pub content_markdown: String,
  /// Subject of the message, used in emails.,
  pub subject: String,
  /// Message language.,
  pub lang: Option<String>,
  /// DID of the user who is creating the template.,
  pub created_by: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneCommunicationDefsTemplateView {
  pub id: String,
  /// Name of the template.,
  pub name: String,
  /// Content of the template, can contain markdown and variable placeholders.,
  pub subject: Option<String>,
  /// Subject of the message, used in emails.,
  pub content_markdown: String,
  pub disabled: bool,
  /// Message language.,
  pub lang: Option<String>,
  /// DID of the user who last updated the template.,
  pub last_updated_by: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneCommunicationDeleteTemplateInput {
  pub id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneCommunicationListTemplatesOutput {
  pub communication_templates: Vec<ToolsOzoneCommunicationDefsTemplateView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneCommunicationUpdateTemplateInput {
  /// ID of the template to be updated.,
  pub id: String,
  /// Name of the template.,
  pub name: Option<String>,
  /// Message language.,
  pub lang: Option<String>,
  /// Content of the template, markdown supported, can contain variable placeholders.,
  pub content_markdown: Option<String>,
  /// Subject of the message, used in emails.,
  pub subject: Option<String>,
  /// DID of the user who is updating the template.,
  pub updated_by: Option<String>,
  pub disabled: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsModEventViewEventUnion {
  #[serde(rename = "tools.ozone.moderation.defs#modEventTakedown")]
  ToolsOzoneModerationDefsModEventTakedown(Box<ToolsOzoneModerationDefsModEventTakedown>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReverseTakedown")]
  ToolsOzoneModerationDefsModEventReverseTakedown(
    Box<ToolsOzoneModerationDefsModEventReverseTakedown>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventComment")]
  ToolsOzoneModerationDefsModEventComment(Box<ToolsOzoneModerationDefsModEventComment>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReport")]
  ToolsOzoneModerationDefsModEventReport(Box<ToolsOzoneModerationDefsModEventReport>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventLabel")]
  ToolsOzoneModerationDefsModEventLabel(Box<ToolsOzoneModerationDefsModEventLabel>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventAcknowledge")]
  ToolsOzoneModerationDefsModEventAcknowledge(Box<ToolsOzoneModerationDefsModEventAcknowledge>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEscalate")]
  ToolsOzoneModerationDefsModEventEscalate(Box<ToolsOzoneModerationDefsModEventEscalate>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMute")]
  ToolsOzoneModerationDefsModEventMute(Box<ToolsOzoneModerationDefsModEventMute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmute")]
  ToolsOzoneModerationDefsModEventUnmute(Box<ToolsOzoneModerationDefsModEventUnmute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMuteReporter")]
  ToolsOzoneModerationDefsModEventMuteReporter(Box<ToolsOzoneModerationDefsModEventMuteReporter>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmuteReporter")]
  ToolsOzoneModerationDefsModEventUnmuteReporter(
    Box<ToolsOzoneModerationDefsModEventUnmuteReporter>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEmail")]
  ToolsOzoneModerationDefsModEventEmail(Box<ToolsOzoneModerationDefsModEventEmail>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventResolveAppeal")]
  ToolsOzoneModerationDefsModEventResolveAppeal(Box<ToolsOzoneModerationDefsModEventResolveAppeal>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventDivert")]
  ToolsOzoneModerationDefsModEventDivert(Box<ToolsOzoneModerationDefsModEventDivert>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventTag")]
  ToolsOzoneModerationDefsModEventTag(Box<ToolsOzoneModerationDefsModEventTag>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsModEventViewSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
  #[serde(rename = "chat.bsky.convo.defs#messageRef")]
  ChatBskyConvoDefsMessageRef(Box<ChatBskyConvoDefsMessageRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventView {
  pub id: i64,
  pub event: ToolsOzoneModerationDefsModEventViewEventUnion,
  pub subject: ToolsOzoneModerationDefsModEventViewSubjectUnion,
  pub subject_blob_cids: Vec<String>,
  pub created_by: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub creator_handle: Option<String>,
  pub subject_handle: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsModEventViewDetailEventUnion {
  #[serde(rename = "tools.ozone.moderation.defs#modEventTakedown")]
  ToolsOzoneModerationDefsModEventTakedown(Box<ToolsOzoneModerationDefsModEventTakedown>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReverseTakedown")]
  ToolsOzoneModerationDefsModEventReverseTakedown(
    Box<ToolsOzoneModerationDefsModEventReverseTakedown>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventComment")]
  ToolsOzoneModerationDefsModEventComment(Box<ToolsOzoneModerationDefsModEventComment>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReport")]
  ToolsOzoneModerationDefsModEventReport(Box<ToolsOzoneModerationDefsModEventReport>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventLabel")]
  ToolsOzoneModerationDefsModEventLabel(Box<ToolsOzoneModerationDefsModEventLabel>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventAcknowledge")]
  ToolsOzoneModerationDefsModEventAcknowledge(Box<ToolsOzoneModerationDefsModEventAcknowledge>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEscalate")]
  ToolsOzoneModerationDefsModEventEscalate(Box<ToolsOzoneModerationDefsModEventEscalate>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMute")]
  ToolsOzoneModerationDefsModEventMute(Box<ToolsOzoneModerationDefsModEventMute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmute")]
  ToolsOzoneModerationDefsModEventUnmute(Box<ToolsOzoneModerationDefsModEventUnmute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMuteReporter")]
  ToolsOzoneModerationDefsModEventMuteReporter(Box<ToolsOzoneModerationDefsModEventMuteReporter>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmuteReporter")]
  ToolsOzoneModerationDefsModEventUnmuteReporter(
    Box<ToolsOzoneModerationDefsModEventUnmuteReporter>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEmail")]
  ToolsOzoneModerationDefsModEventEmail(Box<ToolsOzoneModerationDefsModEventEmail>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventResolveAppeal")]
  ToolsOzoneModerationDefsModEventResolveAppeal(Box<ToolsOzoneModerationDefsModEventResolveAppeal>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventDivert")]
  ToolsOzoneModerationDefsModEventDivert(Box<ToolsOzoneModerationDefsModEventDivert>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventTag")]
  ToolsOzoneModerationDefsModEventTag(Box<ToolsOzoneModerationDefsModEventTag>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsModEventViewDetailSubjectUnion {
  #[serde(rename = "tools.ozone.moderation.defs#repoView")]
  ToolsOzoneModerationDefsRepoView(Box<ToolsOzoneModerationDefsRepoView>),
  #[serde(rename = "tools.ozone.moderation.defs#repoViewNotFound")]
  ToolsOzoneModerationDefsRepoViewNotFound(Box<ToolsOzoneModerationDefsRepoViewNotFound>),
  #[serde(rename = "tools.ozone.moderation.defs#recordView")]
  ToolsOzoneModerationDefsRecordView(Box<ToolsOzoneModerationDefsRecordView>),
  #[serde(rename = "tools.ozone.moderation.defs#recordViewNotFound")]
  ToolsOzoneModerationDefsRecordViewNotFound(Box<ToolsOzoneModerationDefsRecordViewNotFound>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventViewDetail {
  pub id: i64,
  pub event: ToolsOzoneModerationDefsModEventViewDetailEventUnion,
  pub subject: ToolsOzoneModerationDefsModEventViewDetailSubjectUnion,
  pub subject_blobs: Vec<ToolsOzoneModerationDefsBlobView>,
  pub created_by: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsSubjectStatusViewSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsSubjectStatusView {
  pub id: i64,
  pub subject: ToolsOzoneModerationDefsSubjectStatusViewSubjectUnion,
  pub subject_blob_cids: Option<Vec<String>>,
  pub subject_repo_handle: Option<String>,
  /// Timestamp referencing when the last update was made to the moderation status of the subject,
  pub updated_at: chrono::DateTime<chrono::Utc>,
  /// Timestamp referencing the first moderation status impacting event was emitted on the subject,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub review_state: ToolsOzoneModerationDefsSubjectReviewState,
  /// Sticky comment on the subject.,
  pub comment: Option<String>,
  pub mute_until: Option<chrono::DateTime<chrono::Utc>>,
  pub mute_reporting_until: Option<chrono::DateTime<chrono::Utc>>,
  pub last_reviewed_by: Option<String>,
  pub last_reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
  pub last_reported_at: Option<chrono::DateTime<chrono::Utc>>,
  /// Timestamp referencing when the author of the subject appealed a moderation action,
  pub last_appealed_at: Option<chrono::DateTime<chrono::Utc>>,
  pub takendown: Option<bool>,
  /// True indicates that the a previously taken moderator action was appealed against, by the author of the content. False indicates last appeal was resolved by moderators.,
  pub appealed: Option<bool>,
  pub suspend_until: Option<chrono::DateTime<chrono::Utc>>,
  pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolsOzoneModerationDefsSubjectReviewState(pub String);

/// Take down a subject permanently or temporarily
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventTakedown {
  pub comment: Option<String>,
  /// Indicates how long the takedown should be in effect before automatically expiring.,
  pub duration_in_hours: Option<i64>,
  /// If true, all other reports on content authored by this account will be resolved (acknowledged).,
  pub acknowledge_account_subjects: Option<bool>,
}

/// Revert take down action on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventReverseTakedown {
  /// Describe reasoning behind the reversal.,
  pub comment: Option<String>,
}

/// Resolve appeal on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventResolveAppeal {
  /// Describe resolution.,
  pub comment: Option<String>,
}

/// Add a comment to a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventComment {
  pub comment: String,
  /// Make the comment persistent on the subject,
  pub sticky: Option<bool>,
}

/// Report a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventReport {
  pub comment: Option<String>,
  /// Set to true if the reporter was muted from reporting at the time of the event. These reports won't impact the reviewState of the subject.,
  pub is_reporter_muted: Option<bool>,
  pub report_type: ComAtprotoModerationDefsReasonType,
}

/// Apply/Negate labels on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventLabel {
  pub comment: Option<String>,
  pub create_label_vals: Vec<String>,
  pub negate_label_vals: Vec<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventAcknowledge {
  pub comment: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventEscalate {
  pub comment: Option<String>,
}

/// Mute incoming reports on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventMute {
  pub comment: Option<String>,
  /// Indicates how long the subject should remain muted.,
  pub duration_in_hours: i64,
}

/// Unmute action on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventUnmute {
  /// Describe reasoning behind the reversal.,
  pub comment: Option<String>,
}

/// Mute incoming reports from an account
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventMuteReporter {
  pub comment: Option<String>,
  /// Indicates how long the account should remain muted.,
  pub duration_in_hours: i64,
}

/// Unmute incoming reports from an account
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventUnmuteReporter {
  /// Describe reasoning behind the reversal.,
  pub comment: Option<String>,
}

/// Keep a log of outgoing email to a user
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventEmail {
  /// The subject line of the email sent to the user.,
  pub subject_line: String,
  /// The content of the email sent to the user.,
  pub content: Option<String>,
  /// Additional comment about the outgoing comm.,
  pub comment: Option<String>,
}

/// Divert a record's blobs to a 3rd party service for further scanning/tagging
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventDivert {
  pub comment: Option<String>,
}

/// Add/Remove a tag on a subject
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModEventTag {
  /// Tags to be added to the subject. If already exists, won't be duplicated.,
  pub add: Vec<String>,
  /// Tags to be removed to the subject. Ignores a tag If it doesn't exist, won't be duplicated.,
  pub remove: Vec<String>,
  /// Additional comment about added/removed tags.,
  pub comment: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRepoView {
  pub did: String,
  pub handle: String,
  pub email: Option<String>,
  pub related_records: Vec<serde_json::Value>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub moderation: ToolsOzoneModerationDefsModeration,
  pub invited_by: Option<ComAtprotoServerDefsInviteCode>,
  pub invites_disabled: Option<bool>,
  pub invite_note: Option<String>,
  pub deactivated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRepoViewDetail {
  pub did: String,
  pub handle: String,
  pub email: Option<String>,
  pub related_records: Vec<serde_json::Value>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub moderation: ToolsOzoneModerationDefsModerationDetail,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub invited_by: Option<ComAtprotoServerDefsInviteCode>,
  pub invites: Option<Vec<ComAtprotoServerDefsInviteCode>>,
  pub invites_disabled: Option<bool>,
  pub invite_note: Option<String>,
  pub email_confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
  pub deactivated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRepoViewNotFound {
  pub did: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRecordView {
  pub uri: String,
  pub cid: String,
  pub value: serde_json::Value,
  pub blob_cids: Vec<String>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub moderation: ToolsOzoneModerationDefsModeration,
  pub repo: ToolsOzoneModerationDefsRepoView,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRecordViewDetail {
  pub uri: String,
  pub cid: String,
  pub value: serde_json::Value,
  pub blobs: Vec<ToolsOzoneModerationDefsBlobView>,
  pub labels: Option<Vec<ComAtprotoLabelDefsLabel>>,
  pub indexed_at: chrono::DateTime<chrono::Utc>,
  pub moderation: ToolsOzoneModerationDefsModerationDetail,
  pub repo: ToolsOzoneModerationDefsRepoView,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsRecordViewNotFound {
  pub uri: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModeration {
  pub subject_status: Option<ToolsOzoneModerationDefsSubjectStatusView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsModerationDetail {
  pub subject_status: Option<ToolsOzoneModerationDefsSubjectStatusView>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationDefsBlobViewDetailsUnion {
  #[serde(rename = "tools.ozone.moderation.defs#imageDetails")]
  ToolsOzoneModerationDefsImageDetails(Box<ToolsOzoneModerationDefsImageDetails>),
  #[serde(rename = "tools.ozone.moderation.defs#videoDetails")]
  ToolsOzoneModerationDefsVideoDetails(Box<ToolsOzoneModerationDefsVideoDetails>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsBlobView {
  pub cid: String,
  pub mime_type: String,
  pub size: i64,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub details: Option<ToolsOzoneModerationDefsBlobViewDetailsUnion>,
  pub moderation: Option<ToolsOzoneModerationDefsModeration>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsImageDetails {
  pub width: i64,
  pub height: i64,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationDefsVideoDetails {
  pub width: i64,
  pub height: i64,
  pub length: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationEmitEventInputEventUnion {
  #[serde(rename = "tools.ozone.moderation.defs#modEventTakedown")]
  ToolsOzoneModerationDefsModEventTakedown(Box<ToolsOzoneModerationDefsModEventTakedown>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventAcknowledge")]
  ToolsOzoneModerationDefsModEventAcknowledge(Box<ToolsOzoneModerationDefsModEventAcknowledge>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEscalate")]
  ToolsOzoneModerationDefsModEventEscalate(Box<ToolsOzoneModerationDefsModEventEscalate>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventComment")]
  ToolsOzoneModerationDefsModEventComment(Box<ToolsOzoneModerationDefsModEventComment>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventLabel")]
  ToolsOzoneModerationDefsModEventLabel(Box<ToolsOzoneModerationDefsModEventLabel>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReport")]
  ToolsOzoneModerationDefsModEventReport(Box<ToolsOzoneModerationDefsModEventReport>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMute")]
  ToolsOzoneModerationDefsModEventMute(Box<ToolsOzoneModerationDefsModEventMute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmute")]
  ToolsOzoneModerationDefsModEventUnmute(Box<ToolsOzoneModerationDefsModEventUnmute>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventMuteReporter")]
  ToolsOzoneModerationDefsModEventMuteReporter(Box<ToolsOzoneModerationDefsModEventMuteReporter>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventUnmuteReporter")]
  ToolsOzoneModerationDefsModEventUnmuteReporter(
    Box<ToolsOzoneModerationDefsModEventUnmuteReporter>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventReverseTakedown")]
  ToolsOzoneModerationDefsModEventReverseTakedown(
    Box<ToolsOzoneModerationDefsModEventReverseTakedown>,
  ),
  #[serde(rename = "tools.ozone.moderation.defs#modEventResolveAppeal")]
  ToolsOzoneModerationDefsModEventResolveAppeal(Box<ToolsOzoneModerationDefsModEventResolveAppeal>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventEmail")]
  ToolsOzoneModerationDefsModEventEmail(Box<ToolsOzoneModerationDefsModEventEmail>),
  #[serde(rename = "tools.ozone.moderation.defs#modEventTag")]
  ToolsOzoneModerationDefsModEventTag(Box<ToolsOzoneModerationDefsModEventTag>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "$type")]
pub enum ToolsOzoneModerationEmitEventInputSubjectUnion {
  #[serde(rename = "com.atproto.admin.defs#repoRef")]
  ComAtprotoAdminDefsRepoRef(Box<ComAtprotoAdminDefsRepoRef>),
  #[serde(rename = "com.atproto.repo.strongRef")]
  ComAtprotoRepoStrongRef(Box<ComAtprotoRepoStrongRef>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationEmitEventInput {
  pub event: ToolsOzoneModerationEmitEventInputEventUnion,
  pub subject: ToolsOzoneModerationEmitEventInputSubjectUnion,
  pub subject_blob_cids: Option<Vec<String>>,
  pub created_by: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationQueryEventsOutput {
  pub cursor: Option<String>,
  pub events: Vec<ToolsOzoneModerationDefsModEventView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationQueryStatusesOutput {
  pub cursor: Option<String>,
  pub subject_statuses: Vec<ToolsOzoneModerationDefsSubjectStatusView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneModerationSearchReposOutput {
  pub cursor: Option<String>,
  pub repos: Vec<ToolsOzoneModerationDefsRepoView>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneServerGetConfigOutput {
  pub appview: Option<ToolsOzoneServerGetConfigServiceConfig>,
  pub pds: Option<ToolsOzoneServerGetConfigServiceConfig>,
  pub blob_divert: Option<ToolsOzoneServerGetConfigServiceConfig>,
  pub chat: Option<ToolsOzoneServerGetConfigServiceConfig>,
  pub viewer: Option<ToolsOzoneServerGetConfigViewerConfig>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneServerGetConfigServiceConfig {
  pub url: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneServerGetConfigViewerConfig {
  pub role: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneTeamAddMemberInput {
  pub did: String,
  pub role: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneTeamDefsMember {
  pub did: String,
  pub disabled: Option<bool>,
  pub profile: Option<AppBskyActorDefsProfileViewDetailed>,
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
  pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
  pub last_updated_by: Option<String>,
  pub role: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneTeamDeleteMemberInput {
  pub did: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneTeamListMembersOutput {
  pub cursor: Option<String>,
  pub members: Vec<ToolsOzoneTeamDefsMember>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsOzoneTeamUpdateMemberInput {
  pub did: String,
  pub disabled: Option<bool>,
  pub role: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Atproto {
  pub client: reqwest::Client,
  pub host: String,
  pub firehose: String,
  pub access_jwt: Option<String>,
  pub refresh_jwt: Option<String>,
}

impl Default for Atproto {
  fn default() -> Self {
    Self::new(None, None)
  }
}

impl Atproto {
  pub fn new(host: Option<&str>, firehose: Option<&str>) -> Self {
    Self {
      client: reqwest::Client::new(),
      host: host
        .map(|h| h.to_string())
        .unwrap_or_else(|| String::from("bsky.social")),
      firehose: firehose
        .map(|h| h.to_string())
        .unwrap_or_else(|| String::from("bsky.network")),
      access_jwt: None,
      refresh_jwt: None,
    }
  }

  pub async fn login(&mut self, id: &str, pw: &str) -> Result<ComAtprotoServerCreateSessionOutput> {
    let output = self
      .com_atproto_server_create_session(ComAtprotoServerCreateSessionInput {
        identifier: id.to_string(),
        password: pw.to_string(),
        auth_factor_token: None,
      })
      .await?;
    self.access_jwt = Some(output.access_jwt.clone());
    self.refresh_jwt = Some(output.refresh_jwt.clone());
    Ok(output)
  }

  /// Get private preferences attached to the current account. Expected use is synchronization between multiple devices, and import/export during account migration. Requires auth.
  pub async fn app_bsky_actor_get_preferences(&self) -> Result<AppBskyActorGetPreferencesOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/app.bsky.actor.getPreferences",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get detailed profile view of an actor. Does not require auth, but contains relevant metadata with auth.
  ///
  /// # Arguments
  ///
  /// * `actor` - Handle or DID of account to fetch profile of.
  pub async fn app_bsky_actor_get_profile(
    &self,
    actor: &str,
  ) -> Result<AppBskyActorDefsProfileViewDetailed> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.actor.getProfile",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get detailed profile views of multiple actors.
  ///
  /// # Arguments
  ///
  /// * `actors`
  pub async fn app_bsky_actor_get_profiles(
    &self,
    actors: &[&str],
  ) -> Result<AppBskyActorGetProfilesOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut actors
        .iter()
        .map(|i| (String::from("actors"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.actor.getProfiles",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of suggested actors. Expected use is discovery of accounts to follow during new account onboarding.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_actor_get_suggestions(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyActorGetSuggestionsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.actor.getSuggestions",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Set the private preferences attached to the account.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_actor_put_preferences(
    &self,
    body: AppBskyActorPutPreferencesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.actor.putPreferences",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Find actors (profiles) matching search criteria. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `term` - DEPRECATED: use 'q' instead.
  /// * `q` - Search query string. Syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended.
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_actor_search_actors(
    &self,
    term: Option<&str>,
    q: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyActorSearchActorsOutput> {
    let mut query_ = Vec::new();
    if let Some(term) = &term {
      query_.push((String::from("term"), term.to_string()));
    }
    if let Some(q) = &q {
      query_.push((String::from("q"), q.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.actor.searchActors",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Find actor suggestions for a prefix search term. Expected use is for auto-completion during text field entry. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `term` - DEPRECATED: use 'q' instead.
  /// * `q` - Search query prefix; not a full query string.
  /// * `limit`
  pub async fn app_bsky_actor_search_actors_typeahead(
    &self,
    term: Option<&str>,
    q: Option<&str>,
    limit: Option<i64>,
  ) -> Result<AppBskyActorSearchActorsTypeaheadOutput> {
    let mut query_ = Vec::new();
    if let Some(term) = &term {
      query_.push((String::from("term"), term.to_string()));
    }
    if let Some(q) = &q {
      query_.push((String::from("q"), q.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.actor.searchActorsTypeahead",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get information about a feed generator, including policies and offered feed URIs. Does not require auth; implemented by Feed Generator services (not App View).
  pub async fn app_bsky_feed_describe_feed_generator(
    &self,
  ) -> Result<AppBskyFeedDescribeFeedGeneratorOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/app.bsky.feed.describeFeedGenerator",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of feeds (feed generator records) created by the actor (in the actor's repo).
  ///
  /// # Arguments
  ///
  /// * `actor`
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_feed_get_actor_feeds(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetActorFeedsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getActorFeeds",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of posts liked by an actor. Requires auth, actor must be the requesting account.
  ///
  /// # Arguments
  ///
  /// * `actor`
  /// * `limit`
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `BlockedActor`
  /// * `BlockedByActor`
  pub async fn app_bsky_feed_get_actor_likes(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetActorLikesOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getActorLikes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a view of an actor's 'author feed' (post and reposts by the author). Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `actor`
  /// * `limit`
  /// * `cursor`
  /// * `filter` - Combinations of post/repost types to include in response.
  ///
  /// # Errors
  ///
  /// * `BlockedActor`
  /// * `BlockedByActor`
  pub async fn app_bsky_feed_get_author_feed(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
    filter: Option<&str>,
  ) -> Result<AppBskyFeedGetAuthorFeedOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(filter) = &filter {
      query_.push((String::from("filter"), filter.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getAuthorFeed",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a hydrated feed from an actor's selected feed generator. Implemented by App View.
  ///
  /// # Arguments
  ///
  /// * `feed`
  /// * `limit`
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `UnknownFeed`
  pub async fn app_bsky_feed_get_feed(
    &self,
    feed: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetFeedOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("feed"), feed.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!("https://{}/xrpc/app.bsky.feed.getFeed", self.host))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get information about a feed generator. Implemented by AppView.
  ///
  /// # Arguments
  ///
  /// * `feed` - AT-URI of the feed generator record.
  pub async fn app_bsky_feed_get_feed_generator(
    &self,
    feed: &str,
  ) -> Result<AppBskyFeedGetFeedGeneratorOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("feed"), feed.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getFeedGenerator",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get information about a list of feed generators.
  ///
  /// # Arguments
  ///
  /// * `feeds`
  pub async fn app_bsky_feed_get_feed_generators(
    &self,
    feeds: &[&str],
  ) -> Result<AppBskyFeedGetFeedGeneratorsOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut feeds
        .iter()
        .map(|i| (String::from("feeds"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getFeedGenerators",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a skeleton of a feed provided by a feed generator. Auth is optional, depending on provider requirements, and provides the DID of the requester. Implemented by Feed Generator Service.
  ///
  /// # Arguments
  ///
  /// * `feed` - Reference to feed generator record describing the specific feed being requested.
  /// * `limit`
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `UnknownFeed`
  pub async fn app_bsky_feed_get_feed_skeleton(
    &self,
    feed: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetFeedSkeletonOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("feed"), feed.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getFeedSkeleton",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get like records which reference a subject (by AT-URI and CID).
  ///
  /// # Arguments
  ///
  /// * `uri` - AT-URI of the subject (eg, a post record).
  /// * `cid` - CID of the subject record (aka, specific version of record), to filter likes.
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_feed_get_likes(
    &self,
    uri: &str,
    cid: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetLikesOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("uri"), uri.to_string()));
    if let Some(cid) = &cid {
      query_.push((String::from("cid"), cid.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getLikes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a feed of recent posts from a list (posts and reposts from any actors on the list). Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `list` - Reference (AT-URI) to the list record.
  /// * `limit`
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `UnknownList`
  pub async fn app_bsky_feed_get_list_feed(
    &self,
    list: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetListFeedOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("list"), list.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getListFeed",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get posts in a thread. Does not require auth, but additional metadata and filtering will be applied for authed requests.
  ///
  /// # Arguments
  ///
  /// * `uri` - Reference (AT-URI) to post record.
  /// * `depth` - How many levels of reply depth should be included in response.
  /// * `parent_height` - How many levels of parent (and grandparent, etc) post to include.
  ///
  /// # Errors
  ///
  /// * `NotFound`
  pub async fn app_bsky_feed_get_post_thread(
    &self,
    uri: &str,
    depth: Option<i64>,
    parent_height: Option<i64>,
  ) -> Result<AppBskyFeedGetPostThreadOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("uri"), uri.to_string()));
    if let Some(depth) = &depth {
      query_.push((String::from("depth"), depth.to_string()));
    }
    if let Some(parent_height) = &parent_height {
      query_.push((String::from("parent_height"), parent_height.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getPostThread",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Gets post views for a specified list of posts (by AT-URI). This is sometimes referred to as 'hydrating' a 'feed skeleton'.
  ///
  /// # Arguments
  ///
  /// * `uris` - List of post AT-URIs to return hydrated views for.
  pub async fn app_bsky_feed_get_posts(&self, uris: &[&str]) -> Result<AppBskyFeedGetPostsOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut uris
        .iter()
        .map(|i| (String::from("uris"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getPosts",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of quotes for a given post.
  ///
  /// # Arguments
  ///
  /// * `uri` - Reference (AT-URI) of post record
  /// * `cid` - If supplied, filters to quotes of specific version (by CID) of the post record.
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_feed_get_quotes(
    &self,
    uri: &str,
    cid: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetQuotesOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("uri"), uri.to_string()));
    if let Some(cid) = &cid {
      query_.push((String::from("cid"), cid.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getQuotes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of reposts for a given post.
  ///
  /// # Arguments
  ///
  /// * `uri` - Reference (AT-URI) of post record
  /// * `cid` - If supplied, filters to reposts of specific version (by CID) of the post record.
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_feed_get_reposted_by(
    &self,
    uri: &str,
    cid: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetRepostedByOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("uri"), uri.to_string()));
    if let Some(cid) = &cid {
      query_.push((String::from("cid"), cid.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getRepostedBy",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of suggested feeds (feed generators) for the requesting account.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_feed_get_suggested_feeds(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetSuggestedFeedsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getSuggestedFeeds",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a view of the requesting account's home timeline. This is expected to be some form of reverse-chronological feed.
  ///
  /// # Arguments
  ///
  /// * `algorithm` - Variant 'algorithm' for timeline. Implementation-specific. NOTE: most feed flexibility has been moved to feed generator mechanism.
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_feed_get_timeline(
    &self,
    algorithm: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedGetTimelineOutput> {
    let mut query_ = Vec::new();
    if let Some(algorithm) = &algorithm {
      query_.push((String::from("algorithm"), algorithm.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.getTimeline",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Find posts matching search criteria, returning views of those posts.
  ///
  /// # Arguments
  ///
  /// * `q` - Search query string; syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended.
  /// * `sort` - Specifies the ranking order of results.
  /// * `since` - Filter results for posts after the indicated datetime (inclusive). Expected to use 'sortAt' timestamp, which may not match 'createdAt'. Can be a datetime, or just an ISO date (YYYY-MM-DD).
  /// * `until` - Filter results for posts before the indicated datetime (not inclusive). Expected to use 'sortAt' timestamp, which may not match 'createdAt'. Can be a datetime, or just an ISO date (YYY-MM-DD).
  /// * `mentions` - Filter to posts which mention the given account. Handles are resolved to DID before query-time. Only matches rich-text facet mentions.
  /// * `author` - Filter to posts by the given account. Handles are resolved to DID before query-time.
  /// * `lang` - Filter to posts in the given language. Expected to be based on post language field, though server may override language detection.
  /// * `domain` - Filter to posts with URLs (facet links or embeds) linking to the given domain (hostname). Server may apply hostname normalization.
  /// * `url` - Filter to posts with links (facet links or embeds) pointing to this URL. Server may apply URL normalization or fuzzy matching.
  /// * `tag` - Filter to posts with the given tag (hashtag), based on rich-text facet or tag field. Do not include the hash (#) prefix. Multiple tags can be specified, with 'AND' matching.
  /// * `limit`
  /// * `cursor` - Optional pagination mechanism; may not necessarily allow scrolling through entire result set.
  ///
  /// # Errors
  ///
  /// * `BadQueryString`
  pub async fn app_bsky_feed_search_posts(
    &self,
    q: &str,
    sort: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
    mentions: Option<&str>,
    author: Option<&str>,
    lang: Option<&str>,
    domain: Option<&str>,
    url: Option<&str>,
    tag: Option<&[&str]>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyFeedSearchPostsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("q"), q.to_string()));
    if let Some(sort) = &sort {
      query_.push((String::from("sort"), sort.to_string()));
    }
    if let Some(since) = &since {
      query_.push((String::from("since"), since.to_string()));
    }
    if let Some(until) = &until {
      query_.push((String::from("until"), until.to_string()));
    }
    if let Some(mentions) = &mentions {
      query_.push((String::from("mentions"), mentions.to_string()));
    }
    if let Some(author) = &author {
      query_.push((String::from("author"), author.to_string()));
    }
    if let Some(lang) = &lang {
      query_.push((String::from("lang"), lang.to_string()));
    }
    if let Some(domain) = &domain {
      query_.push((String::from("domain"), domain.to_string()));
    }
    if let Some(url) = &url {
      query_.push((String::from("url"), url.to_string()));
    }
    if let Some(tag) = &tag {
      query_.append(
        &mut tag
          .iter()
          .map(|i| (String::from("tag"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.feed.searchPosts",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Send information about interactions with feed items back to the feed generator that served them.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_feed_send_interactions(
    &self,
    body: AppBskyFeedSendInteractionsInput,
  ) -> Result<serde_json::Value> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.feed.sendInteractions",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of starter packs created by the actor.
  ///
  /// # Arguments
  ///
  /// * `actor`
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_graph_get_actor_starter_packs(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetActorStarterPacksOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getActorStarterPacks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates which accounts the requesting account is currently blocking. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_graph_get_blocks(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetBlocksOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getBlocks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates accounts which follow a specified account (actor).
  ///
  /// # Arguments
  ///
  /// * `actor`
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_graph_get_followers(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetFollowersOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getFollowers",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates accounts which a specified account (actor) follows.
  ///
  /// # Arguments
  ///
  /// * `actor`
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_graph_get_follows(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetFollowsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getFollows",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates accounts which follow a specified account (actor) and are followed by the viewer.
  ///
  /// # Arguments
  ///
  /// * `actor`
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_graph_get_known_followers(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetKnownFollowersOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getKnownFollowers",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Gets a 'view' (with additional context) of a specified list.
  ///
  /// # Arguments
  ///
  /// * `list` - Reference (AT-URI) of the list record to hydrate.
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_graph_get_list(
    &self,
    list: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetListOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("list"), list.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getList",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get mod lists that the requesting account (actor) is blocking. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_graph_get_list_blocks(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetListBlocksOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getListBlocks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates mod lists that the requesting account (actor) currently has muted. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_graph_get_list_mutes(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetListMutesOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getListMutes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates the lists created by a specified account (actor).
  ///
  /// # Arguments
  ///
  /// * `actor` - The account (actor) to enumerate lists from.
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_graph_get_lists(
    &self,
    actor: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetListsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getLists",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates accounts that the requesting account (actor) currently has muted. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  pub async fn app_bsky_graph_get_mutes(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyGraphGetMutesOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getMutes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates public relationships between one account, and a list of other accounts. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `actor` - Primary account requesting relationships for.
  /// * `others` - List of 'other' accounts to be related back to the primary.
  ///
  /// # Errors
  ///
  /// * `ActorNotFound` - the primary actor at-identifier could not be resolved
  pub async fn app_bsky_graph_get_relationships(
    &self,
    actor: &str,
    others: Option<&[&str]>,
  ) -> Result<AppBskyGraphGetRelationshipsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    if let Some(others) = &others {
      query_.append(
        &mut others
          .iter()
          .map(|i| (String::from("others"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getRelationships",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Gets a view of a starter pack.
  ///
  /// # Arguments
  ///
  /// * `starter_pack` - Reference (AT-URI) of the starter pack record.
  pub async fn app_bsky_graph_get_starter_pack(
    &self,
    starter_pack: &str,
  ) -> Result<AppBskyGraphGetStarterPackOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("starter_pack"), starter_pack.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getStarterPack",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get views for a list of starter packs.
  ///
  /// # Arguments
  ///
  /// * `uris`
  pub async fn app_bsky_graph_get_starter_packs(
    &self,
    uris: &[&str],
  ) -> Result<AppBskyGraphGetStarterPacksOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut uris
        .iter()
        .map(|i| (String::from("uris"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getStarterPacks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates follows similar to a given account (actor). Expected use is to recommend additional accounts immediately after following one account.
  ///
  /// # Arguments
  ///
  /// * `actor`
  pub async fn app_bsky_graph_get_suggested_follows_by_actor(
    &self,
    actor: &str,
  ) -> Result<AppBskyGraphGetSuggestedFollowsByActorOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.graph.getSuggestedFollowsByActor",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Creates a mute relationship for the specified account. Mutes are private in Bluesky. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_mute_actor(&self, body: AppBskyGraphMuteActorInput) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.muteActor",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Creates a mute relationship for the specified list of accounts. Mutes are private in Bluesky. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_mute_actor_list(
    &self,
    body: AppBskyGraphMuteActorListInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.muteActorList",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Mutes a thread preventing notifications from the thread and any of its children. Mutes are private in Bluesky. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_mute_thread(&self, body: AppBskyGraphMuteThreadInput) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.muteThread",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Unmutes the specified account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_unmute_actor(
    &self,
    body: AppBskyGraphUnmuteActorInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.unmuteActor",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Unmutes the specified list of accounts. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_unmute_actor_list(
    &self,
    body: AppBskyGraphUnmuteActorListInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.unmuteActorList",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Unmutes the specified thread. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_graph_unmute_thread(
    &self,
    body: AppBskyGraphUnmuteThreadInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.graph.unmuteThread",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Get information about a list of labeler services.
  ///
  /// # Arguments
  ///
  /// * `dids`
  /// * `detailed`
  pub async fn app_bsky_labeler_get_services(
    &self,
    dids: &[&str],
    detailed: Option<bool>,
  ) -> Result<AppBskyLabelerGetServicesOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut dids
        .iter()
        .map(|i| (String::from("dids"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    if let Some(detailed) = &detailed {
      query_.push((String::from("detailed"), detailed.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.labeler.getServices",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Count the number of unread notifications for the requesting account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `priority`
  /// * `seen_at`
  pub async fn app_bsky_notification_get_unread_count(
    &self,
    priority: Option<bool>,
    seen_at: Option<&chrono::DateTime<chrono::Utc>>,
  ) -> Result<AppBskyNotificationGetUnreadCountOutput> {
    let mut query_ = Vec::new();
    if let Some(priority) = &priority {
      query_.push((String::from("priority"), priority.to_string()));
    }
    if let Some(seen_at) = &seen_at {
      query_.push((String::from("seen_at"), seen_at.to_rfc3339()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.notification.getUnreadCount",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerate notifications for the requesting account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `priority`
  /// * `cursor`
  /// * `seen_at`
  pub async fn app_bsky_notification_list_notifications(
    &self,
    limit: Option<i64>,
    priority: Option<bool>,
    cursor: Option<&str>,
    seen_at: Option<&chrono::DateTime<chrono::Utc>>,
  ) -> Result<AppBskyNotificationListNotificationsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(priority) = &priority {
      query_.push((String::from("priority"), priority.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(seen_at) = &seen_at {
      query_.push((String::from("seen_at"), seen_at.to_rfc3339()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.notification.listNotifications",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Set notification-related preferences for an account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_notification_put_preferences(
    &self,
    body: AppBskyNotificationPutPreferencesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.notification.putPreferences",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Register to receive push notifications, via a specified service, for the requesting account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_notification_register_push(
    &self,
    body: AppBskyNotificationRegisterPushInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.notification.registerPush",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Notify server that the requesting account has seen notifications. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_notification_update_seen(
    &self,
    body: AppBskyNotificationUpdateSeenInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.notification.updateSeen",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// An unspecced view of globally popular feed generators.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  /// * `query`
  pub async fn app_bsky_unspecced_get_popular_feed_generators(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
    query: Option<&str>,
  ) -> Result<AppBskyUnspeccedGetPopularFeedGeneratorsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(query) = &query {
      query_.push((String::from("query"), query.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.unspecced.getPopularFeedGenerators",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a skeleton of suggested actors. Intended to be called and then hydrated through app.bsky.actor.getSuggestions
  ///
  /// # Arguments
  ///
  /// * `viewer` - DID of the account making the request (not included for public/unauthenticated queries). Used to boost followed accounts in ranking.
  /// * `limit`
  /// * `cursor`
  /// * `relative_to_did` - DID of the account to get suggestions relative to. If not provided, suggestions will be based on the viewer.
  pub async fn app_bsky_unspecced_get_suggestions_skeleton(
    &self,
    viewer: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
    relative_to_did: Option<&str>,
  ) -> Result<AppBskyUnspeccedGetSuggestionsSkeletonOutput> {
    let mut query_ = Vec::new();
    if let Some(viewer) = &viewer {
      query_.push((String::from("viewer"), viewer.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(relative_to_did) = &relative_to_did {
      query_.push((String::from("relative_to_did"), relative_to_did.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.unspecced.getSuggestionsSkeleton",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a list of suggestions (feeds and users) tagged with categories
  pub async fn app_bsky_unspecced_get_tagged_suggestions(
    &self,
  ) -> Result<AppBskyUnspeccedGetTaggedSuggestionsOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/app.bsky.unspecced.getTaggedSuggestions",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Backend Actors (profile) search, returns only skeleton.
  ///
  /// # Arguments
  ///
  /// * `q` - Search query string; syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended. For typeahead search, only simple term match is supported, not full syntax.
  /// * `viewer` - DID of the account making the request (not included for public/unauthenticated queries). Used to boost followed accounts in ranking.
  /// * `typeahead` - If true, acts as fast/simple 'typeahead' query.
  /// * `limit`
  /// * `cursor` - Optional pagination mechanism; may not necessarily allow scrolling through entire result set.
  ///
  /// # Errors
  ///
  /// * `BadQueryString`
  pub async fn app_bsky_unspecced_search_actors_skeleton(
    &self,
    q: &str,
    viewer: Option<&str>,
    typeahead: Option<bool>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyUnspeccedSearchActorsSkeletonOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("q"), q.to_string()));
    if let Some(viewer) = &viewer {
      query_.push((String::from("viewer"), viewer.to_string()));
    }
    if let Some(typeahead) = &typeahead {
      query_.push((String::from("typeahead"), typeahead.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.unspecced.searchActorsSkeleton",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Backend Posts search, returns only skeleton
  ///
  /// # Arguments
  ///
  /// * `q` - Search query string; syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended.
  /// * `sort` - Specifies the ranking order of results.
  /// * `since` - Filter results for posts after the indicated datetime (inclusive). Expected to use 'sortAt' timestamp, which may not match 'createdAt'. Can be a datetime, or just an ISO date (YYYY-MM-DD).
  /// * `until` - Filter results for posts before the indicated datetime (not inclusive). Expected to use 'sortAt' timestamp, which may not match 'createdAt'. Can be a datetime, or just an ISO date (YYY-MM-DD).
  /// * `mentions` - Filter to posts which mention the given account. Handles are resolved to DID before query-time. Only matches rich-text facet mentions.
  /// * `author` - Filter to posts by the given account. Handles are resolved to DID before query-time.
  /// * `lang` - Filter to posts in the given language. Expected to be based on post language field, though server may override language detection.
  /// * `domain` - Filter to posts with URLs (facet links or embeds) linking to the given domain (hostname). Server may apply hostname normalization.
  /// * `url` - Filter to posts with links (facet links or embeds) pointing to this URL. Server may apply URL normalization or fuzzy matching.
  /// * `tag` - Filter to posts with the given tag (hashtag), based on rich-text facet or tag field. Do not include the hash (#) prefix. Multiple tags can be specified, with 'AND' matching.
  /// * `viewer` - DID of the account making the request (not included for public/unauthenticated queries). Used for 'from:me' queries.
  /// * `limit`
  /// * `cursor` - Optional pagination mechanism; may not necessarily allow scrolling through entire result set.
  ///
  /// # Errors
  ///
  /// * `BadQueryString`
  pub async fn app_bsky_unspecced_search_posts_skeleton(
    &self,
    q: &str,
    sort: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
    mentions: Option<&str>,
    author: Option<&str>,
    lang: Option<&str>,
    domain: Option<&str>,
    url: Option<&str>,
    tag: Option<&[&str]>,
    viewer: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<AppBskyUnspeccedSearchPostsSkeletonOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("q"), q.to_string()));
    if let Some(sort) = &sort {
      query_.push((String::from("sort"), sort.to_string()));
    }
    if let Some(since) = &since {
      query_.push((String::from("since"), since.to_string()));
    }
    if let Some(until) = &until {
      query_.push((String::from("until"), until.to_string()));
    }
    if let Some(mentions) = &mentions {
      query_.push((String::from("mentions"), mentions.to_string()));
    }
    if let Some(author) = &author {
      query_.push((String::from("author"), author.to_string()));
    }
    if let Some(lang) = &lang {
      query_.push((String::from("lang"), lang.to_string()));
    }
    if let Some(domain) = &domain {
      query_.push((String::from("domain"), domain.to_string()));
    }
    if let Some(url) = &url {
      query_.push((String::from("url"), url.to_string()));
    }
    if let Some(tag) = &tag {
      query_.append(
        &mut tag
          .iter()
          .map(|i| (String::from("tag"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(viewer) = &viewer {
      query_.push((String::from("viewer"), viewer.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.unspecced.searchPostsSkeleton",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get status details for a video processing job.
  ///
  /// # Arguments
  ///
  /// * `job_id`
  pub async fn app_bsky_video_get_job_status(
    &self,
    job_id: &str,
  ) -> Result<AppBskyVideoGetJobStatusOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("job_id"), job_id.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/app.bsky.video.getJobStatus",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get video upload limits for the authenticated user.
  pub async fn app_bsky_video_get_upload_limits(
    &self,
  ) -> Result<AppBskyVideoGetUploadLimitsOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/app.bsky.video.getUploadLimits",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Upload a video to be processed then stored on the PDS.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn app_bsky_video_upload_video(
    &self,
    body: Vec<u8>,
  ) -> Result<AppBskyVideoUploadVideoOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/app.bsky.video.uploadVideo",
        self.host
      ))
      .header("Content-Type", "video/mp4")
      .body(body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  pub async fn chat_bsky_actor_delete_account(&self) -> Result<serde_json::Value> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/chat.bsky.actor.deleteAccount",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  pub async fn chat_bsky_actor_export_account_data(&self) -> Result<Vec<serde_json::Value>> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/chat.bsky.actor.exportAccountData",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(
      response
        .text()
        .await?
        .split("\n")
        .filter_map(|t| serde_json::from_str(t).ok())
        .collect::<Vec<_>>(),
    )
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_delete_message_for_self(
    &self,
    body: ChatBskyConvoDeleteMessageForSelfInput,
  ) -> Result<ChatBskyConvoDefsDeletedMessageView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.deleteMessageForSelf",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `convo_id`
  pub async fn chat_bsky_convo_get_convo(
    &self,
    convo_id: &str,
  ) -> Result<ChatBskyConvoGetConvoOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("convo_id"), convo_id.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.convo.getConvo",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `members`
  pub async fn chat_bsky_convo_get_convo_for_members(
    &self,
    members: &[&str],
  ) -> Result<ChatBskyConvoGetConvoForMembersOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut members
        .iter()
        .map(|i| (String::from("members"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.convo.getConvoForMembers",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `cursor`
  pub async fn chat_bsky_convo_get_log(
    &self,
    cursor: Option<&str>,
  ) -> Result<ChatBskyConvoGetLogOutput> {
    let mut query_ = Vec::new();
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.convo.getLog",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `convo_id`
  /// * `limit`
  /// * `cursor`
  pub async fn chat_bsky_convo_get_messages(
    &self,
    convo_id: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ChatBskyConvoGetMessagesOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("convo_id"), convo_id.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.convo.getMessages",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_leave_convo(
    &self,
    body: ChatBskyConvoLeaveConvoInput,
  ) -> Result<ChatBskyConvoLeaveConvoOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.leaveConvo",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  pub async fn chat_bsky_convo_list_convos(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ChatBskyConvoListConvosOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.convo.listConvos",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_mute_convo(
    &self,
    body: ChatBskyConvoMuteConvoInput,
  ) -> Result<ChatBskyConvoMuteConvoOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.muteConvo",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_send_message(
    &self,
    body: ChatBskyConvoSendMessageInput,
  ) -> Result<ChatBskyConvoDefsMessageView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.sendMessage",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_send_message_batch(
    &self,
    body: ChatBskyConvoSendMessageBatchInput,
  ) -> Result<ChatBskyConvoSendMessageBatchOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.sendMessageBatch",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_unmute_convo(
    &self,
    body: ChatBskyConvoUnmuteConvoInput,
  ) -> Result<ChatBskyConvoUnmuteConvoOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.unmuteConvo",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_convo_update_read(
    &self,
    body: ChatBskyConvoUpdateReadInput,
  ) -> Result<ChatBskyConvoUpdateReadOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.convo.updateRead",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `actor`
  pub async fn chat_bsky_moderation_get_actor_metadata(
    &self,
    actor: &str,
  ) -> Result<ChatBskyModerationGetActorMetadataOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("actor"), actor.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.moderation.getActorMetadata",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * `convo_id` - Conversation that the message is from. NOTE: this field will eventually be required.
  /// * `message_id`
  /// * `before`
  /// * `after`
  pub async fn chat_bsky_moderation_get_message_context(
    &self,
    convo_id: Option<&str>,
    message_id: &str,
    before: Option<i64>,
    after: Option<i64>,
  ) -> Result<ChatBskyModerationGetMessageContextOutput> {
    let mut query_ = Vec::new();
    if let Some(convo_id) = &convo_id {
      query_.push((String::from("convo_id"), convo_id.to_string()));
    }
    query_.push((String::from("message_id"), message_id.to_string()));
    if let Some(before) = &before {
      query_.push((String::from("before"), before.to_string()));
    }
    if let Some(after) = &after {
      query_.push((String::from("after"), after.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/chat.bsky.moderation.getMessageContext",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// # Arguments
  ///
  /// * body
  pub async fn chat_bsky_moderation_update_actor_access(
    &self,
    body: ChatBskyModerationUpdateActorAccessInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/chat.bsky.moderation.updateActorAccess",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Delete a user account as an administrator.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_delete_account(
    &self,
    body: ComAtprotoAdminDeleteAccountInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.deleteAccount",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Disable an account from receiving new invite codes, but does not invalidate existing codes.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_disable_account_invites(
    &self,
    body: ComAtprotoAdminDisableAccountInvitesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.disableAccountInvites",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Disable some set of codes and/or all codes associated with a set of users.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_disable_invite_codes(
    &self,
    body: ComAtprotoAdminDisableInviteCodesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.disableInviteCodes",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Re-enable an account's ability to receive invite codes.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_enable_account_invites(
    &self,
    body: ComAtprotoAdminEnableAccountInvitesInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.enableAccountInvites",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Get details about an account.
  ///
  /// # Arguments
  ///
  /// * `did`
  pub async fn com_atproto_admin_get_account_info(
    &self,
    did: &str,
  ) -> Result<ComAtprotoAdminDefsAccountView> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.admin.getAccountInfo",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about some accounts.
  ///
  /// # Arguments
  ///
  /// * `dids`
  pub async fn com_atproto_admin_get_account_infos(
    &self,
    dids: &[&str],
  ) -> Result<ComAtprotoAdminGetAccountInfosOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut dids
        .iter()
        .map(|i| (String::from("dids"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.admin.getAccountInfos",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get an admin view of invite codes.
  ///
  /// # Arguments
  ///
  /// * `sort`
  /// * `limit`
  /// * `cursor`
  pub async fn com_atproto_admin_get_invite_codes(
    &self,
    sort: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoAdminGetInviteCodesOutput> {
    let mut query_ = Vec::new();
    if let Some(sort) = &sort {
      query_.push((String::from("sort"), sort.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.admin.getInviteCodes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get the service-specific admin status of a subject (account, record, or blob).
  ///
  /// # Arguments
  ///
  /// * `did`
  /// * `uri`
  /// * `blob`
  pub async fn com_atproto_admin_get_subject_status(
    &self,
    did: Option<&str>,
    uri: Option<&str>,
    blob: Option<&str>,
  ) -> Result<ComAtprotoAdminGetSubjectStatusOutput> {
    let mut query_ = Vec::new();
    if let Some(did) = &did {
      query_.push((String::from("did"), did.to_string()));
    }
    if let Some(uri) = &uri {
      query_.push((String::from("uri"), uri.to_string()));
    }
    if let Some(blob) = &blob {
      query_.push((String::from("blob"), blob.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.admin.getSubjectStatus",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get list of accounts that matches your search query.
  ///
  /// # Arguments
  ///
  /// * `email`
  /// * `cursor`
  /// * `limit`
  pub async fn com_atproto_admin_search_accounts(
    &self,
    email: Option<&str>,
    cursor: Option<&str>,
    limit: Option<i64>,
  ) -> Result<ComAtprotoAdminSearchAccountsOutput> {
    let mut query_ = Vec::new();
    if let Some(email) = &email {
      query_.push((String::from("email"), email.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.admin.searchAccounts",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Send email to a user's account email address.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_send_email(
    &self,
    body: ComAtprotoAdminSendEmailInput,
  ) -> Result<ComAtprotoAdminSendEmailOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.sendEmail",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Administrative action to update an account's email.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_update_account_email(
    &self,
    body: ComAtprotoAdminUpdateAccountEmailInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.updateAccountEmail",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Administrative action to update an account's handle.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_update_account_handle(
    &self,
    body: ComAtprotoAdminUpdateAccountHandleInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.updateAccountHandle",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Update the password for a user account as an administrator.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_update_account_password(
    &self,
    body: ComAtprotoAdminUpdateAccountPasswordInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.updateAccountPassword",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Update the service-specific admin status of a subject (account, record, or blob).
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_admin_update_subject_status(
    &self,
    body: ComAtprotoAdminUpdateSubjectStatusInput,
  ) -> Result<ComAtprotoAdminUpdateSubjectStatusOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.admin.updateSubjectStatus",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Describe the credentials that should be included in the DID doc of an account that is migrating to this service.
  pub async fn com_atproto_identity_get_recommended_did_credentials(
    &self,
  ) -> Result<ComAtprotoIdentityGetRecommendedDidCredentialsOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.identity.getRecommendedDidCredentials",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Request an email with a code to in order to request a signed PLC operation. Requires Auth.
  pub async fn com_atproto_identity_request_plc_operation_signature(&self) -> Result<()> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.identity.requestPlcOperationSignature",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Resolves a handle (domain name) to a DID.
  ///
  /// # Arguments
  ///
  /// * `handle` - The handle to resolve.
  pub async fn com_atproto_identity_resolve_handle(
    &self,
    handle: &str,
  ) -> Result<ComAtprotoIdentityResolveHandleOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("handle"), handle.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.identity.resolveHandle",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Signs a PLC operation to update some value(s) in the requesting DID's document.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_identity_sign_plc_operation(
    &self,
    body: ComAtprotoIdentitySignPlcOperationInput,
  ) -> Result<ComAtprotoIdentitySignPlcOperationOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.identity.signPlcOperation",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Validates a PLC operation to ensure that it doesn't violate a service's constraints or get the identity into a bad state, then submits it to the PLC registry
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_identity_submit_plc_operation(
    &self,
    body: ComAtprotoIdentitySubmitPlcOperationInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.identity.submitPlcOperation",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Updates the current account's handle. Verifies handle validity, and updates did:plc document if necessary. Implemented by PDS, and requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_identity_update_handle(
    &self,
    body: ComAtprotoIdentityUpdateHandleInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.identity.updateHandle",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Find labels relevant to the provided AT-URI patterns. Public endpoint for moderation services, though may return different or additional results with auth.
  ///
  /// # Arguments
  ///
  /// * `uri_patterns` - List of AT URI patterns to match (boolean 'OR'). Each may be a prefix (ending with '*'; will match inclusive of the string leading to '*'), or a full URI.
  /// * `sources` - Optional list of label sources (DIDs) to filter on.
  /// * `limit`
  /// * `cursor`
  pub async fn com_atproto_label_query_labels(
    &self,
    uri_patterns: &[&str],
    sources: Option<&[&str]>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoLabelQueryLabelsOutput> {
    let mut query_ = Vec::new();
    query_.append(
      &mut uri_patterns
        .iter()
        .map(|i| (String::from("uri_patterns"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    if let Some(sources) = &sources {
      query_.append(
        &mut sources
          .iter()
          .map(|i| (String::from("sources"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.label.queryLabels",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Subscribe to stream of labels (and negations). Public endpoint implemented by mod services. Uses same sequencing scheme as repo event stream.
  ///
  /// # Arguments
  ///
  /// * `cursor` - The last known event seq number to backfill from.
  ///
  /// # Messages
  ///
  /// * ComAtprotoLabelSubscribeLabelsLabels
  /// * ComAtprotoLabelSubscribeLabelsInfo
  ///
  /// # Errors
  ///
  /// * `FutureCursor`
  pub async fn com_atproto_label_subscribe_labels(
    &self,
    cursor: Option<i64>,
  ) -> Result<reqwest_websocket::WebSocket> {
    let mut query_ = Vec::new();
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "wss://{}/xrpc/com.atproto.label.subscribeLabels",
        self.firehose
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    Ok(
      reqwest_websocket::RequestBuilderExt::upgrade(request)
        .send()
        .await?
        .into_websocket()
        .await?,
    )
  }

  /// Submit a moderation report regarding an atproto account or record. Implemented by moderation services (with PDS proxying), and requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_moderation_create_report(
    &self,
    body: ComAtprotoModerationCreateReportInput,
  ) -> Result<ComAtprotoModerationCreateReportOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.moderation.createReport",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Apply a batch transaction of repository creates, updates, and deletes. Requires auth, implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `InvalidSwap` - Indicates that the 'swapCommit' parameter did not match current commit.
  pub async fn com_atproto_repo_apply_writes(
    &self,
    body: ComAtprotoRepoApplyWritesInput,
  ) -> Result<ComAtprotoRepoApplyWritesOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.applyWrites",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create a single new repository record. Requires auth, implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `InvalidSwap` - Indicates that 'swapCommit' didn't match current repo commit.
  pub async fn com_atproto_repo_create_record(
    &self,
    body: ComAtprotoRepoCreateRecordInput,
  ) -> Result<ComAtprotoRepoCreateRecordOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.createRecord",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Delete a repository record, or ensure it doesn't exist. Requires auth, implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `InvalidSwap`
  pub async fn com_atproto_repo_delete_record(
    &self,
    body: ComAtprotoRepoDeleteRecordInput,
  ) -> Result<ComAtprotoRepoDeleteRecordOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.deleteRecord",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get information about an account and repository, including the list of collections. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `repo` - The handle or DID of the repo.
  pub async fn com_atproto_repo_describe_repo(
    &self,
    repo: &str,
  ) -> Result<ComAtprotoRepoDescribeRepoOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("repo"), repo.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.repo.describeRepo",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a single record from a repository. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `repo` - The handle or DID of the repo.
  /// * `collection` - The NSID of the record collection.
  /// * `rkey` - The Record Key.
  /// * `cid` - The CID of the version of the record. If not specified, then return the most recent version.
  pub async fn com_atproto_repo_get_record(
    &self,
    repo: &str,
    collection: &str,
    rkey: &str,
    cid: Option<&str>,
  ) -> Result<ComAtprotoRepoGetRecordOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("repo"), repo.to_string()));
    query_.push((String::from("collection"), collection.to_string()));
    query_.push((String::from("rkey"), rkey.to_string()));
    if let Some(cid) = &cid {
      query_.push((String::from("cid"), cid.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.repo.getRecord",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Import a repo in the form of a CAR file. Requires Content-Length HTTP header to be set.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_repo_import_repo(&self, body: Vec<u8>) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.importRepo",
        self.host
      ))
      .header("Content-Type", "application/vnd.ipld.car")
      .body(body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Returns a list of missing blobs for the requesting account. Intended to be used in the account migration flow.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  pub async fn com_atproto_repo_list_missing_blobs(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoRepoListMissingBlobsOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.repo.listMissingBlobs",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// List a range of records in a repository, matching a specific collection. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `repo` - The handle or DID of the repo.
  /// * `collection` - The NSID of the record type.
  /// * `limit` - The number of records to return.
  /// * `cursor`
  /// * `rkey_start` - DEPRECATED: The lowest sort-ordered rkey to start from (exclusive)
  /// * `rkey_end` - DEPRECATED: The highest sort-ordered rkey to stop at (exclusive)
  /// * `reverse` - Flag to reverse the order of the returned records.
  pub async fn com_atproto_repo_list_records(
    &self,
    repo: &str,
    collection: &str,
    limit: Option<i64>,
    cursor: Option<&str>,
    rkey_start: Option<&str>,
    rkey_end: Option<&str>,
    reverse: Option<bool>,
  ) -> Result<ComAtprotoRepoListRecordsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("repo"), repo.to_string()));
    query_.push((String::from("collection"), collection.to_string()));
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    if let Some(rkey_start) = &rkey_start {
      query_.push((String::from("rkey_start"), rkey_start.to_string()));
    }
    if let Some(rkey_end) = &rkey_end {
      query_.push((String::from("rkey_end"), rkey_end.to_string()));
    }
    if let Some(reverse) = &reverse {
      query_.push((String::from("reverse"), reverse.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.repo.listRecords",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Write a repository record, creating or updating it as needed. Requires auth, implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `InvalidSwap`
  pub async fn com_atproto_repo_put_record(
    &self,
    body: ComAtprotoRepoPutRecordInput,
  ) -> Result<ComAtprotoRepoPutRecordOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.putRecord",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Upload a new blob, to be referenced from a repository record. The blob will be deleted if it is not referenced within a time window (eg, minutes). Blob restrictions (mimetype, size, etc) are enforced when the reference is created. Requires auth, implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_repo_upload_blob(
    &self,
    body: Vec<u8>,
    content_type: &str,
  ) -> Result<ComAtprotoRepoUploadBlobOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.repo.uploadBlob",
        self.host
      ))
      .body(body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    request = request.header("Content-Type", content_type);
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Activates a currently deactivated account. Used to finalize account migration after the account's repo is imported and identity is setup.
  pub async fn com_atproto_server_activate_account(&self) -> Result<()> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.activateAccount",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Returns the status of an account, especially as pertaining to import or recovery. Can be called many times over the course of an account migration. Requires auth and can only be called pertaining to oneself.
  pub async fn com_atproto_server_check_account_status(
    &self,
  ) -> Result<ComAtprotoServerCheckAccountStatusOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.server.checkAccountStatus",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Confirm an email using a token from com.atproto.server.requestEmailConfirmation.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `AccountNotFound`
  /// * `ExpiredToken`
  /// * `InvalidToken`
  /// * `InvalidEmail`
  pub async fn com_atproto_server_confirm_email(
    &self,
    body: ComAtprotoServerConfirmEmailInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.confirmEmail",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Create an account. Implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `InvalidHandle`
  /// * `InvalidPassword`
  /// * `InvalidInviteCode`
  /// * `HandleNotAvailable`
  /// * `UnsupportedDomain`
  /// * `UnresolvableDid`
  /// * `IncompatibleDidDoc`
  pub async fn com_atproto_server_create_account(
    &self,
    body: ComAtprotoServerCreateAccountInput,
  ) -> Result<ComAtprotoServerCreateAccountOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.createAccount",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create an App Password.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `AccountTakedown`
  pub async fn com_atproto_server_create_app_password(
    &self,
    body: ComAtprotoServerCreateAppPasswordInput,
  ) -> Result<ComAtprotoServerCreateAppPasswordAppPassword> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.createAppPassword",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create an invite code.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_create_invite_code(
    &self,
    body: ComAtprotoServerCreateInviteCodeInput,
  ) -> Result<ComAtprotoServerCreateInviteCodeOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.createInviteCode",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create invite codes.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_create_invite_codes(
    &self,
    body: ComAtprotoServerCreateInviteCodesInput,
  ) -> Result<ComAtprotoServerCreateInviteCodesOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.createInviteCodes",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Create an authentication session.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `AccountTakedown`
  /// * `AuthFactorTokenRequired`
  pub async fn com_atproto_server_create_session(
    &self,
    body: ComAtprotoServerCreateSessionInput,
  ) -> Result<ComAtprotoServerCreateSessionOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.createSession",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Deactivates a currently active account. Stops serving of repo, and future writes to repo until reactivated. Used to finalize account migration with the old host after the account has been activated on the new host.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_deactivate_account(
    &self,
    body: ComAtprotoServerDeactivateAccountInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.deactivateAccount",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Delete an actor's account with a token and password. Can only be called after requesting a deletion token. Requires auth.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `ExpiredToken`
  /// * `InvalidToken`
  pub async fn com_atproto_server_delete_account(
    &self,
    body: ComAtprotoServerDeleteAccountInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.deleteAccount",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Delete the current session. Requires auth.
  pub async fn com_atproto_server_delete_session(&self) -> Result<()> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.deleteSession",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Describes the server's account creation requirements and capabilities. Implemented by PDS.
  pub async fn com_atproto_server_describe_server(
    &self,
  ) -> Result<ComAtprotoServerDescribeServerOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.server.describeServer",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get all invite codes for the current account. Requires auth.
  ///
  /// # Arguments
  ///
  /// * `include_used`
  /// * `create_available` - Controls whether any new 'earned' but not 'created' invites should be created.
  ///
  /// # Errors
  ///
  /// * `DuplicateCreate`
  pub async fn com_atproto_server_get_account_invite_codes(
    &self,
    include_used: Option<bool>,
    create_available: Option<bool>,
  ) -> Result<ComAtprotoServerGetAccountInviteCodesOutput> {
    let mut query_ = Vec::new();
    if let Some(include_used) = &include_used {
      query_.push((String::from("include_used"), include_used.to_string()));
    }
    if let Some(create_available) = &create_available {
      query_.push((
        String::from("create_available"),
        create_available.to_string(),
      ));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.server.getAccountInviteCodes",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get a signed token on behalf of the requesting DID for the requested service.
  ///
  /// # Arguments
  ///
  /// * `aud` - The DID of the service that the token will be used to authenticate with
  /// * `exp` - The time in Unix Epoch seconds that the JWT expires. Defaults to 60 seconds in the future. The service may enforce certain time bounds on tokens depending on the requested scope.
  /// * `lxm` - Lexicon (XRPC) method to bind the requested token to
  ///
  /// # Errors
  ///
  /// * `BadExpiration` - Indicates that the requested expiration date is not a valid. May be in the past or may be reliant on the requested scopes.
  pub async fn com_atproto_server_get_service_auth(
    &self,
    aud: &str,
    exp: Option<i64>,
    lxm: Option<&str>,
  ) -> Result<ComAtprotoServerGetServiceAuthOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("aud"), aud.to_string()));
    if let Some(exp) = &exp {
      query_.push((String::from("exp"), exp.to_string()));
    }
    if let Some(lxm) = &lxm {
      query_.push((String::from("lxm"), lxm.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.server.getServiceAuth",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get information about the current auth session. Requires auth.
  pub async fn com_atproto_server_get_session(&self) -> Result<ComAtprotoServerGetSessionOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.server.getSession",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// List all App Passwords.
  ///
  /// # Errors
  ///
  /// * `AccountTakedown`
  pub async fn com_atproto_server_list_app_passwords(
    &self,
  ) -> Result<ComAtprotoServerListAppPasswordsOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.server.listAppPasswords",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Refresh an authentication session. Requires auth using the 'refreshJwt' (not the 'accessJwt').
  ///
  /// # Errors
  ///
  /// * `AccountTakedown`
  pub async fn com_atproto_server_refresh_session(
    &self,
  ) -> Result<ComAtprotoServerRefreshSessionOutput> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.refreshSession",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Initiate a user account deletion via email.
  pub async fn com_atproto_server_request_account_delete(&self) -> Result<()> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.requestAccountDelete",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Request an email with a code to confirm ownership of email.
  pub async fn com_atproto_server_request_email_confirmation(&self) -> Result<()> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.requestEmailConfirmation",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Request a token in order to update email.
  pub async fn com_atproto_server_request_email_update(
    &self,
  ) -> Result<ComAtprotoServerRequestEmailUpdateOutput> {
    let mut request = self.client.post(&format!(
      "https://{}/xrpc/com.atproto.server.requestEmailUpdate",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Initiate a user account password reset via email.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_request_password_reset(
    &self,
    body: ComAtprotoServerRequestPasswordResetInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.requestPasswordReset",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Reserve a repo signing key, for use with account creation. Necessary so that a DID PLC update operation can be constructed during an account migraiton. Public and does not require auth; implemented by PDS. NOTE: this endpoint may change when full account migration is implemented.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_reserve_signing_key(
    &self,
    body: ComAtprotoServerReserveSigningKeyInput,
  ) -> Result<ComAtprotoServerReserveSigningKeyOutput> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.reserveSigningKey",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Reset a user account password using a token.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `ExpiredToken`
  /// * `InvalidToken`
  pub async fn com_atproto_server_reset_password(
    &self,
    body: ComAtprotoServerResetPasswordInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.resetPassword",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Revoke an App Password by name.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_server_revoke_app_password(
    &self,
    body: ComAtprotoServerRevokeAppPasswordInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.revokeAppPassword",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Update an account's email.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `ExpiredToken`
  /// * `InvalidToken`
  /// * `TokenRequired`
  pub async fn com_atproto_server_update_email(
    &self,
    body: ComAtprotoServerUpdateEmailInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.server.updateEmail",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Get a blob associated with a given account. Returns the full blob as originally uploaded. Does not require auth; implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * `did` - The DID of the account.
  /// * `cid` - The CID of the blob to fetch
  ///
  /// # Errors
  ///
  /// * `BlobNotFound`
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_get_blob(&self, did: &str, cid: &str) -> Result<Vec<u8>> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    query_.push((String::from("cid"), cid.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getBlob",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(response.bytes().await?.to_vec())
  }

  /// Get data blocks from a given repo, by CID. For example, intermediate MST nodes, or records. Does not require auth; implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * `did` - The DID of the repo.
  /// * `cids`
  ///
  /// # Errors
  ///
  /// * `BlockNotFound`
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_get_blocks(&self, did: &str, cids: &[&str]) -> Result<Vec<u8>> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    query_.append(
      &mut cids
        .iter()
        .map(|i| (String::from("cids"), i.to_string()))
        .collect::<Vec<_>>(),
    );
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getBlocks",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(response.bytes().await?.to_vec())
  }

  /// DEPRECATED - please use com.atproto.sync.getRepo instead
  ///
  /// # Arguments
  ///
  /// * `did` - The DID of the repo.
  pub async fn com_atproto_sync_get_checkout(&self, did: &str) -> Result<Vec<u8>> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getCheckout",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(response.bytes().await?.to_vec())
  }

  /// DEPRECATED - please use com.atproto.sync.getLatestCommit instead
  ///
  /// # Arguments
  ///
  /// * `did` - The DID of the repo.
  ///
  /// # Errors
  ///
  /// * `HeadNotFound`
  pub async fn com_atproto_sync_get_head(&self, did: &str) -> Result<ComAtprotoSyncGetHeadOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getHead",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get the current commit CID & revision of the specified repo. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `did` - The DID of the repo.
  ///
  /// # Errors
  ///
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_get_latest_commit(
    &self,
    did: &str,
  ) -> Result<ComAtprotoSyncGetLatestCommitOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getLatestCommit",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get data blocks needed to prove the existence or non-existence of record in the current version of repo. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * `did` - The DID of the repo.
  /// * `collection`
  /// * `rkey` - Record Key
  /// * `commit` - DEPRECATED: referenced a repo commit by CID, and retrieved record as of that commit
  ///
  /// # Errors
  ///
  /// * `RecordNotFound`
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_get_record(
    &self,
    did: &str,
    collection: &str,
    rkey: &str,
    commit: Option<&str>,
  ) -> Result<Vec<u8>> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    query_.push((String::from("collection"), collection.to_string()));
    query_.push((String::from("rkey"), rkey.to_string()));
    if let Some(commit) = &commit {
      query_.push((String::from("commit"), commit.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getRecord",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(response.bytes().await?.to_vec())
  }

  /// Download a repository export as CAR file. Optionally only a 'diff' since a previous revision. Does not require auth; implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * `did` - The DID of the repo.
  /// * `since` - The revision ('rev') of the repo to create a diff from.
  ///
  /// # Errors
  ///
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_get_repo(&self, did: &str, since: Option<&str>) -> Result<Vec<u8>> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    if let Some(since) = &since {
      query_.push((String::from("since"), since.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getRepo",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(response.bytes().await?.to_vec())
  }

  /// Get the hosting status for a repository, on this server. Expected to be implemented by PDS and Relay.
  ///
  /// # Arguments
  ///
  /// * `did` - The DID of the repo.
  ///
  /// # Errors
  ///
  /// * `RepoNotFound`
  pub async fn com_atproto_sync_get_repo_status(
    &self,
    did: &str,
  ) -> Result<ComAtprotoSyncGetRepoStatusOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.getRepoStatus",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// List blob CIDs for an account, since some repo revision. Does not require auth; implemented by PDS.
  ///
  /// # Arguments
  ///
  /// * `did` - The DID of the repo.
  /// * `since` - Optional revision of the repo to list blobs since.
  /// * `limit`
  /// * `cursor`
  ///
  /// # Errors
  ///
  /// * `RepoNotFound`
  /// * `RepoTakendown`
  /// * `RepoSuspended`
  /// * `RepoDeactivated`
  pub async fn com_atproto_sync_list_blobs(
    &self,
    did: &str,
    since: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoSyncListBlobsOutput> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    if let Some(since) = &since {
      query_.push((String::from("since"), since.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.listBlobs",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Enumerates all the DID, rev, and commit CID for all repos hosted by this service. Does not require auth; implemented by PDS and Relay.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  pub async fn com_atproto_sync_list_repos(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ComAtprotoSyncListReposOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.sync.listRepos",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Notify a crawling service of a recent update, and that crawling should resume. Intended use is after a gap between repo stream events caused the crawling service to disconnect. Does not require auth; implemented by Relay.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_sync_notify_of_update(
    &self,
    body: ComAtprotoSyncNotifyOfUpdateInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.sync.notifyOfUpdate",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Request a service to persistently crawl hosted repos. Expected use is new PDS instances declaring their existence to Relays. Does not require auth.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_sync_request_crawl(
    &self,
    body: ComAtprotoSyncRequestCrawlInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.sync.requestCrawl",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Repository event stream, aka Firehose endpoint. Outputs repo commits with diff data, and identity update events, for all repositories on the current server. See the atproto specifications for details around stream sequencing, repo versioning, CAR diff format, and more. Public and does not require auth; implemented by PDS and Relay.
  ///
  /// # Arguments
  ///
  /// * `cursor` - The last known event seq number to backfill from.
  ///
  /// # Messages
  ///
  /// * ComAtprotoSyncSubscribeReposCommit
  /// * ComAtprotoSyncSubscribeReposIdentity
  /// * ComAtprotoSyncSubscribeReposAccount
  /// * ComAtprotoSyncSubscribeReposHandle
  /// * ComAtprotoSyncSubscribeReposMigrate
  /// * ComAtprotoSyncSubscribeReposTombstone
  /// * ComAtprotoSyncSubscribeReposInfo
  ///
  /// # Errors
  ///
  /// * `FutureCursor`
  /// * `ConsumerTooSlow` - If the consumer of the stream can not keep up with events, and a backlog gets too large, the server will drop the connection.
  pub async fn com_atproto_sync_subscribe_repos(
    &self,
    cursor: Option<i64>,
  ) -> Result<reqwest_websocket::WebSocket> {
    let mut query_ = Vec::new();
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "wss://{}/xrpc/com.atproto.sync.subscribeRepos",
        self.firehose
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    Ok(
      reqwest_websocket::RequestBuilderExt::upgrade(request)
        .send()
        .await?
        .into_websocket()
        .await?,
    )
  }

  /// Check accounts location in signup queue.
  pub async fn com_atproto_temp_check_signup_queue(
    &self,
  ) -> Result<ComAtprotoTempCheckSignupQueueOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/com.atproto.temp.checkSignupQueue",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// DEPRECATED: use queryLabels or subscribeLabels instead -- Fetch all labels from a labeler created after a certain date.
  ///
  /// # Arguments
  ///
  /// * `since`
  /// * `limit`
  pub async fn com_atproto_temp_fetch_labels(
    &self,
    since: Option<i64>,
    limit: Option<i64>,
  ) -> Result<ComAtprotoTempFetchLabelsOutput> {
    let mut query_ = Vec::new();
    if let Some(since) = &since {
      query_.push((String::from("since"), since.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/com.atproto.temp.fetchLabels",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Request a verification code to be sent to the supplied phone number
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn com_atproto_temp_request_phone_verification(
    &self,
    body: ComAtprotoTempRequestPhoneVerificationInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/com.atproto.temp.requestPhoneVerification",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Administrative action to create a new, re-usable communication (email for now) template.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `DuplicateTemplateName`
  pub async fn tools_ozone_communication_create_template(
    &self,
    body: ToolsOzoneCommunicationCreateTemplateInput,
  ) -> Result<ToolsOzoneCommunicationDefsTemplateView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.communication.createTemplate",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Delete a communication template.
  ///
  /// # Arguments
  ///
  /// * body
  pub async fn tools_ozone_communication_delete_template(
    &self,
    body: ToolsOzoneCommunicationDeleteTemplateInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.communication.deleteTemplate",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// Get list of all communication templates.
  pub async fn tools_ozone_communication_list_templates(
    &self,
  ) -> Result<ToolsOzoneCommunicationListTemplatesOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/tools.ozone.communication.listTemplates",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Administrative action to update an existing communication template. Allows passing partial fields to patch specific fields only.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `DuplicateTemplateName`
  pub async fn tools_ozone_communication_update_template(
    &self,
    body: ToolsOzoneCommunicationUpdateTemplateInput,
  ) -> Result<ToolsOzoneCommunicationDefsTemplateView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.communication.updateTemplate",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Take a moderation action on an actor.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `SubjectHasAction`
  pub async fn tools_ozone_moderation_emit_event(
    &self,
    body: ToolsOzoneModerationEmitEventInput,
  ) -> Result<ToolsOzoneModerationDefsModEventView> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.moderation.emitEvent",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about a moderation event.
  ///
  /// # Arguments
  ///
  /// * `id`
  pub async fn tools_ozone_moderation_get_event(
    &self,
    id: i64,
  ) -> Result<ToolsOzoneModerationDefsModEventViewDetail> {
    let mut query_ = Vec::new();
    query_.push((String::from("id"), id.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.getEvent",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about a record.
  ///
  /// # Arguments
  ///
  /// * `uri`
  /// * `cid`
  ///
  /// # Errors
  ///
  /// * `RecordNotFound`
  pub async fn tools_ozone_moderation_get_record(
    &self,
    uri: &str,
    cid: Option<&str>,
  ) -> Result<ToolsOzoneModerationDefsRecordViewDetail> {
    let mut query_ = Vec::new();
    query_.push((String::from("uri"), uri.to_string()));
    if let Some(cid) = &cid {
      query_.push((String::from("cid"), cid.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.getRecord",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about a repository.
  ///
  /// # Arguments
  ///
  /// * `did`
  ///
  /// # Errors
  ///
  /// * `RepoNotFound`
  pub async fn tools_ozone_moderation_get_repo(
    &self,
    did: &str,
  ) -> Result<ToolsOzoneModerationDefsRepoViewDetail> {
    let mut query_ = Vec::new();
    query_.push((String::from("did"), did.to_string()));
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.getRepo",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// List moderation events related to a subject.
  ///
  /// # Arguments
  ///
  /// * `types` - The types of events (fully qualified string in the format of tools.ozone.moderation.defs#modEvent<name>) to filter by. If not specified, all events are returned.
  /// * `created_by`
  /// * `sort_direction` - Sort direction for the events. Defaults to descending order of created at timestamp.
  /// * `created_after` - Retrieve events created after a given timestamp
  /// * `created_before` - Retrieve events created before a given timestamp
  /// * `subject`
  /// * `include_all_user_records` - If true, events on all record types (posts, lists, profile etc.) owned by the did are returned
  /// * `limit`
  /// * `has_comment` - If true, only events with comments are returned
  /// * `comment` - If specified, only events with comments containing the keyword are returned
  /// * `added_labels` - If specified, only events where all of these labels were added are returned
  /// * `removed_labels` - If specified, only events where all of these labels were removed are returned
  /// * `added_tags` - If specified, only events where all of these tags were added are returned
  /// * `removed_tags` - If specified, only events where all of these tags were removed are returned
  /// * `report_types`
  /// * `cursor`
  pub async fn tools_ozone_moderation_query_events(
    &self,
    types: Option<&[&str]>,
    created_by: Option<&str>,
    sort_direction: Option<&str>,
    created_after: Option<&chrono::DateTime<chrono::Utc>>,
    created_before: Option<&chrono::DateTime<chrono::Utc>>,
    subject: Option<&str>,
    include_all_user_records: Option<bool>,
    limit: Option<i64>,
    has_comment: Option<bool>,
    comment: Option<&str>,
    added_labels: Option<&[&str]>,
    removed_labels: Option<&[&str]>,
    added_tags: Option<&[&str]>,
    removed_tags: Option<&[&str]>,
    report_types: Option<&[&str]>,
    cursor: Option<&str>,
  ) -> Result<ToolsOzoneModerationQueryEventsOutput> {
    let mut query_ = Vec::new();
    if let Some(types) = &types {
      query_.append(
        &mut types
          .iter()
          .map(|i| (String::from("types"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(created_by) = &created_by {
      query_.push((String::from("created_by"), created_by.to_string()));
    }
    if let Some(sort_direction) = &sort_direction {
      query_.push((String::from("sort_direction"), sort_direction.to_string()));
    }
    if let Some(created_after) = &created_after {
      query_.push((String::from("created_after"), created_after.to_rfc3339()));
    }
    if let Some(created_before) = &created_before {
      query_.push((String::from("created_before"), created_before.to_rfc3339()));
    }
    if let Some(subject) = &subject {
      query_.push((String::from("subject"), subject.to_string()));
    }
    if let Some(include_all_user_records) = &include_all_user_records {
      query_.push((
        String::from("include_all_user_records"),
        include_all_user_records.to_string(),
      ));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(has_comment) = &has_comment {
      query_.push((String::from("has_comment"), has_comment.to_string()));
    }
    if let Some(comment) = &comment {
      query_.push((String::from("comment"), comment.to_string()));
    }
    if let Some(added_labels) = &added_labels {
      query_.append(
        &mut added_labels
          .iter()
          .map(|i| (String::from("added_labels"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(removed_labels) = &removed_labels {
      query_.append(
        &mut removed_labels
          .iter()
          .map(|i| (String::from("removed_labels"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(added_tags) = &added_tags {
      query_.append(
        &mut added_tags
          .iter()
          .map(|i| (String::from("added_tags"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(removed_tags) = &removed_tags {
      query_.append(
        &mut removed_tags
          .iter()
          .map(|i| (String::from("removed_tags"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(report_types) = &report_types {
      query_.append(
        &mut report_types
          .iter()
          .map(|i| (String::from("report_types"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.queryEvents",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// View moderation statuses of subjects (record or repo).
  ///
  /// # Arguments
  ///
  /// * `include_all_user_records` - All subjects belonging to the account specified in the 'subject' param will be returned.
  /// * `subject` - The subject to get the status for.
  /// * `comment` - Search subjects by keyword from comments
  /// * `reported_after` - Search subjects reported after a given timestamp
  /// * `reported_before` - Search subjects reported before a given timestamp
  /// * `reviewed_after` - Search subjects reviewed after a given timestamp
  /// * `reviewed_before` - Search subjects reviewed before a given timestamp
  /// * `include_muted` - By default, we don't include muted subjects in the results. Set this to true to include them.
  /// * `only_muted` - When set to true, only muted subjects and reporters will be returned.
  /// * `review_state` - Specify when fetching subjects in a certain state
  /// * `ignore_subjects`
  /// * `last_reviewed_by` - Get all subject statuses that were reviewed by a specific moderator
  /// * `sort_field`
  /// * `sort_direction`
  /// * `takendown` - Get subjects that were taken down
  /// * `appealed` - Get subjects in unresolved appealed status
  /// * `limit`
  /// * `tags`
  /// * `exclude_tags`
  /// * `cursor`
  pub async fn tools_ozone_moderation_query_statuses(
    &self,
    include_all_user_records: Option<bool>,
    subject: Option<&str>,
    comment: Option<&str>,
    reported_after: Option<&chrono::DateTime<chrono::Utc>>,
    reported_before: Option<&chrono::DateTime<chrono::Utc>>,
    reviewed_after: Option<&chrono::DateTime<chrono::Utc>>,
    reviewed_before: Option<&chrono::DateTime<chrono::Utc>>,
    include_muted: Option<bool>,
    only_muted: Option<bool>,
    review_state: Option<&str>,
    ignore_subjects: Option<&[&str]>,
    last_reviewed_by: Option<&str>,
    sort_field: Option<&str>,
    sort_direction: Option<&str>,
    takendown: Option<bool>,
    appealed: Option<bool>,
    limit: Option<i64>,
    tags: Option<&[&str]>,
    exclude_tags: Option<&[&str]>,
    cursor: Option<&str>,
  ) -> Result<ToolsOzoneModerationQueryStatusesOutput> {
    let mut query_ = Vec::new();
    if let Some(include_all_user_records) = &include_all_user_records {
      query_.push((
        String::from("include_all_user_records"),
        include_all_user_records.to_string(),
      ));
    }
    if let Some(subject) = &subject {
      query_.push((String::from("subject"), subject.to_string()));
    }
    if let Some(comment) = &comment {
      query_.push((String::from("comment"), comment.to_string()));
    }
    if let Some(reported_after) = &reported_after {
      query_.push((String::from("reported_after"), reported_after.to_rfc3339()));
    }
    if let Some(reported_before) = &reported_before {
      query_.push((
        String::from("reported_before"),
        reported_before.to_rfc3339(),
      ));
    }
    if let Some(reviewed_after) = &reviewed_after {
      query_.push((String::from("reviewed_after"), reviewed_after.to_rfc3339()));
    }
    if let Some(reviewed_before) = &reviewed_before {
      query_.push((
        String::from("reviewed_before"),
        reviewed_before.to_rfc3339(),
      ));
    }
    if let Some(include_muted) = &include_muted {
      query_.push((String::from("include_muted"), include_muted.to_string()));
    }
    if let Some(only_muted) = &only_muted {
      query_.push((String::from("only_muted"), only_muted.to_string()));
    }
    if let Some(review_state) = &review_state {
      query_.push((String::from("review_state"), review_state.to_string()));
    }
    if let Some(ignore_subjects) = &ignore_subjects {
      query_.append(
        &mut ignore_subjects
          .iter()
          .map(|i| (String::from("ignore_subjects"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(last_reviewed_by) = &last_reviewed_by {
      query_.push((
        String::from("last_reviewed_by"),
        last_reviewed_by.to_string(),
      ));
    }
    if let Some(sort_field) = &sort_field {
      query_.push((String::from("sort_field"), sort_field.to_string()));
    }
    if let Some(sort_direction) = &sort_direction {
      query_.push((String::from("sort_direction"), sort_direction.to_string()));
    }
    if let Some(takendown) = &takendown {
      query_.push((String::from("takendown"), takendown.to_string()));
    }
    if let Some(appealed) = &appealed {
      query_.push((String::from("appealed"), appealed.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(tags) = &tags {
      query_.append(
        &mut tags
          .iter()
          .map(|i| (String::from("tags"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(exclude_tags) = &exclude_tags {
      query_.append(
        &mut exclude_tags
          .iter()
          .map(|i| (String::from("exclude_tags"), i.to_string()))
          .collect::<Vec<_>>(),
      );
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.queryStatuses",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Find repositories based on a search term.
  ///
  /// # Arguments
  ///
  /// * `term` - DEPRECATED: use 'q' instead
  /// * `q`
  /// * `limit`
  /// * `cursor`
  pub async fn tools_ozone_moderation_search_repos(
    &self,
    term: Option<&str>,
    q: Option<&str>,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ToolsOzoneModerationSearchReposOutput> {
    let mut query_ = Vec::new();
    if let Some(term) = &term {
      query_.push((String::from("term"), term.to_string()));
    }
    if let Some(q) = &q {
      query_.push((String::from("q"), q.to_string()));
    }
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.moderation.searchRepos",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Get details about ozone's server configuration.
  pub async fn tools_ozone_server_get_config(&self) -> Result<ToolsOzoneServerGetConfigOutput> {
    let mut request = self.client.get(&format!(
      "https://{}/xrpc/tools.ozone.server.getConfig",
      self.host
    ));
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Add a member to the ozone team. Requires admin role.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `MemberAlreadyExists` - Member already exists in the team.
  pub async fn tools_ozone_team_add_member(
    &self,
    body: ToolsOzoneTeamAddMemberInput,
  ) -> Result<ToolsOzoneTeamDefsMember> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.team.addMember",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Delete a member from ozone team. Requires admin role.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `MemberNotFound` - The member being deleted does not exist
  /// * `CannotDeleteSelf` - You can not delete yourself from the team
  pub async fn tools_ozone_team_delete_member(
    &self,
    body: ToolsOzoneTeamDeleteMemberInput,
  ) -> Result<()> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.team.deleteMember",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    Ok(())
  }

  /// List all members with access to the ozone service.
  ///
  /// # Arguments
  ///
  /// * `limit`
  /// * `cursor`
  pub async fn tools_ozone_team_list_members(
    &self,
    limit: Option<i64>,
    cursor: Option<&str>,
  ) -> Result<ToolsOzoneTeamListMembersOutput> {
    let mut query_ = Vec::new();
    if let Some(limit) = &limit {
      query_.push((String::from("limit"), limit.to_string()));
    }
    if let Some(cursor) = &cursor {
      query_.push((String::from("cursor"), cursor.to_string()));
    }
    let mut request = self
      .client
      .get(&format!(
        "https://{}/xrpc/tools.ozone.team.listMembers",
        self.host
      ))
      .query(&query_);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }

  /// Update a member in the ozone service. Requires admin role.
  ///
  /// # Arguments
  ///
  /// * body
  ///
  /// # Errors
  ///
  /// * `MemberNotFound` - The member being updated does not exist in the team
  pub async fn tools_ozone_team_update_member(
    &self,
    body: ToolsOzoneTeamUpdateMemberInput,
  ) -> Result<ToolsOzoneTeamDefsMember> {
    let mut request = self
      .client
      .post(&format!(
        "https://{}/xrpc/tools.ozone.team.updateMember",
        self.host
      ))
      .json(&body);
    if let Some(token) = &self.access_jwt {
      request = request.header("Authorization", format!("Bearer {token}"));
    }
    let response = request.send().await?;
    if response.status() == 429 {
      return Err(Error::Rate((
        response
          .headers()
          .get("ratelimit-limit")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-remaining")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-reset")
          .and_then(|v| v.to_str().ok())
          .and_then(|v| v.parse().ok())
          .unwrap_or_default(),
        response
          .headers()
          .get("ratelimit-policy")
          .and_then(|v| v.to_str().map(|v| v.to_string()).ok())
          .unwrap_or_default(),
      )));
    }
    let text = response.text().await?;
    Ok(serde_json::from_str(&text).map_err(|e| Error::from((e, text)))?)
  }
}
