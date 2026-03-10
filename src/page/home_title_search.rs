use crate::myw;
use leptos::prelude::*;
/// 搜索图标组件
#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style="left: 4px;top: -2px;" class="pa">
            <myw::input::Input placeholder="请输入网址" w=188/>
        </div>
    }
}
