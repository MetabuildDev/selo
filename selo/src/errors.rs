#[derive(Debug, thiserror::Error)]
pub enum GeometryError {
    #[error("invalid geometry")]
    InvalidGeometry,
}
