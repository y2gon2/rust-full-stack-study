pub mod clips;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    
    html! {
        <>
            <clips::c16_inline_styles::InlineStyle/>
            // <clips::c17_separates_css_file::SeparateCSSFile/>   
        </>
    }
}

