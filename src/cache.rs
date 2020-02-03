use cached::cached_key_result;
use cached::UnboundCache;
use default_env::default_env;
use minreq::Response;

static ENDPOINT: &str = default_env!("POKERUST_ENDPOINT", "http://localhost:8000/api/v2/");

cached_key_result! {
   POKEAPI_CACHE: UnboundCache<String, Response> = UnboundCache::new();
   Key = { path.to_owned() };
   fn get_resource(path: &str) -> Result<Response, minreq::Error> = {
       minreq::get(format!("{}{}", ENDPOINT, path)).send()
   }
}
