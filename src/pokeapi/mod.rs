pub mod berries;

mod utility;

trait FromId {
    fn from_id(id: &u64) -> Self;
}
