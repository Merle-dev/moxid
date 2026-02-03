use katex::{render_with_opts, Opts};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use styled::style;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn Latex(#[prop(into)] formula: String, #[prop(optional)] display_mode: bool) -> impl IntoView {
    let opts = Opts::builder().display_mode(display_mode).build().unwrap();

    let html = render_with_opts(&formula, &opts)
        .unwrap_or_else(|_| format!("Error rendering: {}", formula));

    view! {
        <span class="formula" inner_html=html />
    }
}
#[component]
pub fn App() -> impl IntoView {
    use leptos::*;
    view! {
        <div class="container">
            <SideBar />
            <main class="main-content">
            <div class="editor-header">
                <span class="editor-title">Welcome Note</span>
            </div>
            <div class="editor-container">
                <textarea class="text-field" placeholder="Start typing your notes here..."></textarea>
            </div>
        </main>
            // <Latex formula=r#"-p\pm \sqrt{(\frac{p}{2})^2-q}"# display_mode=true />
        </div>
    }
}

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
            <aside class="sidebar">
                <div class="sidebar-header">
                    FILES
                </div>
                <div class="sidebar-content"> {
                    (1..10)
                        .into_iter()
                        .map(|n| view!{ <li class="sidebar-item">{n}</li> })
                        .collect_view()

                }
                </div>
            </aside>

    }
}
