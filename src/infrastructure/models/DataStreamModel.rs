use crate::domain::entities::DataStream;
use crate::domain::repositories::DataStreamRepository;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct DataStreamModel {
    data_streams: Mutex<HashMap<String, DataStream>>,
}

impl DataStreamModel {
    pub fn new() -> Self {
        Self {
            data_streams: Mutex::new(HashMap::new()),
        }
    }
}

impl DataStreamRepository for DataStreamModel {
    fn save(&self, data_stream: DataStream) {
        let mut data_streams = self.data_streams.lock().unwrap();
        data_streams.insert(data_stream.id.clone(), data_stream);
    }

    fn find_by_id(&self, id: &str) -> Option<DataStream> {
        let data_streams = self.data_streams.lock().unwrap();
        data_streams.get(id).cloned()
    }

    fn find_all(&self) -> Vec<DataStream> {
        let data_streams = self.data_streams.lock().unwrap();
        data_streams.values().cloned().collect()
    }

    fn delete(&self, id: &str) {
        let mut data_streams = self.data_streams.lock().unwrap();
        data_streams.remove(id);
    }
}
