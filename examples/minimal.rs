use dioxus::prelude::*;
use dioxus_toastr::{use_toast, ToastProvider};

fn main() {
    launch(App);
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
