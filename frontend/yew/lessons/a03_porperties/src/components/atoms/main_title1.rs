use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(MainTitle1)]
pub fn main_title1(props: &Props) -> Html {

    html! {
        <div>
            // <h1>{"Clip18: Passing Properties into Compoenents"} </h1>
            <h1>{&props.title}</h1>        
        </div>
    }
}