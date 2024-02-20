use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="assets/bloxide_logo.webp" alt="Bloxide logo" />
            <h1>{ "Making the world a Rustier place" }</h1>
            <p>{ "If you'd like to work with us, please reach out" }</p>
            <iframe src="https://docs.google.com/forms/d/e/1FAIpQLSfkgLYjHyEhlYerLFAEUEM_XDEXdJUVZ8zgA8bbrCihlQxglA/viewform?embedded=true" width="640" height="1065" frameborder="0" marginheight="0" marginwidth="0">{"Loading…"}</iframe>
        </main>
    }
}
