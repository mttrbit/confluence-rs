use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    pub base: String,
    pub context: String,
    #[serde(rename = "self")]
    pub target_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SelfLink {
    #[serde(rename = "self")]
    pub target_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Expandable {
    pub attachment: Option<String>,
    pub comment: Option<String>,
    pub page: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extensions {
    pub position: Option<String>,
    #[serde(rename = "mediaType")]
    pub media_type: Option<String>,
    #[serde(rename = "fileSize")]
    pub file_size: Option<u64>,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentLinks {
    pub webui: Option<String>,
    pub thumbnail: Option<String>,
    pub download: Option<String>,
    pub edit: Option<String>,
    pub tinyui: Option<String>,
    #[serde(rename = "self")]
    pub target_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfilePicture {
    pub path: String,
    pub width: u64,
    pub height: u64,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserExpandable {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "type")]
    pub user_type: String,
    pub username: String,
    #[serde(rename = "userKey")]
    pub user_key: String,
    #[serde(rename = "profilePicture")]
    pub profile_picture: ProfilePicture,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "_links")]
    pub links: SelfLink,
    #[serde(rename = "_expandable")]
    pub expandable: UserExpandable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentVersionExpandable {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentVersion {
    pub by: User,
    pub when: String,
    pub message: Option<String>,
    pub number: u64,
    #[serde(rename = "minorEdit")]
    pub minor_edit: bool,
    pub hidden: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentExpandable {
    pub container: Option<String>,
    pub metadata: Option<String>,
    pub operations: Option<String>,
    pub children: Option<String>,
    pub restrictions: Option<String>,
    pub history: Option<String>,
    pub ancestors: Option<String>,
    pub body: Option<String>,
    pub version: Option<String>,
    pub descendants: Option<String>,
    pub space: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultsLinks {
    #[serde(rename = "self")]
    pub target_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results<T> {
    pub results: Vec<T>,
    pub start: u64,
    pub limit: u64,
    pub size: u64,
    #[serde(rename = "_links")]
    pub links: ResultsLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub prefix: String,
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub labels: Results<Label>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub id: String,
    #[serde(rename = "type")]
    pub content_type: String,
    pub status: String,
    pub title: String,
    pub version: Option<ContentVersion>,
    pub extensions: Option<Extensions>,
    #[serde(rename = "_links")]
    pub links: ContentLinks,
    #[serde(rename = "_expandable")]
    pub expandable: ContentExpandable,
    pub metadata: Option<Metadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildContentServiceResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    #[serde(rename = "_expandable")]
    pub expandable: ContentExpandable,
    pub page: Results<Content>,
}

/////////////////////////////////////////
// ContentService
/////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServiceExpandable {
    pub children: Option<String>,
    pub descendants: Option<String>,
    pub metadata: Option<String>,
    pub operations: Option<String>,
    pub restrictions: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServiceLinks {
    pub base: String,
    pub collection: String,
    pub context: String,
    pub edit: String,
    #[serde(rename = "self")]
    pub target_url: String,
    pub tinyui: String,
    pub webui: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ancestor {
    pub id: String,
}

impl Ancestor {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServiceBodyExpandable {
    pub anonymous_export_view: Option<String>,
    pub editor: Option<String>,
    pub export_view: Option<String>,
    pub styled_view: Option<String>,
    pub view: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServiceBodyStorageExpandable {
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServiceBodyStorage {
    #[serde(rename = "_expandable")]
    pub expandable: ContentServiceBodyStorageExpandable,
    pub representation: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServiceBodyContainerExpandable {
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub icon: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServiceBodyContainerLinks {
    #[serde(rename = "self")]
    pub target_url: String,
    pub webui: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentHistoryExpandable {
    pub contributors: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    #[serde(rename = "nextVersion")]
    pub next_version: String,
    #[serde(rename = "previousVersion")]
    pub previous_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentSpaceExpandable {
    pub description: String,
    pub homepage: String,
    pub icon: String,
    pub metadata: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentSpaceLinks {
    #[serde(rename = "self")]
    pub target_url: String,
    pub webui: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Space {
    #[serde(rename = "_expandable")]
    pub expandable: ContentSpaceExpandable,
    #[serde(rename = "_links")]
    pub links: ContentSpaceLinks,
    pub id: u64,
    pub key: String,
    pub name: String,
    #[serde(rename = "type")]
    pub space_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentHistory {
    #[serde(rename = "_expandable")]
    pub expandable: ContentHistoryExpandable,
    #[serde(rename = "_links")]
    pub links: SelfLink,
    #[serde(rename = "createdBy")]
    pub created_by: User,
    #[serde(rename = "createdDate")]
    pub created_date: String,
    pub latest: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServiceBodyContainer {
    #[serde(rename = "_expandable")]
    pub expandable: ContentServiceBodyContainerExpandable,
    #[serde(rename = "_links")]
    pub links: ContentServiceBodyContainerLinks,
    pub id: u64,
    pub key: String,
    pub name: String,
    #[serde(rename = "type")]
    pub container_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServiceBody {
    #[serde(rename = "_expandable")]
    pub expandable: ContentServiceBodyExpandable,
    pub storage: ContentServiceBodyStorage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentServiceResponse {
    #[serde(rename = "_links")]
    pub links: ContentServiceLinks,
    #[serde(rename = "_expandable")]
    pub expandable: ContentServiceExpandable,
    pub ancestors: Vec<Ancestor>,
    pub body: ContentServiceBody,
    pub container: ContentServiceBodyContainer,
    pub extensions: Extensions,
    pub history: ContentHistory,
    pub id: String,
    pub space: Space,
    pub status: String,
    pub title: String,
    #[serde(rename = "type")]
    pub content_type: String,
    pub version: ContentVersion,
}
