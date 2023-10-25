use leptos::*;
use reqwest::get;
// use leptos::leptos_dom::logging::console_log;
use crate::store::Todo;
// use sqlx::query_as;
// use super::store::
#[server(About "/api")]
pub async fn get_about() -> Result<String, ServerFnError> {
    println!("calling about");
    Ok("hello".to_string())
}

// #[server(Todos "/api")]
// async fn get_todo(id: i16) -> Result<Todo> {
//     let item_id = id.into_inner();
//     let result = query_as::<_, Todo>("SELECT * FROM todos where id = $1")
//         .bind(item_id)
//         .fetch_one(&app_state.pool)
//         .await;
//
//     match result {
//         Ok(payload) => payload,
//         Err(err) => err.to_string()
//     }
// }

async fn fetch_todos() -> Vec<Todo> {
    println!("fetching data");
    get("http://localhost:8080/api/todo")
        .await
        .unwrap()
        .json::<Vec<Todo>>()
        .await
        .expect("Failed fetching todos")
}

#[component]
pub fn About() -> impl IntoView {
    let (count, set_count) = create_signal(4);
    let (data, setData) = create_signal::<Vec<Todo>>(vec![]);
    let _ = create_resource(
        || (),
        move |_| async move {
            setData(fetch_todos().await);
        },
    );
    // let is_loading = once.loading();
    view! {
        <div class="text-blue-300">
            <div>
                "hello"
                <For
                    each=data
                    key=|state| state.id
                    let:child
                >
                    <p>{child.title}</p>
                </For>
            </div>
            <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            >
                "Click me"
            </button>
            <p>{count}</p>
        </div>
    }
}
