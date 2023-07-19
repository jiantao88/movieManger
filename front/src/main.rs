#![allow(non_snake_case)]

mod components;
use components::{Footer, Header};
use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));

    log::info!("Message on my console12");

    dioxus_web::launch(App);


}
// Define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {

       main {
           class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
           Header {}
           section {
               class: "md:container md:mx-auto md:py-8 flex-1",
           }
           Footer {}
       }
    })
}

pub fn MyComponent(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "my-component",
            "data-my-attribute": "my value",
            "My component"
        }
    ))
}
