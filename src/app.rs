use anyhow::Result;
use leptos::{logging::*, prelude::*, reactive::spawn_local};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
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
mod editor_path {
    use leptos::prelude::*;

    use crate::app::{invoke, invoke_without_args};

    pub async fn new() -> Vec<String> {
        let mut path: Vec<String> =
            serde_wasm_bindgen::from_value::<String>(invoke_without_args("directory").await)
                .unwrap()
                .split('/')
                .map(String::from)
                .collect();
        path.remove(0);
        leptos::logging::log!("{:?}", path);
        path
    }
    pub fn add(path: WriteSignal<Vec<String>>, addition: String) {
        path.update(|v| v.push(addition));
    }
    pub fn pop(path: WriteSignal<Vec<String>>) {
        path.update(|v| {
            v.pop();
        });
    }
    pub fn get(path: ReadSignal<Vec<String>>) -> String {
        path.get()
            .iter()
            .map(|s| format!("/{s}"))
            .collect::<String>()
    }
}

#[component]
pub fn App() -> impl IntoView {
    log!("starting wasm app");
    let (path, set_path) = signal(Vec::<String>::new());
    spawn_local(async move {
        set_path.set(editor_path::new().await);
    });
    let a = LocalResource::new(|| async move { editor_path::new().await });
    view! {
        <div class="container">
            <SideBar path set_path />
            <main class="main-content">
            <div class="editor-header">
                {
                    move || match a.get().and_then(|v| v.last().cloned()) {
                        Some(p) => view! { <span class="editor-title"> { p } </span> }.into_any(),
                        None =>  view! { <span class="editor-title">"loading"</span> }.into_any(),
                    }
                }
            </div>
            <div class="editor-container">
                <textarea class="text-field" placeholder="Start typing your notes here..."></textarea>
            </div>
        </main>
        </div>
    }
}

#[derive(Deserialize, Serialize)]
struct FileRequest {
    path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct File {
    name: String,
    // path: Vec<String>,
    directory: bool,
}

#[component]
pub fn SideBar(path: ReadSignal<Vec<String>>, set_path: WriteSignal<Vec<String>>) -> impl IntoView {
    let data = LocalResource::new(move || async move {
        call_tauri::<Vec<File>>(
            "files",
            FileRequest {
                path: editor_path::get(path),
            },
        )
        .await
        .unwrap()
    });
    view! {
        <aside class="sidebar">
                <div class="sidebar-header">
                    FILES
                </div>
                <div class="sidebar-container">
                <button on:click=move |_| editor_path::pop(set_path)>"Pop"</button>
                <button on:click=move |_| set_path.set((0..10).into_iter().map(|n| n.to_string()).collect())>"Load"</button>
                {
                    move || match data.get() {
                        Some(result) => result.into_iter().map(|file| view! { <SideBarItem file path=set_path /> }).collect_view().into_any(),
                        None => view! { <p> "loading" </p>}.into_any()
                    }
                }
                </div>
            </aside>


    }
}

#[component]
pub fn SideBarItem(file: File, path: WriteSignal<Vec<String>>) -> impl IntoView {
    let icon = if file.directory { "📁" } else { "📄" };
    let name = file.name.clone();
    let click = move |_| {
        log!("cd to {}", file.name);
        editor_path::add(path, file.name.clone());
    };
    view! {
        <div class="sidebar-item" on:click=click>
            {icon}
            {name}
        </div>
    }
}

// #[component]
// pub fn Latex(#[prop(into)] formula: String, #[prop(optional)] display_mode: bool) -> impl IntoView {
//     let opts = Opts::builder().display_mode(display_mode).build().unwrap();

//     let html = render_with_opts(&formula, &opts)
//         .unwrap_or_else(|_| format!("Error rendering: {}", formula));

//     view! {
//         <span class="formula" inner_html=html />
//     }
// }
