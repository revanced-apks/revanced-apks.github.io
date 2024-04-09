pub mod icons;
pub mod release;

use dioxus::prelude::*;

use crate::components::icons::{HomeIcon, SettingsIcon};

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
                span { class: "flex flex-col items-center justify-center px-1 ",
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
