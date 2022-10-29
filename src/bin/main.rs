
use actix_web::{App, web::Data};
use actix::Actor;
use nacos_rust::grpc::nacos_proto::bi_request_stream_server::BiRequestStreamServer;
use nacos_rust::grpc::nacos_proto::request_server::RequestServer;
use nacos_rust::grpc::server::BiRequestStreamServerImpl;
use nacos_rust::{naming::core::NamingActor, grpc::server::RequestServerImpl};
use nacos_rust::config::config::ConfigActor;
use tonic::transport::Server;
use std::error::Error;

use nacos_rust::web_config::app_config;
use actix_web::{
    middleware,HttpServer,
};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>>  {
    std::env::set_var("RUST_LOG","actix_web=debug,actix_server=info,info");
    env_logger::init();
    let config_addr = ConfigActor::new().start();
    let naming_addr = NamingActor::new_and_create(5000);

    tokio::spawn(async move {
        let addr = "0.0.0.0:9848".parse().unwrap();
        let request_server = RequestServerImpl::default();
        let bi_request_stream_server = BiRequestStreamServerImpl::default();
        Server::builder()
        .add_service(RequestServer::new(request_server))
        .add_service(BiRequestStreamServer::new(bi_request_stream_server))
        .serve(addr)
        .await.unwrap();
    });

    HttpServer::new(move || {
        let config_addr = config_addr.clone();
        let naming_addr = naming_addr.clone();
        App::new()
            .app_data(Data::new(config_addr))
            .app_data(Data::new(naming_addr))
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .workers(8)
    .bind("0.0.0.0:8848")?
    .run()
    .await?;
    Ok(())
}
