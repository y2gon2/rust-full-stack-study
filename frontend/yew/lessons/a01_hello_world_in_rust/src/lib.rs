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
    // let message = Some("I am a message.");
    let message: Option<&str> = None;

    let _items = (1..10).collect::<Vec<_>>();
    let tasks = vec!["record video", "grocery shopping", "pet Xilbe"];

    let stylesheet = "color:brown";

    html! { 
        <> 
            <h1 class={class} style="color:red">{"Hello World!"}</h1>

            if class == "my_titles" {
                <p>{"Hi there"}</p>
            } else {
                <p>{"I'm not a titles"}</p>
            }

            if let Some(msg) = message {
                <p>{msg}</p>
            } else {
                <p>{"no messages to see today"}</p>
            }

            <ul class="task-list" style={stylesheet}>
                {tasks.iter().map(|task| html!{<li>{task}</li>}).collect::<Html>()}
            </ul>

            <ul class="items" style="color:skyblue">
                {list_to_html(tasks)}
            </ul>
        </>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html!{<li>{item}</li>}).collect()
} 

