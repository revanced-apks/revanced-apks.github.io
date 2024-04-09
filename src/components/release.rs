use dioxus::prelude::*;

use crate::utils::get_github_release;

#[component]
pub fn ReleaseCard() -> Element {
    let release = use_resource(move || get_github_release());
    let read_unchecked = &*release.read();
    match read_unchecked {
        Some(Ok(release)) => {
            let assets = &release.assets.iter().rev().collect::<Vec<_>>();
            rsx! {
                div { class: "shadow-lg shadow-blue-500 rounded-lg overflow-auto",
                    div { class: "px-6 py-4",
                        h1 { class: "font-bold text-xl mb-2", "{release.name}" }
                        // p { class: "text-gray text-base", "{release.body}" },
                        div {
                            for asset in assets.iter() {
                                div { class: "flex items-center",

                                    a {
                                        class: "text-blue-500 hover:text-blue-700 text-base",
                                        href: asset.browser_download_url.as_str(),
                                        "{asset.name}"
                                    }
                                }
                            }
                        }
                    }
                    div { class: "bg-blue-600 hover:bg-blue-800 cursor-pointer py-2 px-4 rounded flex flex-row  items-center justify-center gap-2",
                        img {
                            class: "w-6 h-6",
                            src: "https://cdn-icons-png.flaticon.com/512/25/25231.png"
                        }
                        a { class: " text-white font-bold", href: release.html_url.as_str(), "Release" }
                    }
                }
            }
        }
        Some(Err(err)) => {
            rsx! {
                div { class: "bg-white shadow-lg rounded-lg overflow-hidden",
                    div { class: "px-6 py-4",
                        h1 { class: "font-bold text-xl mb-2", "Error" }
                        p { class: "text-gray text-base", "{err}" }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "bg-white shadow-lg rounded-lg overflow-hidden",
                    div { class: "px-6 py-4",
                        h1 { class: "font-bold text-xl mb-2", "Loading" }
                        p { class: "text-gray text-base", "Fetching release..." }
                    }
                }
            }
        }
    }
}
