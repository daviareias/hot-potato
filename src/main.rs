use actix_files::Files;
use actix_web::{ post, web, App, HttpServer, HttpResponse , Responder};
use std::assert;

mod web_socket;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut ui_path = std::env::current_dir().unwrap();
    ui_path.push("ui");
    assert!( ui_path.is_dir(), "Path is not a directory: {}", ui_path.display());

    HttpServer::new(move || {
        App::new()
            //App::new()
            //.route("/", web::get().to(index))
            .service(web::resource("/ws/").route(web::get().to(web_socket::my_web_socket)))
            // serve all files in ui directory
            .service( Files::new("/", "ui") .index_file("index.html") .show_files_listing(),)
    })
    .bind(("127.0.0.1", 8080))?.
    run()
    .await
}
