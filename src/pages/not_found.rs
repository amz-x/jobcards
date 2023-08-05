use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
  html! {
    <div class="container">
      <div class="row min-vh-100 justify-content-center align-items-center">
        <div class="col-md-12 text-center">
          <h1>{"404 - Page Not Found"}</h1>
        </div>
      </div>
    </div>   
  }
}
