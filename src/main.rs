// #![deny(clippy::all)]
// #![deny(clippy::pedantic)]
// #![deny(clippy::nursery)]
// #![deny(clippy::cargo)]
// #![deny(missing_docs)]

use z2p::{get_server, get_state};

#[tokio::main]
async fn main() {
    let state = get_state().await;
    get_server(state).await.unwrap();
}
