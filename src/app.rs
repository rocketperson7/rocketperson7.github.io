use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World from the world's best embedded Rust Dev!" }</h1>
            <p>{ "This is a sample application made with the Yew framework, more to come" }</p>
        </main>
    }
}
