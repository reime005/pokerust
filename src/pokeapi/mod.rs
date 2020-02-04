pub mod berries;
pub mod contests;
pub mod encounters;
pub mod evolution;
pub mod games;
pub mod items;
pub mod locations;
pub mod machines;
pub mod moves;
pub mod pokemon;
pub mod resource_lists;
pub mod utility;

trait FromId {
    fn from_id(id: &u64) -> Self;
}
