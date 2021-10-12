use crate::common::Order;
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use std::sync::mpsc::Sender;
use std::sync::mpsc::SyncSender;
mod ws_server;
use ws_server::ws_handler;
mod order_handler;

pub struct AppState {
    order_channel: SyncSender<Order>,
}

#[actix_web::main]
pub async fn start(order_channel: SyncSender<Order>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                order_channel: order_channel.clone(),
            })
            // enable logger
            .wrap(middleware::Logger::default())
            // websocket route
            .service(web::resource("/ws").route(web::get().to(ws_handler)))
            // static files
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
    // let _ = order_channel.send(srv);
}
