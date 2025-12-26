use dioxus::prelude::*;
use dioxus::web;
use dioxus::web::launch::launch_cfg;
use dioxus_toastr::{use_toast, ToastKind, ToastOptions, ToastProvider, ToastRequest};
use std::time::Duration;

const DEMO_CSS: &str = include_str!("demo.css");

const MESSAGES: &[&str] = &[
    "My name is Inigo Montoya. You killed my father. Prepare to die!",
    "Are you the six fingered man?",
    "Inconceivable!",
    "I do not think that means what you think it means.",
    "Have fun storming the castle!",
];

#[derive(Clone, Copy, PartialEq)]
enum Position {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
    TopFullWidth,
    BottomFullWidth,
    TopCenter,
    BottomCenter,
}

impl Position {
    fn label(self) -> &'static str {
        match self {
            Position::TopRight => "Top Right",
            Position::BottomRight => "Bottom Right",
            Position::BottomLeft => "Bottom Left",
            Position::TopLeft => "Top Left",
            Position::TopFullWidth => "Top Full Width",
            Position::BottomFullWidth => "Bottom Full Width",
            Position::TopCenter => "Top Center",
            Position::BottomCenter => "Bottom Center",
        }
    }

    fn class_name(self) -> &'static str {
        match self {
            Position::TopRight => "toast-top-right",
            Position::BottomRight => "toast-bottom-right",
            Position::BottomLeft => "toast-bottom-left",
            Position::TopLeft => "toast-top-left",
            Position::TopFullWidth => "toast-top-full-width",
            Position::BottomFullWidth => "toast-bottom-full-width",
            Position::TopCenter => "toast-top-center",
            Position::BottomCenter => "toast-bottom-center",
        }
    }
}

fn main() {
    let cfg = web::Config::new().rootname("app-root");
    launch_cfg(App, cfg);
}

