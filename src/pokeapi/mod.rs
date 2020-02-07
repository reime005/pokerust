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

fn get_api_loc_from_url(url: &str) -> &str {
    let pre = "api/v2/";
    &url[(url.rfind(pre).unwrap() + pre.len())..]
}
