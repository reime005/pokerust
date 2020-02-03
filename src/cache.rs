use cached::cached_key_result;
use cached::UnboundCache;
use lazy_static::lazy_static;
use minreq::Response;
use std::env;

lazy_static! {
    static ref ENDPOINT: String = match env::var("POKERUST_ENDPOINT") {
        Ok(val) => val,
        Err(env::VarError::NotPresent) => String::from("http://localhost:8000/api/v2/"),
        Err(_) => panic!("Error reading endpoint from POKERUST_ENDPOINT"),
    };
}

cached_key_result! {
   POKEAPI_CACHE: UnboundCache<String, Response> = UnboundCache::new();
   Key = { path.to_owned() };
   fn get_resource(path: &str) -> Result<Response, minreq::Error> = {
       minreq::get(format!("{}{}", *ENDPOINT, path)).send()
   }
}
