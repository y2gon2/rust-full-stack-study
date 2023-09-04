use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(InlineStyle)]
pub fn inline_style() -> Html {
    let stylesheet = style!(
        r#"
            h1 {
                color: orange;
            }
            p {
                color: white;
            }
        "#
    ).unwrap();

    html! {
        <div class={stylesheet}>
            <h1>{"clip 16: Inline Style"}</h1>
            <p>{"more text!!"}</p>
        </div>
    }
}