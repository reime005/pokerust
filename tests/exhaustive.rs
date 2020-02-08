use paste;
use pokerust::*;

mod common;

#[cfg(test)]
macro_rules! test_every {
    { $A:ty, $B:ident } => {
paste::item! {
#[test]
#[ignore]
fn [<get_every_ $B>]() {
    let [<$B _list>] = $A::full_list().unwrap();

    for [<$B _res>] in [<$B _list>].results {
        let $B = [<$B _res>].get().unwrap();

        assert_eq!([<$B _res>].id(), $B.id)
    }
}
}
};
}

test_every!{Ability, ability}
test_every!{Berry, berry}
test_every!{BerryFirmness, berry_firmness}
test_every!{BerryFlavor, berry_flavor}
test_every!{Characteristic, characteristic}
test_every!{ContestEffect, contest_effect}
test_every!{ContestType, contest_type}
test_every!{EggGroup, egg_group}
test_every!{EncounterCondition, encounter_condition}
test_every!{EncounterConditionValue, encounter_condition_value}
test_every!{EncounterMethod, encounter_method}
test_every!{EvolutionChain, evolution_chain}
test_every!{EvolutionTrigger, evolution_trigger}
test_every!{Gender, gender}
test_every!{Generation, generation}
test_every!{GrowthRate, growth_rate}
test_every!{Item, item}
test_every!{ItemAttribute, item_attribute}
test_every!{ItemCategory, item_category}
test_every!{ItemFlingEffect, item_fling_effect}
test_every!{ItemPocket, item_pocket}
test_every!{Language, language}
test_every!{Location, location}
test_every!{LocationArea, location_area}
test_every!{Machine, machine}
test_every!{Move, r#move}
test_every!{MoveAilment, move_ailment}
test_every!{MoveBattleStyle, move_battle_style}
test_every!{MoveCategory, move_category}
test_every!{MoveDamageClass, move_damage_class}
test_every!{MoveLearnMethod, move_learn_method}
test_every!{MoveTarget, move_target}
test_every!{Nature, nature}
test_every!{PalParkArea, pal_park_area}
test_every!{PokeathlonStat, pokeathlon_stat}
test_every!{Pokedex, pokedex}
test_every!{Pokemon, pokemon}
test_every!{PokemonColor, pokemon_color}
test_every!{PokemonForm, pokemon_form}
test_every!{PokemonHabitat, pokemon_habitat}
test_every!{PokemonShape, pokemon_shape}
test_every!{PokemonSpecies, pokemon_species}
test_every!{Region, region}
test_every!{Stat, stat}
test_every!{SuperContestEffect, super_contest_effect}
test_every!{Type, r#type}
test_every!{Version, version}
test_every!{VersionGroup, version_group}
