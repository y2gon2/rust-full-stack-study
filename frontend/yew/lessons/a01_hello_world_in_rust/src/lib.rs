use yew::prelude::*;
use gloo::console::log;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "Gon";
    let my_object = MyObject { 
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!("my name is", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let class = "my_title";
    html! { 
        <> 
            <h1 class={class} style="color:red">{"Hello World!"}</h1>
            <p>{"Hi there"}</p>
        </>
    }
}