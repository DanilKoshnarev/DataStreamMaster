pub struct DataStream {
    pub id: String,
    pub name: String,
    pub status: String,
}

impl DataStream {
    pub fn new(id: String, name: String, status: String) -> Self {
        Self { id, name, status }
    }
}
