use stylist::{yew::styled_component, Style};
use yew::prelude::*;

mod components;

use components::atoms::main_title1::MainTitle1;
use components::atoms::main_title2::{MainTitle2, Color};

const _STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    // let styleshteet = Style::new(STYLE_FILE).unwrap();
    html!{
        <div>
            <MainTitle1 title="Clip18: Passing Properties into Compoenents"/>
            <MainTitle2 title="Clip19: Enum Properties" color={Color::Normal}/>
        </div>
    }
}