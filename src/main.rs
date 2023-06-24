use yew::prelude::*;

mod pages;
mod components;
mod api;

#[function_component(App)]
fn app() -> Html {
  html! {
    <pages::signin::SignInPage />
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
