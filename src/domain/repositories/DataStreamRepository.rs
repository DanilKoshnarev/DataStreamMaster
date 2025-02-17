use crate::domain::entities::DataStream;

pub trait DataStreamRepository {
    fn save(&self, data_stream: DataStream);
    fn find_by_id(&self, id: &str) -> Option<DataStream>;
    fn find_all(&self) -> Vec<DataStream>;
    fn delete(&self, id: &str);
}
