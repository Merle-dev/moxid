use anyhow::Result;
use katex::{render_with_opts, Opts};
use leptos::{logging::*, prelude::*};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use styled::style;
use wasm_bindgen::prelude::*;
use web_sys::console::info;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn call_tauri<R: DeserializeOwned>(
    cmd: &str,
    input: impl Serialize,
) -> Result<R, serde_wasm_bindgen::Error> {
    let result = invoke(cmd, serde_wasm_bindgen::to_value(&input)?).await;
    serde_wasm_bindgen::from_value::<R>(result)
}

#[component]
pub fn App() -> impl IntoView {
    let path = String::from("./");
    log!("starting wasm app");
    view! {
        <div class="container">
            <SideBar path=path />
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

#[derive(Deserialize, Serialize)]
struct FileRequest {
    path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct File {
    name: String,
    directory: bool,
}

#[component]
pub fn SideBar(path: String) -> impl IntoView {
    let (path, set_path) = signal(path);
    let data = LocalResource::new(move || async move {
        call_tauri::<Vec<File>>("files", FileRequest { path: path.get() })
            .await
            .unwrap()
    });
    view! {
        <aside class="sidebar">
                <div class="sidebar-header">
                    FILES
                </div>
                <div class="sidebar-container">
                <button on:click=move |_| set_path.set(format!("../"))>"<"</button>
                {
                    move || match data.get() {
                        Some(result) => result.into_iter().map(|file| view! { <SideBarItem file setter=set_path /> }).collect_view().into_any(),
                        None => view! { <p> "loading" </p>}.into_any()
                    }
                }
                </div>
            </aside>


    }
}

#[component]
pub fn SideBarItem(file: File, setter: WriteSignal<String>) -> impl IntoView {
    let icon = if file.directory { "📁" } else { "📄" };
    let name = file.name.clone();
    let click = move |_| {
        if file.directory {
            log!("cd to {}", file.name);
            setter.set(file.name.clone());
        }
    };
    view! {
        <div class="sidebar-item" on:click=click>
            {icon}
            {name}
        </div>
    }
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
