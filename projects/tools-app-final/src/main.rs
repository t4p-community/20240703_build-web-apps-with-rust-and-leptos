#![feature(fn_traits)]

#[cfg(feature = "ssr")]
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    use std::io;

    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use sqlx::{migrate, sqlite::SqlitePoolOptions};
    use tools_app_final::app::*;

    console_error_panic_hook::set_once();

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite:/workspaces/20240703_build-web-apps-with-rust-and-leptos/projects/tools-app-final/toolsapp.sqlite")
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    migrate!("./migrations")
        .run(&db_pool)
        .await
        .unwrap_or_else(|_| panic!("could not run sqlx migration {}", whoami::username().as_str()));

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::Compress::default())
            // .service(tools_app_wasm)
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{}/pkg", site_root))
                .prefer_utf8(true).use_etag(true).use_last_modified(true))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root)
                .prefer_utf8(true).use_etag(true).use_last_modified(true))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}


// use actix_web::Responder;

// #[cfg(feature = "ssr")]
// #[actix_web::get("/pkg/tools-app.wasm")]
// async fn tools_app_wasm(
//     request: actix_web::HttpRequest,
//     leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
// ) -> actix_web::Result<impl Responder> {
//     use actix_web::{http::header, Responder};

//     let leptos_options = leptos_options.into_inner();
//     let site_root = &leptos_options.site_root;

//     let enc = request.headers().get("Accept-Encoding").map(|enc| enc.to_str().unwrap());
//     let file = if enc.is_some() && enc.unwrap().contains("br") {
//         format!("{}/pkg/tools-app.wasm.br", site_root)
//     } else if enc.is_some() && enc.unwrap().contains("gzip") {
//         format!("{}/pkg/tools-app.wasm.gz", site_root)
//     } else {
//         format!("{}/pkg/tools-app.wasm", site_root)
//     };

//     if file.ends_with(".br") {
//         Ok(
//             actix_files::NamedFile::open_async(
//                 file
//             ).await?
//                 .customize()
//                 .insert_header(header::ContentEncoding::Brotli))
//     } else if file.ends_with(".gz") {
//         Ok(
//             actix_files::NamedFile::open_async(
//                 file
//             ).await?
//                 .customize()
//                 .insert_header(header::ContentEncoding::Gzip))
//     } else {
//         Ok(
//             actix_files::NamedFile::open_async(
//                 file
//             ).await?
//                 .customize()
//         )
//     }
// }


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
    use tools_app::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
