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

fn App() -> Element {
    // Ensure labubu_png_data_uri is imported from the correct module
    use crate::labubu_maker::labubu_maker;

    // Convert Vec<u8> to base64 data URI string if necessary
    let mut image_uri = use_signal(|| {
        let png_bytes = labubu_maker(0x1337);
        format!("data:image/png;base64,{}", base64::encode(png_bytes))
    });

    rsx! {
        div { class: "flex flex-col items-center gap-4 p-4",
            img {
                class: "w-56 h-auto rounded-2xl shadow-lg",
                src: "{image_uri.read()}",
            }
            button {
                class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-xl",
                onclick: move |_| {
                    // pick a fresh pseudo‑random seed & update URI
                    let seed: u64 = random();
                    let png_bytes = labubu_maker(seed);
                    let uri = format!("data:image/png;base64,{}", base64::encode(png_bytes));
                    image_uri.set(uri);
                },
                "Generate new Labubu"
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
