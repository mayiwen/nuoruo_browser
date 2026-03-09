use crate::myw;
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
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

// 1. 定义导航函数的类型（入参是 &'static str，无返回值）
pub type GlobalNavFn = Callback<&'static str, ()>;
// 2. 定义上下文的唯一 Key（避免和其他上下文冲突）
// 定义导航函数的类型别名（入参是 &'static str）
pub type NavFn = Box<dyn Fn(&'static str) + Send + Sync>;

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };

    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }

    //         let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };

    view! {
        <div style="height: 100%">
            <div style="height: 40px;" class="lh-40 pr" data-tauri-drag-region>
                <div style="padding-left: 8px; cursor: move;"  data-tauri-drag-region>  "诺若浏览器"</div>
                <crate::page::home_title_control::HomeTitleControl/>
                <crate::page::home_title_setting::I/>
                // <div style="right: 0; top: 1px" class="pa">
                //     <myw::button::Button w=44 border="none" on_click=|_|{}> "" <myw::icon::Minimize /></myw::button::Button>
                //     <myw::button::Button w=44 border="none" on_click=|_|{}> "" <myw::icon::Maximize /></myw::button::Button>
                //     <myw::button::Button w=44 border="none" on_click=|_|{}> "" <myw::icon::Closer /></myw::button::Button>
                // </div>
            </div>
        </div>
        // <main class="container">
        // <myw::icon::Myw/>


            // <h1>"Welcome to Tauri + Leptos"</h1>

            // <div class="row">
            //     <a href="https://tauri.app" target="_blank">
            //         <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
            //     </a>
            //     <a href="https://docs.rs/leptos/" target="_blank">
            //         <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
            //     </a>
            // </div>
            // <p>"Click on the Tauri and Leptos logos to learn more."</p>

            // <form class="row" on:submit=greet>
            //     <input
            //         id="greet-input"
            //         placeholder="Enter a name..."
            //         on:input=update_name
            //     />
            //     <button type="submit">"Greet"</button>
            // </form>
            // <p>{ move || greet_msg.get() }</p>
        // </main>
    }
}
