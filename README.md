# Confluence API

Pure Rust bindings to the Confluence API based on https://docs.atlassian.com/atlassian-confluence/REST/6.6.0/.

I hope the presented code inspires or help others to build their own tools.

Note: At the moment, this client only supports the use case the author needs to deal with, which is a session based authorization.
The Client can easily be modified to support both api tokens and basic authentication if needed.

# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
confluence = { git = "https://github.com/mttrbit/confluence-rs", branch = "main"}
```

Confluence is built with Rust 1.48.

# Usage
There is no documentation yet, so please check out the tests in `client.rs` to get an idea of how to use the client.

As only my session based workflow is supported, the client will be initialiezed using something like
```rust,ignored
let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
let rc_client = std::rc::Rc::new(client);
let rc_user = std::rc::Rc::new(crate::client::User::new(username, password));
let _ = saml_auth::SAMLAuth::new(&rc_client, &rc_user)
        .authenticate()
        .unwrap();
let svc = ConfluenceService::new(Confluence::with_client(
        rc_client,
        "https://your.confluence.example.com",
));
```

In teh example above saml based authentication workflow is executed in order to fetch a valid session cookie. This cookie is stored in the
cookie store of the reqwest client. In my workflow this cookie is needed to invoke the Confluence Rest API.

# How to extend
If you need basic auth or an API tokne based workflow, you can easily extend this code, by adding new fields to `struct Confluence` in `client.rs`. 
In `macros.rs` you need to add some logic the `Executor` implementation to ensure the new authentication mechanism is supported.
