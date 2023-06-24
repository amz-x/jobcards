use yew::prelude::*;

use crate::components::forms::signin::SignInForm;

#[function_component(SignInPage)]
pub fn signin() -> Html {
  html! {
    <div class="container">
      <div class="row min-vh-100 justify-content-center align-items-center">
        <div class="col-md-4">
          <SignInForm />
        </div>
      </div>
    </div>   
  }
}
