#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum GeometryError {
    #[display("invalid geometry")]
    InvalidGeometry,
}
