#[derive(Clone, serde::Serialize)]
pub struct CopyProgressEvent {
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub source: String,
    pub destination: String,
}

#[derive(Clone, Copy, serde::Serialize)]
pub struct CopyStartEvent;

#[derive(Clone, Copy, serde::Serialize)]
pub struct CopyFinishedEvent;
