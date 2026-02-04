use anyhow::Result;
use katex::{render_with_opts, Opts};
use leptos::leptos_dom::logging::console_log;
use leptos::task::spawn_local;
use leptos::{leptos_dom::logging, prelude::*};
use leptos_reactive::{create_local_resource, SignalGet};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use styled::style;
use wasm_bindgen::prelude::*;
use web_sys::console::info;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke_c(cmd: &str, args: JsValue) -> JsValue;
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

async fn invoke<R: DeserializeOwned>(
    cmd: &str,
    input: impl Serialize,
) -> Result<R, serde_wasm_bindgen::Error> {
    let result = invoke_c(cmd, serde_wasm_bindgen::to_value(&input)?).await;
    serde_wasm_bindgen::from_value::<R>(result)
}

#[component]
pub fn App() -> impl IntoView {
    let path = String::from("./");
    view! {
        <div class="container">
            <SideBar path />
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
}

#[component]
pub fn SideBar(path: String) -> impl IntoView {
    let (path, set_path) = create_signal(path);
    // let fetch_files = |path| async move {
    //     invoke::<Vec<File>>("files", FileRequest { path })
    //         .await
    //         .unwrap()
    // };
    let resource = create_local_resource(
        move || path.get(),
        move |path| async move {
            match invoke::<Vec<File>>("files", FileRequest { path }).await {
                Ok(files) => {
                    console_log(&format!("Fetched {} files", files.len()));
                    files
                }
                Err(e) => {
                    log::error!("Error fetching files: {:?}", e);
                    vec![]
                }
            }
        },
    );
    view! {
            <aside class="sidebar">
                <div class="sidebar-header">
                    FILES
                </div>
                <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                    <div class="sidebar-content">
                    {move || resource.get().map(|files: Vec<File>| {
                        files.iter().enumerate()
                            .map(|(n, file)| view! { <li class="sidebar-item">{n}</li> })
                            .collect_view()
                        })
                    }
                    </div>
                </Suspense>
            </aside>

    }
}
