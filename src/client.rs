use crate::util::url_join;
use reqwest::blocking::{Client, Request};
pub use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method, StatusCode,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub trait Executor {
    fn execute<T>(self) -> Result<(HeaderMap, StatusCode, Option<T>)>
    where
        T: DeserializeOwned;
}

pub struct Confluence {
    client: Rc<Client>,
    host: String,
}

impl<'g> Clone for Confluence {
    fn clone(&self) -> Self {
        Self {
            client: Rc::clone(&self.client),
            host: self.host.clone(),
        }
    }
}

new_type!(GetQueryBuilder);
new_type!(PostQueryBuilder);
new_type!(PutQueryBuilder);
new_type!(CustomQuery);
exec!(CustomQuery);

impl Confluence {
    pub fn new(host: &str) -> Self {
        let client = Client::builder().cookie_store(true).build().unwrap();
        Self {
            client: Rc::new(client),
            host: host.to_string(),
        }
    }

    pub fn with_client(client: Rc<Client>, host: &str) -> Self {
        Self {
            client,
            host: host.to_string(),
        }
    }

    pub fn get(&self) -> GetQueryBuilder {
        self.into()
    }

    pub fn post<T>(&self, body: T) -> PostQueryBuilder
    where
        T: Serialize,
    {
        let mut qb: PostQueryBuilder = self.into();
        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    *qbr.get_mut().body_mut() = Some(json.into());
                    qb.request = Ok(qbr);
                }
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }

        qb
    }
    pub fn put<T>(&self, body: T) -> PutQueryBuilder
    where
        T: Serialize,
    {
        let mut qb: PutQueryBuilder = self.into();
        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    *qbr.get_mut().body_mut() = Some(json.into());
                    qb.request = Ok(qbr);
                }
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }

        qb
    }
}

impl<'g> GetQueryBuilder<'g> {
    func_client!(custom_endpoint, CustomQuery, endpoint_str);

