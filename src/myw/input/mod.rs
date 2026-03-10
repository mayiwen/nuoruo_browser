use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(optional, into)] placeholder: Signal<String>,
    // 直接传数字（如 200），自动转成 px
    #[prop(optional, into)] w: Signal<usize>,
    #[prop(optional, into)] width: Signal<usize>,
) -> impl IntoView {
    // 优先 w，否则用 width，都没有就默认 200px
    let final_width = move || {
        let w_val = w.get();
        if w_val != 0 {
            format!("{}px", w_val)
        } else {
            let width_val = width.get();
            if width_val != 0 {
                format!("{}px", width_val)
            } else {
                "100px".to_string()
            }
        }
    };

    view! {
        <input
            class="myw-input"
            placeholder=placeholder
            // ✅ 唯一能生效的宽度写法
            style:width=final_width
        />
    }
}
