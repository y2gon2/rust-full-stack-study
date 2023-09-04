// use stylist::{Style, yew::styled_component};
// use yew::prelude::*;

// pub const STYLE_FILE: &str = include_str!("../main.css");

// #[styled_component(SeparateCSSFile)]
// pub fn separate_css_file() -> Html {
//     // Syte::new() 가 왜 에러가 발생하는지 모르겠음. 근데 main.rs 
//     let stylesheet = Style::new("background-color: red;").unwrap();
//     html!{
//         <div class={stylesheet}>
//             <h1>{"clip 17: Separate CSS file"}</h1>
//             <p>{"more text"}</p>
//         </div>
//     }
// } 