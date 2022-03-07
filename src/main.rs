use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(App);
}

fn Terminal(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "terminal",
            Items {}
            Input {}
        }
    ))
}
fn Items(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "items" ,
            Item {},
            Item {},
            Item {},
        }
    ))
}
fn Item(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "item" ,
            Command {}
            Output {}
        }
    ))
}
fn Command(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "command",
            div { class: "host", "sweeks@laptop"}
            div { class: "prompt", "~/src/weaver3"}
            div { class: "run", "cargo test"}
            div { class: "time", "15:02:02"}
        }
    ))
}
fn Output(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "output",
            "   Compiling weaver3 v0.1.0 (/Users/sweeks/src/weaver3)
warning: `weaver3` (bin \"weaver3\" test) generated 7 warnings
    Finished test [unoptimized + debuginfo] target(s) in 2.39s
     Running unittests (target/debug/deps/weaver3-f412cbfb6a30a242)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s"
        }
    ))
}
fn Input(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "command input",
            div { class: "host", "sweeks@laptop"}
            div { class: "prompt", "~/src/weaver3"}
            input {
                r#type: "text",
                class: "run",
                autofocus: "true",
                ""
            }
            div { class: "time", "15:02:02"}
        }
    ))
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        style { [include_str!("./assets/synthwave.css")] }
        Terminal {}
    ))
}
