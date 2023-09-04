use yew::prelude::*;
use stylist::{style, yew::styled_component};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
}

#[derive(PartialEq)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_owned(),
            Color::Ok => "ok".to_owned(),
            Color::Error => "Error".to_owned(),
        }
    }
}


#[styled_component(MainTitle2)]
pub fn main_title2(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
        .normal {
            color: yellow;
        }
        .ok {
            color: green;
        }
        .error {
            color: red;
        }
        "#
    ).unwrap();

    html! {
        <div class={stylesheet}>
            <h1 class={props.color.to_string()}>{&props.title}</h1>
        </div>
    }
}