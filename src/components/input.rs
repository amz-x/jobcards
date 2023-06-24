use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub label: AttrValue,
  pub name: AttrValue,
  pub input_type: AttrValue,
  pub input_class: AttrValue,
  pub label_class: AttrValue,
  pub value: AttrValue,
  pub onchange: Callback<Event>
  
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
  let html_id = format!("edit-{}", props.name);
  html! {
    <>
      <label for={html_id.clone()} class={format!("{}", props.label_class.clone())}>{props.label.clone()}</label>
      <input
        id={html_id} 
        name={props.name.clone()}
        type={props.input_type.clone()}
        class={format!("{}", props.input_class.clone())}
        value={props.value.clone()}
        onchange={props.onchange.clone()}
      />
    </>
  }
}
