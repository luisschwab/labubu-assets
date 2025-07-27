//! Labubu Assets
//!
//! Labubu Assets home page.

use dioxus::prelude::*;

use bitcoin::{Address, Network};
use secp256k1::{rand::thread_rng, Keypair, SecretKey, SECP256K1};
use web_sys::window;

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
    let mut image_uri = use_signal(|| "data:image/png;base64,".to_string());

    // Load the private key from local storage.
    use_effect(move || {
        if let Ok(Some(sk)) = storage.get("SecretKey") {
            sk_string.set(sk);
        }
        if let Ok(Some(deposit)) = storage.get("DepositAddress") {
            deposit_address_string.set(deposit);
        }
        if let Ok(Some(destination)) = storage.get("DestinationAddress") {
            destination_address_string.set(destination);
        }
    });

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-indigo-500 to-purple-600 p-8",

            div { class: "max-w-4xl mx-auto bg-white/95 backdrop-blur-sm rounded-3xl p-8 shadow-2xl",

                div { class: "text-center mb-12",
                    h1 { class: "text-5xl font-bold mb-3 bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent",
                        "🎯 Labubu Assets"
                    }
                    p { class: "text-xl text-gray-600", "Labubu Assets™ Mint" }
                }

                // Secret Key Section
                div { class: "bg-gradient-to-r from-blue-50 to-indigo-50 rounded-2xl p-6 mb-8 border border-blue-200",

                    h3 { class: "text-2xl font-bold text-gray-800 mb-4 flex items-center gap-3",
                        "🔐 Secret Key"
                    }

                    div { class: "bg-white/90 border-2 border-gray-200 rounded-xl p-4 mb-4 min-h-[4rem] flex items-center shadow-inner",
                        if sk_string() == "nil" {
                            span { class: "text-gray-400 italic", "No secret key generated yet" }
                        } else {
                            span { class: "break-all text-sm leading-relaxed text-gray-800",
                                "{sk_string}"
                            }
                        }
                    }

                    button {
                        class: "w-full bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white font-semibold py-4 px-8 rounded-xl transition-all duration-300 transform hover:-translate-y-1 hover:shadow-xl",
                        onclick: move |_| {
                            let mut rng = thread_rng();
                            let sk = SecretKey::new(&mut rng);
                            let _ = storage_c1
                                .set("SecretKey", &serde_json::to_string(&sk).unwrap())
                                .unwrap();
                            sk_string.set(storage_c1.get("SecretKey").unwrap().unwrap());
                        },
                        "🎲 Generate New Secret Key"
                    }
                }


                // Deposit Address Section
                div { class: "bg-gradient-to-r from-green-50 to-emerald-50 rounded-2xl p-6 mb-8 border border-green-200",

                    h3 { class: "text-2xl font-bold text-gray-800 mb-4 flex items-center gap-3",
                        "📍 Deposit Address"
                    }

                    div { class: "bg-white/90 border-2 border-green-200 rounded-xl p-4 mb-4 min-h-[4rem] flex items-center shadow-inner",
                        if deposit_address_string() == "nil" {
                            span { class: "text-gray-400 italic",
                                "Generate a secret key first, then create your deposit address"
                            }
                        } else {
                            span { class: "break-all text-sm leading-relaxed text-gray-800",
                                "{deposit_address_string}"
                            }
                        }
                    }

                    button {
                        class: "w-full bg-gradient-to-r from-green-500 to-green-600 hover:from-green-600 hover:to-green-700 text-white font-semibold py-4 px-8 rounded-xl transition-all duration-300 transform hover:-translate-y-1 hover:shadow-xl",
                        onclick: move |_| {
                            if let Ok(Some(sk_string)) = storage_c2.get("SecretKey") {
                                let sk: SecretKey = serde_json::from_str(&sk_string).unwrap();
                                let keypair = Keypair::from_secret_key(&SECP256K1, &sk);
                                let (pk, _) = keypair.x_only_public_key();
                                let pk_ser = pk.serialize();
                                let seed = u64::from_be_bytes([
                                    pk_ser[0],
                                    pk_ser[1],
                                    pk_ser[2],
                                    pk_ser[3],
                                    pk_ser[4],
                                    pk_ser[5],
                                    pk_ser[6],
                                    pk_ser[7],
                                ]);
                                let payload = labubu_maker(seed);
                                let spend_info = create_control_block_address(pk, payload.clone()).unwrap();
                                let address = Address::p2tr_tweaked(
                                    spend_info.output_key(),
                                    Network::Bitcoin,
                                );
                                let _ = storage_c2.set("DepositAddress", &address.to_string()).unwrap();
                                deposit_address_string
                                    .set(storage_c2.get("DepositAddress").unwrap().unwrap());
                                let image_data = format!(
                                    "data:image/png;base64,{}",
                                    base64::encode(&payload),
                                );
                                image_uri.set(image_data);
                            }
                        },
                        "🏗️ Generate Deposit Address"
                    }

                    div { class: "flex flex-col items-enter gap-4 p-4",
                        img {
                            class: "w-56 h-auto items-center gap-4 p-4",
                            src: "{image_uri}",
                        }
                    }
                
                }

                // Destination Address Section
                div { class: "bg-gradient-to-r from-orange-50 to-amber-50 rounded-2xl p-6 border border-orange-200",

                    h3 { class: "text-2xl font-bold text-gray-800 mb-4 flex items-center gap-3",
                        "🎯 Destination Address"
                    }

                    div { class: "bg-white/90 border-2 border-orange-200 rounded-xl p-4 mb-4 min-h-[4rem] flex items-center shadow-inner",
                        if destination_address_string() == "nil" {
                            span { class: "text-gray-400 italic", "Enter a destination address below" }
                        } else {
                            span { class: "break-all text-sm leading-relaxed text-gray-800",
                                "{destination_address_string}"
                            }
                        }
                    }

                    input {
                        class: "w-full px-5 py-4 border-2 border-gray-200 rounded-xl text-base transition-all duration-300 bg-white/90 shadow-inner focus:outline-none focus:border-orange-500 focus:ring-4 focus:ring-orange-100 focus:-translate-y-0.5 mb-4",
                        r#type: "text",
                        value: "{destination_address_input}",
                        oninput: move |e| destination_address_input.set(e.value()),
                        placeholder: "bc1p... (Enter Bitcoin address)",
                    }

                    button {
                        class: "w-full bg-gradient-to-r from-orange-500 to-orange-600 hover:from-orange-600 hover:to-orange-700 text-white font-semibold py-4 px-8 rounded-xl transition-all duration-300 transform hover:-translate-y-1 hover:shadow-xl",
                        onclick: move |_| {
                            let address_string = destination_address_input();
                            let _ = storage_c3.set("DestinationAddress", &address_string).unwrap();
                            destination_address_string
                                .set(storage_c3.get("DestinationAddress").unwrap().unwrap());
                        },
                        "💾 Save Address"
                    }
                }
            }
        }
    }
}
