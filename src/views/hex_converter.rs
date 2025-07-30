use dioxus::prelude::*;
use hex::decode;
use image::{codecs::png::PngEncoder, ColorType, ImageBuffer, ImageReader, Rgba};
use std::io::Cursor;
use base64;

#[component]
pub fn HexConverter() -> Element {
    let mut hex_input = use_signal(|| String::new());
    let mut image_uri = use_signal(|| "data:image/png;base64,".to_string());
    let mut error_message = use_signal(|| String::new());

    rsx! {
        div { 
            class: "min-h-screen bg-gradient-to-br from-purple-400 via-pink-400 to-yellow-400 p-4",
            style: "background-image: radial-gradient(circle at 20% 80%, rgba(120, 119, 198, 0.3) 0%, transparent 50%), radial-gradient(circle at 80% 20%, rgba(255, 119, 198, 0.3) 0%, transparent 50%), radial-gradient(circle at 40% 40%, rgba(120, 219, 255, 0.3) 0%, transparent 50%);",
            
            // Animated background elements
            div { 
                class: "fixed inset-0 pointer-events-none",
                style: "background: repeating-linear-gradient(45deg, transparent, transparent 10px, rgba(255,255,255,0.1) 10px, rgba(255,255,255,0.1) 20px); animation: slide 20s linear infinite;"
            }
            
            div { class: "max-w-6xl mx-auto bg-white/90 backdrop-blur-sm rounded-3xl p-8 shadow-2xl border-4 border-purple-300",
                style: "box-shadow: 0 0 30px rgba(147, 51, 234, 0.3), inset 0 0 30px rgba(255, 255, 255, 0.1);",
                
                // Header with vintage styling
                div { class: "text-center mb-12",
                    div { class: "inline-block bg-gradient-to-r from-purple-600 via-pink-500 to-yellow-400 p-2 rounded-full mb-4",
                        style: "box-shadow: 0 0 20px rgba(147, 51, 234, 0.5);"
                    }
                    h1 { 
                        class: "text-6xl font-bold mb-3 bg-gradient-to-r from-purple-600 via-pink-500 to-yellow-400 bg-clip-text text-transparent",
                        style: "text-shadow: 3px 3px 0px rgba(0,0,0,0.2), -1px -1px 0px rgba(255,255,255,0.8); font-family: 'Comic Sans MS', cursive;",
                        "🔧 Hex to PNG Converter 🔧"
                    }
                    p { 
                        class: "text-2xl text-gray-700 font-bold", 
                        style: "text-shadow: 1px 1px 2px rgba(255,255,255,0.8); font-family: 'Comic Sans MS', cursive;",
                        "Convert hex strings to PNG images with style! ✨"
                    }
                }

                // Input Section with retro styling
                div { 
                    class: "bg-gradient-to-r from-blue-100 via-purple-100 to-pink-100 rounded-3xl p-8 mb-8 border-4 border-blue-300",
                    style: "box-shadow: inset 0 0 20px rgba(59, 130, 246, 0.2), 0 8px 32px rgba(0,0,0,0.1);",
                    
                    h3 { 
                        class: "text-3xl font-bold text-gray-800 mb-6 flex items-center gap-3 justify-center",
                        style: "text-shadow: 2px 2px 4px rgba(255,255,255,0.8); font-family: 'Comic Sans MS', cursive;",
                        "📝 Hex Input 📝"
                    }

                    textarea {
                        class: "w-full px-6 py-5 border-4 border-purple-300 rounded-2xl text-base transition-all duration-300 bg-white/95 shadow-inner focus:outline-none focus:border-pink-500 focus:ring-4 focus:ring-pink-200 focus:-translate-y-1 mb-6 min-h-[10rem] font-mono text-sm",
                        style: "box-shadow: inset 0 0 10px rgba(147, 51, 234, 0.2), 0 4px 15px rgba(0,0,0,0.1);",
                        placeholder: "Enter hex string here (e.g., 89504e470d0a1a0a0000000d49484452...)",
                        value: "{hex_input}",
                        oninput: move |e| hex_input.set(e.value()),
                    }

                    div { class: "text-center",
                        button {
                            class: "bg-gradient-to-r from-purple-500 via-pink-500 to-yellow-400 hover:from-purple-600 hover:via-pink-600 hover:to-yellow-500 text-white font-bold py-5 px-10 rounded-2xl transition-all duration-300 transform hover:-translate-y-2 hover:shadow-2xl text-xl",
                            style: "box-shadow: 0 8px 25px rgba(147, 51, 234, 0.4), inset 0 1px 0 rgba(255,255,255,0.3); text-shadow: 2px 2px 4px rgba(0,0,0,0.3); font-family: 'Comic Sans MS', cursive;",
                            onclick: move |_| {
                                spawn(async move {
                                    let hex_str = hex_input();
                                    if hex_str.is_empty() {
                                        error_message.set("Please enter a hex string".to_string());
                                        return;
                                    }

                                    // Clean the hex string - remove whitespace and convert to lowercase
                                    let cleaned_hex = hex_str.trim().to_lowercase();
                                    if cleaned_hex.is_empty() {
                                        error_message.set("Hex string is empty after cleaning".to_string());
                                        return;
                                    }

                                    // Find PNG header in the hex string
                                    let png_header = "89504e470d0a1a0a";
                                    let final_hex = if cleaned_hex.starts_with(png_header) {
                                        cleaned_hex
                                    } else if let Some(header_pos) = cleaned_hex.find(png_header) {
                                        // Extract everything from the PNG header onwards
                                        cleaned_hex[header_pos..].to_string()
                                    } else {
                                        // If no PNG header found, add it to the beginning
                                        format!("{}{}", png_header, cleaned_hex)
                                    };

                                    // Try to decode the hex string
                                    match decode(&final_hex) {
                                        Ok(png_bytes) => {
                                            // Verify it's valid PNG data by trying to decode it
                                            match ImageReader::new(Cursor::new(&png_bytes))
                                                .with_guessed_format()
                                                .and_then(|reader| reader.decode().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
                                            {
                                                Ok(_) => {
                                                    // Encode to base64 for display
                                                    let image_data = format!(
                                                        "data:image/png;base64,{}",
                                                        base64::encode(&png_bytes),
                                                    );
                                                    image_uri.set(image_data);
                                                    error_message.set(String::new());
                                                }
                                                Err(_) => {
                                                    error_message.set("Invalid PNG data in hex string - the hex may be corrupted or not contain valid PNG data".to_string());
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            error_message.set("Invalid hex string format - please check that the input contains valid hexadecimal characters".to_string());
                                        }
                                    }
                                });
                            },
                            "🔄 Convert to PNG! 🔄"
                        }
                    }

                    if !error_message().is_empty() {
                        div { 
                            class: "mt-6 p-6 bg-red-100 border-4 border-red-400 text-red-700 rounded-2xl text-center",
                            style: "box-shadow: inset 0 0 10px rgba(239, 68, 68, 0.2); font-family: 'Comic Sans MS', cursive; font-weight: bold;",
                            "⚠️ {error_message} ⚠️"
                        }
                    }
                }

                // Output Section with big centered image
                div { 
                    class: "bg-gradient-to-r from-green-100 via-emerald-100 to-teal-100 rounded-3xl p-8 mb-8 border-4 border-green-300",
                    style: "box-shadow: inset 0 0 20px rgba(34, 197, 94, 0.2), 0 8px 32px rgba(0,0,0,0.1);",
                    
                    h3 { 
                        class: "text-3xl font-bold text-gray-800 mb-6 flex items-center gap-3 justify-center",
                        style: "text-shadow: 2px 2px 4px rgba(255,255,255,0.8); font-family: 'Comic Sans MS', cursive;",
                        "🖼️ PNG Output 🖼️"
                    }

                    div { 
                        class: "flex justify-center items-center p-8",
                        if image_uri() == "data:image/png;base64," {
                            div { 
                                class: "text-gray-500 italic text-center text-xl",
                                style: "font-family: 'Comic Sans MS', cursive; text-shadow: 1px 1px 2px rgba(255,255,255,0.8);",
                                "🎨 Convert a hex string to see the PNG image here! 🎨"
                            }
                        } else {
                            div { 
                                class: "bg-white rounded-3xl p-6 shadow-2xl border-4 border-purple-300",
                                style: "box-shadow: 0 0 30px rgba(147, 51, 234, 0.4), inset 0 0 20px rgba(255,255,255,0.5);",
                                img {
                                    class: "max-w-full h-auto rounded-2xl",
                                    style: "max-height: 600px; min-height: 200px; box-shadow: 0 10px 30px rgba(0,0,0,0.3); image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
                                    src: "{image_uri}",
                                    alt: "Converted PNG image"
                                }
                            }
                        }
                    }
                }

                // Back to Home Button with vintage styling
                div { class: "text-center",
                    a {
                        href: "/",
                        class: "inline-block bg-gradient-to-r from-purple-500 via-pink-500 to-yellow-400 hover:from-purple-600 hover:via-pink-600 hover:to-yellow-500 text-white font-bold py-4 px-8 rounded-2xl transition-all duration-300 transform hover:-translate-y-1 hover:shadow-xl text-xl",
                        style: "box-shadow: 0 6px 20px rgba(147, 51, 234, 0.4), inset 0 1px 0 rgba(255,255,255,0.3); text-shadow: 2px 2px 4px rgba(0,0,0,0.3); font-family: 'Comic Sans MS', cursive;",
                        "🏠 Back to Home 🏠"
                    }
                }
            }
        }
    }
} 