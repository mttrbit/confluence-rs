#![allow(deprecated)] // cause()
error_chain! {
    foreign_links {
        CellBorrowMut(::std::cell::BorrowMutError);
        Reqwest(::reqwest::Error);
        HeaderValue(::reqwest::header::InvalidHeaderValue);
        Io(::std::io::Error);
        SerdeJson(::serde_json::Error);
    }
}
