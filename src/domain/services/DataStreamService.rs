use crate::domain::entities::DataStream;
use crate::domain::repositories::DataStreamRepository;

pub struct DataStreamService<T: DataStreamRepository> {
    repository: T,
}

impl<T: DataStreamRepository> DataStreamService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn add_data_stream(&self, data_stream: DataStream) {
        self.repository.save(data_stream);
    }

    pub fn get_data_stream(&self, id: &str) -> Option<DataStream> {
        self.repository.find_by_id(id)
    }

    pub fn get_all_data_streams(&self) -> Vec<DataStream> {
        self.repository.find_all()
    }

    pub fn delete_data_stream(&self, id: &str) {
        self.repository.delete(id);
    }
}
