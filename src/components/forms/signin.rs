
use yew::{prelude::*, platform::spawn_local};
use web_sys::HtmlInputElement;
use gloo_console::log;
use gloo_net::Error;

use crate::components::alert::Alert;
use crate::components::input::Input;
use crate::api;
use crate::api::user::{UserLoginResponse, UserInfoResponse};

async fn login(username: String, password: String) -> Result<(UserLoginResponse, UserInfoResponse), Error> {
  let user_login_response = api::user::login(username, password).await?;
  let user_info_response = api::user::info(&user_login_response.token).await?;

  Ok((user_login_response, user_info_response))
}

#[function_component(SignInForm)]
pub fn signin() -> Html {

  let username_handle = use_state(String::default);
  let password_handle = use_state(String::default);
  let error_message_handle = use_state(String::default);

  let username = (*username_handle).clone();
  let password = (*password_handle).clone();
  let error_message = (*error_message_handle).clone();

  let username_changed = Callback::from(move |e: Event| {
    let target = e.target_dyn_into::<HtmlInputElement>();
    if let Some(input) = target {
      username_handle.set(input.value());
    }
  });

  let password_changed = Callback::from(move |e: Event| {
    let target = e.target_dyn_into::<HtmlInputElement>();
    if let Some(input) = target {
      password_handle.set(input.value());
    }
  });

  let cloned_username = username.clone();
  let cloned_password = password.clone();

  let onsubmit = Callback::from(move |e: SubmitEvent| {
    e.prevent_default();
    // log!("Submitting form", username_cloned.clone(), password_cloned.clone());
    
    let username_val = cloned_username.clone();
    let password_val = cloned_password.clone();
    let cloned_error_message_handle = error_message_handle.clone();
    spawn_local(async move {
      match login(username_val.clone(), password_val.clone()).await {
        Ok(responses) => log!(responses.1.username),
        Err(e) => cloned_error_message_handle.set(e.to_string()),
      }
    });

    
  });

  html! {
    <form onsubmit={onsubmit}>
      if error_message.len() > 0 {
        <Alert
          alert_type={"danger"}
          message={error_message}
        />
      }
      <div class="mb-3 form-group">
        <Input 
          label="Username"
          label_class="form-label"
          name="username"
          input_type="text"
          input_class="form-control"
          value={username}
          onchange={username_changed}
        />
      </div>
      <div class="mb-3">
        <Input 
          label="Password"
          label_class="form-label"
          name="password"
          input_type="password"
          input_class="form-control"
          value={password}
          onchange={password_changed}
        />
      </div>
      <div class="mb-3 text-center">
        <button class="btn btn-primary" type="submit">
          {"Login"}
        </button>
      </div>
    </form>
  }
}
