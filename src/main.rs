use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod components;
mod api;

#[derive(Routable, PartialEq, Clone)]
enum Route {
  #[at("/")]
  Home,
  #[at("/rustaceans")]
  Rustaceans,
  #[at("/crates")]
  Crates,
  #[at("/login")]
  Login,
  #[not_found]
  #[at("/404")]
  NotFound
}

fn switch(route: Route) -> Html {
  match route {
    Route::Home     => html! { <pages::home::HomePage /> },
    Route::Login    => html! { <pages::signin::SignInPage /> },
    Route::NotFound => html! { <pages::not_found::NotFoundPage /> },
    _               => html! { <pages::not_found::NotFoundPage /> }
  }
}

#[function_component(App)]
fn app() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={switch} />
    </BrowserRouter>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
