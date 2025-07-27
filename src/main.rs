//! Labubu Assets

use base64;
use dioxus::prelude::*;
use secp256k1::rand::random;

use views::{Home, Navbar};

pub(crate) mod components;
pub(crate) mod error;
pub(crate) mod esplora;
pub(crate) mod labubu;
pub(crate) mod labubu_maker;
pub(crate) mod types;
pub(crate) mod views;

/// The default [`Network`].
static NETWORK: GlobalSignal<String> = Global::new(|| "Signet".to_string());
/// The default Esplora endpoint.
static ESPLORA_ENDPOINT: GlobalSignal<String> =
    Global::new(|| "https://mempool.space/signet/api".to_string());

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
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
