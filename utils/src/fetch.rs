use std::sync::LazyLock;

pub static ASYNC_CLIENT: LazyLock<reqwest::Client> =
    LazyLock::new(|| reqwest::ClientBuilder::new().build().unwrap());
