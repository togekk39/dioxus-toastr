# Dioxus Toastr

This crate provides a Dioxus-based toast notification system inspired by [Toastr](https://github.com/CodeSeven/toastr), enabling Rust/Dioxus apps to show typed, reactive notifications.

## Usage Example

```rust
use dioxus::prelude::*;
use dioxus_toastr::{ToastProvider, use_toast};

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
```

## Development Environment

### Requirements

- Rust stable toolchain (recommended via rustup)
- Cargo
- [Trunk](https://trunkrs.dev/) (for serving with `trunk serve`)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.5/getting_started/cli) (for serving with `dx serve`)

### Setup

```bash
rustup update stable
```

Enter the crate directory:

```bash
cd dioxus_toastr
```

Run a compile check:

```bash
cargo check
```

## Run the example in a browser

### Using Trunk

1. Install Trunk (if needed):

   ```bash
   cargo install trunk
   ```

2. Start the dev server:

   ```bash
   trunk serve --example minimal
   ```

3. Open the printed local URL (typically <http://127.0.0.1:8080>) in your browser.

> Note: run the command from the project root so Trunk can find `index.html`.

### Using Dioxus CLI

1. Install the CLI (if needed):

   ```bash
   cargo install dioxus-cli
   ```

2. Start the dev server:

   ```bash
   dx serve --example minimal
   ```

3. Open the printed local URL in your browser.

## Development Workflow

1. **Edit modules**: Update the relevant files under `src/`.
2. **Compile check**: Run `cargo check` to validate types and syntax.
3. **Integration test**: Use `dioxus_toastr` in your Dioxus app to verify rendering and interactions.

Keep commits focused and scoped to a single feature or fix.

## Architecture

```
dioxus_toastr/
├── Cargo.toml
├── README.md
└── src/
    ├── handle.rs   # ToastHandle and use_toast API
    ├── lib.rs      # Public re-exports
    ├── provider.rs # ToastProvider and ToastItem components
    ├── store.rs    # ToastStore (signals + auto-dismiss)
    └── types.rs    # Toast/ToastRequest/ToastOptions/ToastKind
```

### Module Overview

- **types.rs**
  - Defines the toast types and data structures: `ToastKind`, `ToastOptions`, `ToastRequest`, and `Toast`.
- **store.rs**
  - Implements `ToastStore` using Dioxus `Signal`s to manage toasts and options, including auto-dismiss.
- **handle.rs**
  - Exposes `ToastHandle` and `use_toast()` with convenience helpers for `success/info/warning/error`.
- **provider.rs**
  - Renders the toast container via `ToastProvider` and handles user interactions in `ToastItem`.
- **lib.rs**
  - Re-exports the public API surface.
