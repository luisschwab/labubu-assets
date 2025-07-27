//! Labubu Assets
//!
//! Labubu Assets home page.

use dioxus::prelude::*;

use bitcoin::{Address, Network};
use secp256k1::{rand::thread_rng, Keypair, SecretKey, SECP256K1};
use serde::Serialize;
use web_sys::{window, Storage};

use crate::labubu::create_control_block_address;
use crate::labubu_maker::labubu_maker;

#[component]
pub fn Home() -> Element {
    // Save some stuff to local storage:
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let storage_c1 = storage.clone();
    let storage_c2 = storage.clone();
    let storage_c3 = storage.clone();

    let mut sk_string = use_signal(|| "nil".to_string());
    let mut deposit_address_string = use_signal(|| "nil".to_string());
    let mut destination_address_input = use_signal(|| String::new());
    let mut destination_address_string = use_signal(|| "nil".to_string());

    // Load the private key from local storage.
    use_effect(move || {
        if let Ok(Some(sk)) = storage.get("SecretKey") {
            sk_string.set(sk);
        }
    });

    rsx! {
        // TODO(@luisschwab): make it pretty.

        // Generate a [`SecretKey`] and save it to local storage under "SecretKey".
        h3 { "SecretKey: {sk_string}"}
        button { onclick: move |_| {
            let mut rng = thread_rng();
            let sk = SecretKey::new(&mut rng);

            let _ = storage_c1.set("SecretKey", &serde_json::to_string(&sk).unwrap()).unwrap();
            sk_string.set(storage_c1.get("SecretKey").unwrap().unwrap());
        }}

        // Generate the deposit address and save it to local storage under "DepositAddress".
        h3 { "Deposit Address: {deposit_address_string} "}
        button { onclick: move |_| {
            let sk_string = storage_c2.get("SecretKey").unwrap().unwrap();

            let sk: SecretKey = serde_json::from_str(&sk_string).unwrap();
            let keypair = Keypair::from_secret_key(SECP256K1, &sk);
            let (pk, _) = keypair.x_only_public_key();

            let pk_ser = pk.serialize();
            let seed = u64::from_be_bytes([
                pk_ser[0], pk_ser[1], pk_ser[2], pk_ser[3],
                pk_ser[4], pk_ser[5], pk_ser[6], pk_ser[7]
            ]);
            let payload = labubu_maker(seed);
            let spend_info = create_control_block_address(pk, payload).unwrap();
            let address = Address::p2tr_tweaked(spend_info.output_key(), Network::Bitcoin);

            let _ = storage_c2.set("DepositAddress", &address.to_string()).unwrap();
            deposit_address_string.set(storage_c2.get("DepositAddress").unwrap().unwrap());
        }}

        // Prompt the user for the destination address and save it to local storage under "DestinationAddress" (not strictly needed but may be useful).
        h3 { "Destination Address: {destination_address_string}" }
        input {
            r#type: "text",
            value: "{destination_address_input}",
            oninput: move |e| destination_address_input.set(e.value()),
            placeholder: "enter Labubu destination address",
        }
        button { onclick: move |_| {
            let address_string = destination_address_input();
            let _ = storage_c3.set("DestinationAddress", &address_string).unwrap();
            destination_address_string.set(storage_c3.get("DestinationAddress").unwrap().unwrap());
        }}

    }
}

// use crate::labubu_maker::labubu_maker;

// Convert Vec<u8> to base64 data URI string if necessary
// let mut image_uri = use_signal(|| {
//     let png_bytes = labubu_maker(0x1337);
//     format!("data:image/png;base64,{}", base64::encode(png_bytes))
// });

// rsx! {
//     div { class: "flex flex-col items-center gap-4 p-4",
//         img {
//             class: "w-56 h-auto rounded-2xl shadow-lg",
//             src: "{image_uri.read()}",
//         }
//         button {
//             class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-xl",
//             onclick: move |_| {
//                 // pick a fresh pseudo‑random seed & update URI
//                 let seed: u64 = random();
//                 let png_bytes = labubu_maker(seed);
//                 let uri = format!("data:image/png;base64,{}", base64::encode(png_bytes));
//                 image_uri.set(uri);
//             },
//             "Generate new Labubu"
//         }
//     }
// }
