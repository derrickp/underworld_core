use super::{
    attack_npc::AttackNpc, cast_spell_on_npc::CastSpellOnNpc,
    cast_spell_on_player::CastSpellOnPlayer, exit_room::ExitRoom, inspect_fixture::InspectFixture,
    inspect_npc::InspectNpc, look_at_current_room::LookAtCurrentRoom,
    look_at_fixture::LookAtFixture, look_at_npc::LookAtNpc, loot_fixture::LootFixture,
    loot_npc::LootNpc, move_player_item::MovePlayerItem,
};

#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Action {
    AttackNpc(AttackNpc),
    CastSpellOnNpc(CastSpellOnNpc),
    CastSpellOnPlayer(CastSpellOnPlayer),
    InspectFixture(InspectFixture),
    InspectNpc(InspectNpc),
    LookAtCurrentRoom(LookAtCurrentRoom),
    LookAtFixture(LookAtFixture),
    LookAtNpc(LookAtNpc),
    LootFixture(LootFixture),
    LootNpc(LootNpc),
    MovePlayerItem(MovePlayerItem),
    ExitRoom(ExitRoom),
}
