# Pokérust

Wrapper library for <https://pokeapi.co/> v2 with caching support.

## Documentation

Documentation for the crate can be found on
[docs.rs](https://docs.rs/pokerust) (WIP). For documentation of the API, see
<https://pokeapi.co/docs/v2.html>.

## Basic Usage

Get an object from an API by id

```rust
use pokerust::{Berry, FromId};

fn main() {
    let berry = Berry::from_id(1).unwrap();
}
```

or by name

```rust
use pokerust::{Berry, FromName};

fn main() {
    let berry = Berry::from_name("cheri").unwrap();
}
```

API responses are automatically cached.

You can also fetch the resource lists:

```rust
use pokerust::{Item, Endpoint, List};

fn main() {
    let items = Item::list(5, 20).unwrap();  // ?offset=5&limit=20

    // get the lists referenced in the next and previous fields
    items.previous_list().unwrap();
    items.next_list().unwrap();

    // you can also just get the full list
    let all_items = Item::full_list().unwrap();
}
```

To get resources pointed to by `(Named)APIResource`, use `get()`:

```rust
let berry = Berry::from_name("cheri").unwrap();
let berry_item = berry.item.get().unwrap(); // berry_item is an Item
```

This can be chained:

```rust
let marill = PokemonSpecies::from_name("marill").unwrap();
let sea_incense = marill.evolution_chain.get().unwrap().baby_trigger_item.unwrap().get().unwrap();
```

The location of the pokeapi used can be changed by setting the
POKERUST_ENDPOINT environment variable. Defaults to the public instance at
<https://pokeapi.co/api/v2/>. Please consult the pokeapi documentation and read
the fair use policy before using the public API instance.

## License

This software is licensed under the BSD 3-Clause "New" or "Revised" License.
