//! Labubu Assets

use base64;
use bitcoin::{Address, Network};
use dioxus::prelude::*;
use secp256k1::{rand::random, Keypair, Secp256k1, SecretKey};

use views::{Home, Navbar};

use crate::labubu::{create_control_block_address, mint};

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
     let secp = Secp256k1::new();
    // pick a fresh pseudo‑random secret key & update URI
    let mut rng = secp256k1::rand::thread_rng();
    let secret_key = SecretKey::new(&mut rng);
    let keypair = Keypair::from_secret_key(&secp, &secret_key);
    let pubkey = keypair.public_key();
    let seed: u64 = random();
    let png_bytes = labubu_maker(seed);
    let spend_info = create_control_block_address(pubkey.into(), png_bytes.clone())
        .expect("Failed to create control block address");

    let address = Address::p2tr_tweaked(spend_info.output_key(), Network::Signet);

    println!("Labubu address {:?}", address);

    let transaction = mint(
        pubkey.into(),
        100_000, // amount in satoshis
        address,
        1_000, // fee in satoshis
        vec![], // inputs (empty for this example)
        vec![], // previous txouts (empty for this example)
        spend_info,
        keypair,
    )
    .expect("Failed to mint Labubu asset");

    //broadcast the transaction to the network


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
