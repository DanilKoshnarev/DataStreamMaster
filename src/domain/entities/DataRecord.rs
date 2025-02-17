pub struct DataRecord {
    pub id: String,
    pub stream_id: String,
    pub data: String,
    pub timestamp: String,
}

impl DataRecord {
    pub fn new(id: String, stream_id: String, data: String, timestamp: String) -> Self {
        Self { id, stream_id, data, timestamp }
    }
}
