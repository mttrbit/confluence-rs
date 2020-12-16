# Confluence API

Pure Rust bindings to the Confluence API.

Note: At the moment, this client only supports the use case the author needs to deal with, which is a session based authorization.
The Client can easily be modified to support both api tokens and basic authentication if needed.

# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
confluence = { git = "https://github.com/mttrbit/confluence-rs", branch = "main"}
```

Confluence is built with Rust 1.48.

