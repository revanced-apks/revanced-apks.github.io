#![allow(non_snake_case)]

mod components;
mod utils;

use dioxus::prelude::*;
use log::LevelFilter;

use crate::components::{Header, NavBar, ReleaseCard};

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(|| rsx! { Router::<Route> {} });
}

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[layout(BasicLayout)]
    #[route("/")]
    Home {},
    #[end_layout]
    //  if the current location doesn't match any of the above routes, render the NotFound component
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let route = segments.join("/");
    rsx! {
        HeadTags {}
        div { class: "container flex flex-col items-center justify-center mx-auto p-2 text-gray-300 gap-4",
            h1 { class: "text-4xl font-bold text-red-500", "404" }
            p { class: "text-sm sm:text-base", "Page {route} not found" }
            Link {
                to: Route::Home {},
                class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                "Go Home"
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Header {}
        div { class: "flex flex-col items-center justify-center", ReleaseCard {} }
    }
}

#[component]
fn HeadTags() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        link { rel: "stylesheet", href: "tailwind.css" }
        link {
            rel: "stylesheet",
            href: "//fonts.googleapis.com/css?family=Poppins"
        }
    }
}

#[component]
fn BasicLayout() -> Element {
    rsx! {
        HeadTags {}
        div {
            id: "content",
            class: "container mx-auto px-2 text-gray-300 h-[calc(100vh-74px)] overflow-auto scroll-smooth",
            Outlet::<Route> {}
        }
        NavBar {}
    }
}
