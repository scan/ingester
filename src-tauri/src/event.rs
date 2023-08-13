#[derive(Clone, serde::Serialize)]
pub struct CopyProgressEvent {
    pub file_name: String,
    pub index: usize,
    pub total: usize,
}
