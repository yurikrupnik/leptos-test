use leptos::*;

// fn print() {
//     println!("Button");
// }

#[component]
pub fn Button(
    #[prop(optional)] title: String,
    #[prop(into)] handler: Box<dyn Fn() + 'static>,
) -> impl IntoView {
    view! {
        <button on:click=move |_|handler()>{title}</button>
    }
}
