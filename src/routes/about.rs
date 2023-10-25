use leptos::*;

#[server(About "/api")]
pub async fn get_about() -> Result<String, ServerFnError> {
    println!("calling about");
    Ok("hello".to_string())
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <h1 class="text-blue-300">"About page"</h1>
    }
}