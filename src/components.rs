pub mod icons;

use dioxus::prelude::*;

use crate::components::icons::{HomeIcon, SettingsIcon};
use crate::utils::get_github_release;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "navbar", class: "px-7 bg-gray-800 shadow-lg rounded w-full",
            div { class: "flex",
                NavItem { text: "Home", href: "/", Icon: HomeIcon() }
                NavItem { text: "Settings", href: "/settings", Icon: SettingsIcon() }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
struct NavItemProps {
    text: String,
    href: String,
    Icon: Element,
}

#[component]
fn NavItem(props: NavItemProps) -> Element {
    rsx! {
        div { class: "flex-1 group",
            a {
                class: "flex items-end justify-center text-center mx-auto px-4 pt-2 w-full text-gray-400 group-hover:text-indigo-500",
                href: props.href,
                span { class: "flex flex-col items-center justify-center px-1 pt-1 pb-1",
                    {props.Icon},
                    span { class: "block text-xs pb-2", "{props.text}" }
                    span { class: "block w-5 mx-auto h-1 group-hover:bg-indigo-500 rounded-full" }
                }
            }
        }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        div { id: "header", class: "mx-auto text-red-200 px-2",
            h1 { class: "text-2xl sm:text-4xl font-bold text-center mt-8 text-red-500",
                "ReVanced APKs"
            }
            p { class: "text-center text-sm sm:text-base",
                "Download the latest ReVanced Apps. Open Source, Safe and Secure."
            }
        }
    }
}

#[component]
pub fn ReleaseCard() -> Element {
    let release = use_resource(move || get_github_release());
    let read_unchecked = &*release.read();
    match read_unchecked {
        Some(Ok(release)) => {
            let assets = &release.assets.iter().rev().collect::<Vec<_>>();
            rsx! {
                div { class: "shadow-lg shadow-blue-500 rounded-lg overflow-scroll",
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
