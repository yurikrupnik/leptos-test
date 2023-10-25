use leptos::*;
use leptos::ev::{MouseEvent, Event};
use leptos::leptos_dom::logging::{console_error, console_log};
use leptos_meta::*;
use leptos_router::*;
// use super::routes::About;
use crate::routes::About;
use crate::components::{Button};
use crate::components::{Navbar};
// use crate::routes::home::*;
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet\
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        // <Script src="https://unpkg.com/htmx.org@1.9.6" integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni" crossorigin="anonymous"></Script>
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <Navbar />
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/about" view=About/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (name, set_name) = create_signal("Controlled".to_string());
    let (count, set_count) = create_signal(0);
    let on_click = move |e: MouseEvent| {
        let s = e.client_y();
        println!("Button clicked! {}", s);
        set_count(count() + 1);
    };
    let on_change = move |ev: Event| {
        set_name(event_target_value(&ev));
    };
    let handler = || {
        let s = "Hello World!!";
        console_log(&format!("Button clicked! {}", s));
        console_error("Button clicked!!!!");
    };

    let handler_boxed: Box<dyn Fn()> = Box::new(handler);
    view! {
        <h1 class="text-black-300">"Welcome to Leptos!"</h1>
        <input
            on:input=on_change
            type="text"
        />
        <div class="mb-6">
          <label for="username-success" class="block mb-2 text-sm font-medium text-green-700 dark:text-green-500">Your name</label>
          <input type="text" on:input=on_change id="username-success" class="bg-green-50 border border-green-500 text-green-900 placeholder-green-700 text-sm rounded-lg focus:ring-green-500 focus:border-green-500 block w-full p-2.5 dark:bg-green-100 dark:border-green-400" placeholder="Bonnie Green" />
          <p class="mt-2 text-sm text-green-600 dasdark:text-green-500">
            <span class="font-medium">Alright!</span> Username available!
          </p>
        </div>
        <p>{name}</p>
        <Button handler=handler_boxed title="Og".to_string()/>
        <button on:click=on_click>"Click Me: " {count}</button>
        <button hx-post="/clicked" hx-swap="outerHTML">
            Click Me
        </button>
    }
}

// #[component]
// fn About() -> impl IntoView {
//     view! {
//         <h1 class="text-gray-500">"About page"</h1>
//     }
// }

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