#[component]
fn App() -> Element {
    let mut title = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut toast_kind = use_signal(|| ToastKind::Success);
    let mut position = use_signal(|| Position::TopRight);
    let mut close_button = use_signal(|| false);
    let mut progress_bar = use_signal(|| false);
    let mut rtl = use_signal(|| false);
    let mut prevent_duplicates = use_signal(|| false);
    let mut newest_on_top = use_signal(|| true);
    let mut tap_to_dismiss = use_signal(|| true);
    let mut time_out = use_signal(|| 5000_u64);
    let mut extended_time_out = use_signal(|| 1000_u64);
    let last_toast_id = use_signal(|| None::<u64>);
    let message_index = use_signal(|| 0_usize);

    let options = {
        let mut options = ToastOptions::default();
        options.close_button = *close_button.read();
        options.progress_bar = *progress_bar.read();
        options.rtl = *rtl.read();
        options.prevent_duplicates = *prevent_duplicates.read();
        options.newest_on_top = *newest_on_top.read();
        options.tap_to_dismiss = *tap_to_dismiss.read();
        options.position_class = position.read().class_name();
        options.time_out = Duration::from_millis(*time_out.read());
        options.extended_time_out = Duration::from_millis(*extended_time_out.read());
        options
    };

    let options_preview = format!(
        "toast[\"{:?}\"](\"{}\", \"{}\")\n\nToastOptions {{\n  close_button: {},\n  progress_bar: {},\n  rtl: {},\n  prevent_duplicates: {},\n  newest_on_top: {},\n  tap_to_dismiss: {},\n  position_class: \"{}\",\n  time_out: {}ms,\n  extended_time_out: {}ms,\n}}",
        *toast_kind.read(),
        message.read(),
        title.read(),
        close_button.read(),
        progress_bar.read(),
        rtl.read(),
        prevent_duplicates.read(),
        newest_on_top.read(),
        tap_to_dismiss.read(),
        position.read().class_name(),
        time_out.read(),
        extended_time_out.read(),
    );

    rsx! {
        ToastProvider {
            options: options,
            document::Style { "{DEMO_CSS}" }

            div { class: "demo",
                h1 { "toastr demo" }
                p { "A Dioxus example inspired by the original toastr demo page." }

                div { class: "demo-grid",
                    div { class: "panel",
                        h2 { "Content" }
                        div { class: "field",
                            label { r#for: "title", "Title" }
                            input {
                                id: "title",
                                r#type: "text",
                                placeholder: "Enter a title...",
                                value: "{title.read()}",
                                oninput: move |evt| title.set(evt.value()),
                            }
                        }
                        div { class: "field",
                            label { r#for: "message", "Message" }
                            textarea {
                                id: "message",
                                placeholder: "Enter a message...",
                                value: "{message.read()}",
                                oninput: move |evt| message.set(evt.value()),
                            }
                        }
                    }

                    div { class: "panel",
                        h2 { "Behavior" }
                        div { class: "options",
                            label {
                                input {
                                    r#type: "checkbox",
                                    checked: *close_button.read(),
                                    oninput: move |evt| close_button.set(evt.checked()),
                                }
                                "Close Button"
                            }
                            label {
                                input {
                                    r#type: "checkbox",
                                    checked: *progress_bar.read(),
                                    oninput: move |evt| progress_bar.set(evt.checked()),
                                }
                                "Progress Bar"
                            }
                            label {
                                input {
                                    r#type: "checkbox",
                                    checked: *tap_to_dismiss.read(),
                                    oninput: move |evt| tap_to_dismiss.set(evt.checked()),
                                }
                                "Tap to Dismiss"
                            }
                            label {
                                input {
                                    r#type: "checkbox",
                                    checked: *rtl.read(),
                                    oninput: move |evt| rtl.set(evt.checked()),
                                }
                                "Right-to-Left"
                            }
                            label {
                                input {
                                    r#type: "checkbox",
                                    checked: *prevent_duplicates.read(),
                                    oninput: move |evt| prevent_duplicates.set(evt.checked()),
                                }
                                "Prevent Duplicates"
                            }
                            label {
                                input {
                                    r#type: "checkbox",
                                    checked: *newest_on_top.read(),
                                    oninput: move |evt| newest_on_top.set(evt.checked()),
                                }
                                "Newest on Top"
                            }
                        }
                    }

                    div { class: "panel",
                        h2 { "Toast Type" }
                        div { class: "radio-group",
                            label {
                                input {
                                    r#type: "radio",
                                    name: "toast-kind",
                                    checked: *toast_kind.read() == ToastKind::Success,
                                    onclick: move |_| toast_kind.set(ToastKind::Success),
                                }
                                "Success"
                            }
                            label {
                                input {
                                    r#type: "radio",
                                    name: "toast-kind",
                                    checked: *toast_kind.read() == ToastKind::Info,
                                    onclick: move |_| toast_kind.set(ToastKind::Info),
                                }
                                "Info"
                            }
                            label {
                                input {
                                    r#type: "radio",
                                    name: "toast-kind",
                                    checked: *toast_kind.read() == ToastKind::Warning,
                                    onclick: move |_| toast_kind.set(ToastKind::Warning),
                                }
                                "Warning"
                            }
                            label {
                                input {
                                    r#type: "radio",
                                    name: "toast-kind",
                                    checked: *toast_kind.read() == ToastKind::Error,
                                    onclick: move |_| toast_kind.set(ToastKind::Error),
                                }
                                "Error"
                            }
                        }
                    }

                    div { class: "panel",
                        h2 { "Position" }
                        div { class: "radio-group",
                            for pos in [
                                Position::TopRight,
                                Position::BottomRight,
                                Position::BottomLeft,
                                Position::TopLeft,
                                Position::TopFullWidth,
                                Position::BottomFullWidth,
                                Position::TopCenter,
                                Position::BottomCenter,
                            ] {
                                label {
                                    input {
                                        r#type: "radio",
                                        name: "toast-position",
                                        checked: *position.read() == pos,
                                        onclick: move |_| position.set(pos),
                                    }
                                    "{pos.label()}"
                                }
                            }
                        }
                    }

                    div { class: "panel",
                        h2 { "Timing" }
                        div { class: "field",
                            label { r#for: "timeout", "Time out (ms)" }
                            input {
                                id: "timeout",
                                r#type: "number",
                                value: "{time_out.read()}",
                                oninput: move |evt| {
                                    let value = evt.value().parse().unwrap_or(5000_u64);
                                    time_out.set(value);
                                },
                            }
                        }
                        div { class: "field",
                            label { r#for: "extendedTimeout", "Extended time out (ms)" }
                            input {
                                id: "extendedTimeout",
                                r#type: "number",
                                value: "{extended_time_out.read()}",
                                oninput: move |evt| {
                                    let value = evt.value().parse().unwrap_or(1000_u64);
                                    extended_time_out.set(value);
                                },
                            }
                        }
                    }
                }

                Controls {
                    title: title,
                    message: message,
                    toast_kind: toast_kind,
                    last_toast_id: last_toast_id,
                    message_index: message_index,
                }

                pre { class: "options-preview", "{options_preview}" }

                footer { class: "links",
                    h2 { "Links" }
                    ul {
                        li { a { href: "https://crates.io/crates/dioxus-toastr", "Crates.io" } }
                        li { a { href: "https://github.com/CodeSeven/toastr", "Original toastr" } }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ControlsProps {
    title: Signal<String>,
    message: Signal<String>,
    toast_kind: Signal<ToastKind>,
    last_toast_id: Signal<Option<u64>>,
    message_index: Signal<usize>,
}

#[component]
fn Controls(mut props: ControlsProps) -> Element {
    let toast = use_toast();
    let toast_for_show = toast.clone();
    let toast_for_clear = toast.clone();
    let toast_for_remove = toast.clone();

    rsx! {
        div { class: "controls",
            button {
                class: "primary",
                onclick: move |_| {
                    let mut index = *props.message_index.read();
                    let fallback_message = MESSAGES[index].to_string();
                    index = (index + 1) % MESSAGES.len();
                    props.message_index.set(index);

                    let message = props.message.read();
                    let message = if message.trim().is_empty() {
                        fallback_message
                    } else {
                        message.to_string()
                    };

                    let mut request = ToastRequest::new(*props.toast_kind.read(), message);
                    let title = props.title.read();
                    if !title.trim().is_empty() {
                        request = request.with_title(title.to_string());
                    }

                    let id = toast_for_show.push(request);
                    if id != 0 {
                        props.last_toast_id.set(Some(id));
                    }
                },
                "Show Toast"
            }
            button {
                class: "secondary",
                onclick: move |_| toast_for_clear.clear(),
                "Clear Toasts"
            }
            button {
                class: "secondary",
                disabled: props.last_toast_id.read().is_none(),
                onclick: move |_| {
                    let last_id = *props.last_toast_id.read();
                    if let Some(id) = last_id {
                        toast_for_remove.remove(id);
                    }
                    if last_id.is_some() {
                        props.last_toast_id.set(None);
                    }
                },
                "Clear Last Toast"
            }
        }
    }
}
