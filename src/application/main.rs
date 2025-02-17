use actix_web::{web, App, HttpServer};
use data_stream_master::domain::repositories::DataStreamRepository;
use data_stream_master::domain::services::DataStreamService;
use data_stream_master::domain::use_cases::ManageDataStream;
use data_stream_master::infrastructure::models::DataStreamModel;
use data_stream_master::infrastructure::controllers::DataStreamController;

mod domain;
mod infrastructure;
mod application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data_stream_repository = DataStreamModel::new();
    let data_stream_service = DataStreamService::new(data_stream_repository);
    let manage_data_stream = ManageDataStream::new(data_stream_service);
    let data_stream_controller = DataStreamController::new(manage_data_stream);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data_stream_controller.clone()))
            .route("/data_streams", web::post().to(DataStreamController::create_data_stream))
            .route("/data_streams", web::get().to(DataStreamController::get_all_data_streams))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
