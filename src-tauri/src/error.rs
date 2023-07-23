#[derive(Debug, thiserror::Error)]
pub enum CopyError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Glob(#[from] globwalk::GlobError),
}

impl serde::Serialize for CopyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
