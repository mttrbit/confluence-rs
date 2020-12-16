use {
    crate::content::model::{Ancestor, Content},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Space {
    pub key: String,
}

impl Space {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub storage: Storage,
}

impl Body {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    pub value: String,
    pub representation: String,
}

impl Storage {
    pub fn new(value: &str, representation: &str) -> Self {
        Self {
            value: value.to_string(),
            representation: representation.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub number: u64,
}

impl Version {
    pub fn new(number: u64) -> Self {
        Self { number }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePageRequest {
    #[serde(rename = "type")]
    pub type_name: String,
    pub title: String,
    pub space: Space,
    pub body: Body,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<Ancestor>>,
}

impl CreatePageRequest {
    pub fn new(
        type_name: &str,
        title: &str,
        space: Space,
        body: Body,
        ancestors: Option<Vec<Ancestor>>,
    ) -> Self {
        Self {
            type_name: type_name.to_string(),
            title: title.to_string(),
            space,
            body,
            ancestors,
        }
    }
}

impl Clone for CreatePageRequest {
    fn clone(&self) -> Self {
        Self {
            type_name: self.type_name.clone(),
            title: self.title.clone(),
            space: Space::new(&self.space.key.clone()),
            body: Body::new(Storage::new(
                &self.body.storage.value.clone(),
                &self.body.storage.representation.clone(),
            )),
            ancestors: match &self.ancestors {
                Some(v) => {
                    if v.is_empty() {
                        None
                    } else {
                        Some(vec![Ancestor::new(&v.get(0).unwrap().id)])
                    }
                }
                _ => None,
            },
        }
    }
}

#[derive(Debug)]
pub struct FindPageRequest {
    pub title: String,
    pub space: String,
    pub expand: String,
}

impl FindPageRequest {
    pub fn new(title: &str, space: &str) -> Self {
        Self {
            title: title.to_string(),
            space: space.to_string(),
            expand: "version".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct FindChildPageRequest {
    pub content_id: String,
    pub title: String,
}

impl FindChildPageRequest {
    pub fn new(content_id: &str, title: &str) -> Self {
        Self {
            title: title.to_string(),
            content_id: content_id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePageRequest {
    #[serde(skip_serializing)]
    pub id: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub title: String,
    pub space: Space,
    pub body: Body,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<Ancestor>>,
    pub version: Version,
}

impl UpdatePageRequest {
    pub fn new(
        id: &str,
        type_name: &str,
        title: &str,
        space: Space,
        body: Body,
        ancestors: Option<Vec<Ancestor>>,
        version: Version,
    ) -> Self {
        Self {
            id: id.to_string(),
            type_name: type_name.to_string(),
            title: title.to_string(),
            space,
            body,
            ancestors: None,
            version,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadAttachmentRequest {
    #[serde(skip_serializing)]
    pub content_id: String,
    pub name: String,
    pub file: String,
    pub comment: String,
}

impl UploadAttachmentRequest {
    pub fn new(content_id: &str, file_name: &str, file: &str, comment: &str) -> Self {
        Self {
            content_id: content_id.to_string(),
            name: file_name.to_string(),
            file: file.to_string(),
            comment: comment.to_string(),
        }
    }

    pub fn to_update_attachment_request(&self, data: AttachmentData) -> UpdateAttachmentRequest {
        UpdateAttachmentRequest {
            attachment_id: data.id.to_string(),
            content_id: self.content_id.clone(),
            name: self.name.clone(),
            file: self.file.clone(),
            comment: self.comment.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAttachmentRequest {
    #[serde(skip_serializing)]
    pub content_id: String,
    #[serde(skip_serializing)]
    pub attachment_id: String,
    pub name: String,
    pub file: String,
    pub comment: String,
}

impl UpdateAttachmentRequest {
    pub fn new(
        content_id: &str,
        attachment_id: &str,
        file_name: &str,
        file: &str,
        comment: &str,
    ) -> Self {
        Self {
            content_id: content_id.to_string(),
            attachment_id: attachment_id.to_string(),
            name: file_name.to_string(),
            file: file.to_string(),
            comment: comment.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentData {
    pub id: String,
    pub title: String,
    pub version: u64,
}

impl From<&Content> for ContentData {
    fn from(content: &Content) -> Self {
        let v = match &content.version {
            Some(v) => v.number,
            None => 1,
        };
        Self {
            id: content.id.clone(),
            title: content.title.clone(),
            version: v,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttachmentData {
    id: String,
    title: String,
}

impl From<&Content> for AttachmentData {
    fn from(content: &Content) -> Self {
        Self {
            id: content.id.clone(),
            title: content.title.clone(),
        }
    }
}
