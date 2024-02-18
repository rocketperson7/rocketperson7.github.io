use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Making the world a Rustier place" }</h1>
            <p>{ "If you'd like to work with us, please reach out" }</p>
        </main>
    }
}
