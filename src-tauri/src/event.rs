#[derive(Clone, serde::Serialize)]
pub struct CopyProgressEvent {
    pub file_name: String,
}

#[derive(Clone, Copy, serde::Serialize)]
pub struct CopyStartEvent;

#[derive(Clone, Copy, serde::Serialize)]
pub struct CopyFinishedEvent;
