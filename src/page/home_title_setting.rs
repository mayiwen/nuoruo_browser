use crate::myw;
use leptos::prelude::*;
/// 搜索图标组件
#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style="right: 160px; top: 1px" class="pa">
            <myw::button::Button w=44 border="both" on_click=|_|{}> "" <myw::icon::Minimize /></myw::button::Button>
        </div>
    }
}
