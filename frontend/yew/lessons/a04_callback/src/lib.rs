use stylist::yew::styled_component;
use yew::prelude::*;
use gloo::console::log;

mod components;
use components::atoms::main_title::{MainTitle, Color};
use components::molecules::custom_form::CustomForm;

#[styled_component(App)]
pub fn app() -> Html {
    let main_title_load = Callback::from(|message| log!(message));
    html!{
        <div>
            <MainTitle title="Clip20: Callbacks" color={Color::Normal} on_load={main_title_load}/>
            <CustomForm />
        </div>
    }
}