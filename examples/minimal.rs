use dioxus::prelude::*;
use dioxus_toastr::{use_toast, ToastProvider};
use dioxus::web;
use dioxus::web::launch::launch_cfg;

fn main() {
    let cfg = web::Config::new().rootname("app-root");
    launch_cfg(App, cfg);
}

#[component]
fn App() -> Element {
    rsx! {
        ToastProvider {
            Demo {}
        }
    }
}

#[component]
fn Demo() -> Element {
    let toast = use_toast();

    rsx! {
        button {
            onclick: move |_| {
                toast.success("Saved!", "Your changes are now live.");
            },
            "Show toast"
        }
    }
}
