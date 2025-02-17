use crate::domain::entities::DataStream;
use crate::domain::services::DataStreamService;

pub struct ManageDataStream {
    data_stream_service: DataStreamService,
}

impl ManageDataStream {
    pub fn new(data_stream_service: DataStreamService) -> Self {
        Self { data_stream_service }
    }

    pub fn create_data_stream(&self, id: String, name: String, status: String) {
        let data_stream = DataStream::new(id, name, status);
        self.data_stream_service.add_data_stream(data_stream);
    }

    pub fn view_data_stream(&self, id: &str) -> Option<DataStream> {
        self.data_stream_service.get_data_stream(id)
    }

    pub fn view_all_data_streams(&self) -> Vec<DataStream> {
        self.data_stream_service.get_all_data_streams()
    }

    pub fn remove_data_stream(&self, id: &str) {
        self.data_stream_service.delete_data_stream(id);
    }
}
