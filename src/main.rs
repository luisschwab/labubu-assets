//! Labubu Assets

use base64;
use bitcoin::Network;
use dioxus::prelude::*;
use secp256k1::rand::random;

use views::{Home, HexConverter};

pub(crate) mod components;
pub(crate) mod error;
pub(crate) mod esplora;
pub(crate) mod labubu;
pub(crate) mod labubu_maker;
pub(crate) mod types;
pub(crate) mod views;

/// The default [`Network`].
static NETWORK: Network = Network::Bitcoin;
/// The default Esplora endpoint.
static ESPLORA_ENDPOINT: GlobalSignal<String> =
    Global::new(|| "https://mempool.space/api".to_string());

const FAVICON: Asset = asset!("/assets/favicon.jpg");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/converter")]
    HexConverter {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
