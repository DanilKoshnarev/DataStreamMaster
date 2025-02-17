use actix_web::{web, HttpResponse};
use crate::domain::use_cases::ManageDataStream;
use serde::Deserialize;

pub struct DataStreamController {
    manage_data_stream: ManageDataStream,
}

#[derive(Deserialize)]
pub struct CreateDataStreamRequest {
    pub id: String,
    pub name: String,
    pub status: String,
}

impl DataStreamController {
    pub fn new(manage_data_stream: ManageDataStream) -> Self {
        Self { manage_data_stream }
    }

    pub async fn create_data_stream(
        manage_data_stream: web::Data<Self>,
        request: web::Json<CreateDataStreamRequest>,
    ) -> HttpResponse {
        manage_data_stream.manage_data_stream.create_data_stream(
            request.id.clone(),
            request.name.clone(),
            request.status.clone(),
        );
        HttpResponse::Ok().finish()
    }

    pub async fn get_all_data_streams(
        manage_data_stream: web::Data<Self>,
    ) -> HttpResponse {
        let data_streams = manage_data_stream.manage_data_stream.view_all_data_streams();
        HttpResponse::Ok().json(data_streams)
    }
}
