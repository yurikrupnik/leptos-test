use actix_web::{HttpResponse, Responder};
use leptos::*;
mod components;
use components::Navbar;

async fn clicked() -> impl Responder {
    let html = ssr::render_to_string(Navbar).to_string();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
mod store;
// use store::Store;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_cors::Cors;
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_test::app::*;
    // use env_logger::Env;
    // use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

    // async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    //     let pool = PgPoolOptions::new()
    //         .max_connections(5)
    //         .connect("postgres://myuser:mypassword@localhost/mydatabase").await?;
    //     Ok(pool)
    // }
    // env_logger::init_from_env(Env::default().default_filter_or("info"));
    // let pool = create_pool().await.expect("Failed to create pool");
    // let app_state = Store::new(pool);

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            // .allowed_origin("http://localhost:5173")
            .allowed_origin_fn(|origin, _req_head| {
                print!("origin!! {}, url!! {}", origin.is_empty(), _req_head.uri);
                true
            })
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .max_age(3600);
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            // .app_data(app_state.clone())
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            // .service(favicon)
            .route("/clicked", web::post().to(clicked))
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
            .wrap(cors)
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::*;
    use leptos_test::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
