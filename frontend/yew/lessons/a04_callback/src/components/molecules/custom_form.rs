use yew::prelude::*;
use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::custom_button::CustomButton;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    html! {
        <from>
            <h1>{"Clip21: Creating a Form"}</h1>
            <TextInput name="username" />
            <CustomButton label="Submit" />
        </from>
    }
}