    func_client!(content, crate::content::get::Content<'g>);

    pub fn set_header(
        mut self,
        header_name: impl Into<HeaderName>,
        accept_header: impl Into<HeaderValue>,
    ) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(header_name.into(), accept_header.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

impl<'g> PostQueryBuilder<'g> {
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(content, crate::content::post::Content<'g>);

    pub fn set_header(
        mut self,
        header_name: impl Into<HeaderName>,
        accept_header: impl Into<HeaderValue>,
    ) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(header_name.into(), accept_header.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

impl<'g> PutQueryBuilder<'g> {
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    func_client!(content, crate::content::put::Content<'g>);
}

from!(
    @GetQueryBuilder
        => "GET"
    @PostQueryBuilder
        => "POST"
    @PutQueryBuilder
        => "PUT"
);

from!(
    @GetQueryBuilder
        => CustomQuery
    @PostQueryBuilder
        => CustomQuery
    @PutQueryBuilder
        => CustomQuery
);

impl<'a> CustomQuery<'a> {
    pub fn set_header(
        mut self,
        header_name: impl Into<HeaderName>,
        accept_header: impl Into<HeaderValue>,
    ) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(header_name.into(), accept_header.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::model::{Ancestor, ChildContentServiceResponse, Content, Results};
    use crate::model::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    fn setup_confluence_connection() -> Confluence {
        Confluence::new("https://your.confluence.example.com/rest/api")
    }

    #[test]
    fn list_content() {
        let (_, status, d) = setup_confluence_connection()
            .get()
            .custom_endpoint("content")
            .execute::<serde_json::Value>()
            .unwrap();

        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }

    #[test]
    fn list_child_pages() {
        let (headers, status, d) = setup_confluence_connection()
            .get()
            .content()
            .content_id("205613650")
            .child()
            .expand("page.version")
            // .execute::<serde_json::Value>()
            .execute::<ChildContentServiceResponse>()
            .unwrap();
        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }

    #[test]
    fn find_page() {
        let (headers, status, d) = setup_confluence_connection()
            .get()
            .content()
            .space_key("ICF")
            .title("My+Fancy+Page+Title")
            .expand("version")
            // .execute::<serde_json::Value>()
            .execute::<Results<Content>>()
            .unwrap();
        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }

    #[test]
    fn find_child_page() {
        let (headers, status, d) = setup_confluence_connection()
            .get()
            .content()
            .content_id("205618124")
            .child()
            .expand("page.version")
            //.execute::<serde_json::Value>()
            .execute::<ChildContentServiceResponse>()
            .unwrap();
        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }

    #[test]
    fn find_attachments() {
        let (headers, status, d) = setup_confluence_connection()
            .get()
            .content()
            .content_id("205618124")
            .child()
            .attachment()
            //.execute::<serde_json::Value>()
            .execute::<Results<Content>>()
            .unwrap();
        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }

    #[test]
    fn find_attachments_by() {
        let (headers, status, d) = setup_confluence_connection()
            .get()
            .content()
            .content_id("205618124")
            .child()
            .attachment()
            .filename("profile.jpg")
            //.execute::<serde_json::Value>()
            .execute::<Results<Content>>()
            .unwrap();
        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }

    #[test]
    fn create_page() {
        let request = CreatePageRequest::new(
            "page",
            "Hello World",
            Space::new("~john.doe@example.com"),
            Body::new(Storage::new("no text", "storage")),
            None,
        );
        let (headers, status, d) = setup_confluence_connection()
            .post(request)
            .content()
            .execute::<serde_json::Value>()
            .unwrap();

        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }

    #[test]
    fn update_page() {
        let request = UpdatePageRequest::new(
            "205618124",
            "page",
            "Once Upon",
            Space::new("~john.doe@example.com"),
            Body::new(Storage::new("no text", "storage")),
            None,
            Version::new(2),
        );

        let (headers, status, d) = setup_confluence_connection()
            .put(request)
            .content()
            .content_id("205618124")
            .execute::<serde_json::Value>()
            .unwrap();

        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }

    #[test]
    fn create_child_page() {
        let request = CreatePageRequest::new(
            "page",
            "Hello Foo",
            Space::new("~john.doe@example.com"),
            Body::new(Storage::new("no text", "storage")),
            Some(vec![Ancestor::new("205618124")]),
        );
        let (_, status, d) = setup_confluence_connection()
            .post(request)
            .content()
            .execute::<serde_json::Value>()
            .unwrap();

        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }

    #[test]
    fn upload_attachment() {
        let request = UploadAttachmentRequest::new(
            "205618124",
            "sequence.png",
            "/absolute/path/attachment/sequence.png",
            "",
        );
        let file = File::open(request.file.clone()).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = Vec::new();
        buf_reader.read_to_end(&mut content).unwrap();

        let part = reqwest::blocking::multipart::Part::bytes(content)
            .file_name(request.name.clone())
            .mime_str("image/png")
            .unwrap();

        let form = reqwest::blocking::multipart::Form::new().part("file", part);
        let (_, status, d) = setup_confluence_connection()
            .post(request)
            .content()
            .content_id("205618124")
            .child()
            .attachment(form)
            .execute::<serde_json::Value>()
            .unwrap();

        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }

    #[test]
    fn update_attachment() {
        let request = UploadAttachmentRequest::new(
            "205618124",
            "sequence.png",
            "/absolute/path/attachment/sequence.png",
            "",
        );
        let file = File::open(request.file.clone()).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = Vec::new();
        buf_reader.read_to_end(&mut content).unwrap();

        let part = reqwest::blocking::multipart::Part::bytes(content)
            .file_name(request.name.clone())
            .mime_str("image/png")
            .unwrap();

        let form = reqwest::blocking::multipart::Form::new().part("file", part);
        let (_, status, d) = setup_confluence_connection()
            .post(request)
            .content()
            .content_id("205618124")
            .child()
            .attachment(form)
            .attachment_id("205618156")
            .data()
            .execute::<serde_json::Value>()
            .unwrap();

        assert_eq!(status, reqwest::StatusCode::OK);
        assert!(d.is_some());
    }
}
