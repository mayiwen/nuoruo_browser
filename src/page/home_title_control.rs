use crate::myw;
use leptos::prelude::*;
/// 搜索图标组件
#[component]
pub fn HomeTitleControl() -> impl IntoView {
    view! {
        <div style="right: 0; top: 1px" class="pa">
            <myw::button::Button w=44 border="none" on_click=|_|{}> "" <myw::icon::Minimize /></myw::button::Button>
            <myw::button::Button w=44 border="none" on_click=|_|{}> "" <myw::icon::Maximize /></myw::button::Button>
            <myw::button::Button w=44 border="none" on_click=|_|{}> "" <myw::icon::Closer /></myw::button::Button>
        </div>
    }
}
