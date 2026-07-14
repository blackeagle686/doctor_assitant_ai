use qdrant_client::qdrant::{UpsertPointsBuilder, PointStruct};
fn test() {
    let _ = UpsertPointsBuilder::new("col", vec![]).wait(true).build();
}
