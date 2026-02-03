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
    // Configure KaTeX options
    let opts = Opts::builder().display_mode(display_mode).build().unwrap();

    let html = render_with_opts(&formula, &opts)
        .unwrap_or_else(|_| format!("Error rendering: {}", formula));

    // Use inner_html to inject the raw KaTeX output
    view! {
        <span class="formula" inner_html=html />
    }
}
#[component]
pub fn App() -> impl IntoView {
    let (latex_input, set_latex_input) =
        signal(String::from(r"\frac{-b \pm \sqrt{b^2 - 4ac}}{2a}"));
    let (rendered_output, set_rendered_output) = signal(String::new());

    // Trigger rendering whenever input changes
    // Effect::new(move |_| {
    //     let input = latex_input.get();

    //     spawn_local(async move {
    //         // render_katex(&input);
    //     });
    // });

    view! {
        <div class="container">
            <SideBar />
            <Latex formula=r#"-p\pm \sqrt{(\frac{p}{2})^2-q}"# display_mode=true />
        </div>
    }
}

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        <div class="side_bar">
            {
                (1..10)
                    .into_iter()
                    .map(|n| view!{ <li>{n}</li> })
                    .collect_view()

            }
        </div>
    }
}

// #[wasm_bindgen(inline_js = r#"
// export function render_katex(latex) {
//     try {
//         const output = document.getElementById('latex-output');
//         if (output && window.katex) {
//             katex.render(latex, output, {
//                 throwOnError: false,
//                 displayMode: true
//             });
//         }
//     } catch (e) {
//         console.error('KaTeX rendering error:', e);
//     }
// }
// "#)]
// extern "C" {
//     fn render_katex(latex: &str);
// }
