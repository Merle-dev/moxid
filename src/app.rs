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

type PathReader = ReadSignal<(Vec<String>, PathType)>;
type PathWriter = WriteSignal<(Vec<String>, PathType)>;

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

pub fn get(path: Vec<String>) -> String {
    path.iter().map(|s| format!("/{s}")).collect::<String>()
}

#[component]
pub fn App() -> impl IntoView {
    log!("starting wasm app");
    let (path_reader, path_writer) = signal((Vec::<String>::new(), PathType::Directory));
    spawn_local(async move {
        path_writer.set((new().await, PathType::Directory));
    });
    let a = LocalResource::new(|| async move { (new().await, PathType::Directory) });
    view! {
        <div class="container">
            <SideBar path_reader path_writer />
            <main class="main-content">
            <div class="editor-header">
                {
                    move || match a.get().and_then(|(v, _)| v.last().cloned()) {
                        Some(p) => view! { <span class="editor-title"> { p } </span> }.into_any(),
                        None =>  view! { <span class="editor-title">"loading"</span> }.into_any(),
                    }
                }
            </div>
            <Editor path_reader/>
        </main>
        </div>
    }
}

#[component]
pub fn Editor(path_reader: PathReader) -> impl IntoView {
    let file = LocalResource::new(move || async move {
        if path_reader.get().1 != PathType::File {
            None
        } else if let Some(file_path) = path_reader.get().0.last().cloned() {
            call_tauri::<Option<String>>("file", FileRequest { path: file_path })
                .await
                .unwrap_or(None)
        } else {
            None
        }
    });
    view! {
        {
            move || match file.get() {
                Some(file) => view! {
                     <div class="editor-container">
                        <textarea class="text-field" placeholder="Start typing your notes here...">{file}</textarea>
                    </div>
                }.into_any(),
                None => view! { <p> No File Loaded </p> }.into_any(),
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
enum PathType {
    Directory,
    File,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FileRequest {
    path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct File {
    name: String,
    // path: Vec<String>,
    path_type: PathType,
}

#[component]
pub fn SideBar(path_reader: PathReader, path_writer: PathWriter) -> impl IntoView {
    let (files, set_files) = signal(vec![]);
    // Use a Localresource
    spawn_local(async move {
        let (path, path_type) = (move || path_reader.get())();
        log!("{:?}", path);
        if path_type == PathType::Directory {
            if let Ok(Some(new_files)) =
                call_tauri::<Option<Vec<File>>>("files", FileRequest { path: get(path) }).await
            {
                set_files.set(new_files);
            }
        }
    });
    view! {
        <aside class="sidebar">
            <div class="sidebar-header">
                <button class="sidebar-back-button" on:click=move |_| path_writer.update(|(path, _)| if path.len() > 2 { path.pop(); })>"◀"</button>
                <h3 class="sidebar-header-name"> "Files" </h3>
            </div>
            <div class="sidebar-container">
                {
                    move || files.get().into_iter().map(|file| view! { <SideBarItem file path_writer /> }).collect_view()
                }
            </div>
        </aside>
    }
}

#[component]
pub fn SideBarItem(file: File, path_writer: PathWriter) -> impl IntoView {
    let icon = match file.path_type {
        PathType::Directory => "📁",
        PathType::File => "📄",
    };
    let name = file.name.clone();
    let click = move |_| {
        path_writer.update(|(v, path_type)| {
            v.push(file.name.clone());
            *path_type = file.path_type.clone();
        });
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
