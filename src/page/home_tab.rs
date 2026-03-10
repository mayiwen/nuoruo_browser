use crate::myw;
use leptos::prelude::*;
/// 搜索图标组件
#[component]
pub fn I() -> impl IntoView {
    view! {
        <div style="height: 40px; padding-top: 1px; padding-left: 4px;" class="lh-40 pr">
             <myw::button::Button w=44 border="both" on_click=|_|{}>
                <myw::icon::Icon class="ri-arrow-left-line"/>
             </myw::button::Button>
             <myw::Gap w=4/>
             <myw::button::Button w=44 border="both" on_click=|_|{}>
                <myw::icon::Icon class="ri-arrow-right-line"/>
             </myw::button::Button>
             <myw::Gap w=4/>
             <myw::button::Button w=44 border="both" on_click=|_|{}>
                <myw::icon::Icon class="ri-loader-4-line"/>
             </myw::button::Button>
              <myw::Gap w=4/>
             <myw::button::Button w=44 border="both" on_click=|_|{}>
                <myw::icon::Icon class="ri-home-2-line"/>
             </myw::button::Button>

        </div>
    }
}
