use crate::components::{
    character::CharacterViewArgs,
    rooms::{
        fixture_position::FixturePositionView,
        npc_position::{NpcPositionView, NpcPositionViewArgs},
        room::Room,
        room_view::{RoomView, RoomViewArgs},
    },
};

pub fn quick_look(room: &Room) -> RoomView {
    let npc_position_args = NpcPositionViewArgs {
        character_args: CharacterViewArgs {
            knows_health: false,
            knows_species: true,
            knows_life_modifier: true,
            knows_inventory: false,
            knows_hidden_in_inventory: false,
            knows_packed_in_inventory: false,
        },
        knows_name: false,
    };

    view(room, npc_position_args, false)
}

pub fn look_at(room: &Room, args: RoomViewArgs, knows_all: bool) -> RoomView {
    let npc_position_args = NpcPositionViewArgs {
        character_args: CharacterViewArgs {
            knows_health: args.knows_character_health,
            knows_species: true,
            knows_life_modifier: true,
            knows_inventory: true,
            knows_hidden_in_inventory: args.can_see_hidden,
            knows_packed_in_inventory: args.can_see_packed,
        },
        knows_name: args.knows_names,
    };

    view(room, npc_position_args, knows_all)
}

fn view(room: &Room, npc_position_args: NpcPositionViewArgs, knows_all: bool) -> RoomView {
    let fixture_positions: Vec<FixturePositionView> = room
        .fixture_positions
        .iter()
        .map(|fixture_position| super::fixture_position::look_at(&fixture_position))
        .into_iter()
        .collect();
    let npc_positions: Vec<NpcPositionView> = room
        .npc_positions
        .iter()
        .map(|npc_position| {
            super::npc_position::look_at(&npc_position, &npc_position_args, knows_all)
        })
        .into_iter()
        .collect();

    RoomView {
        identifier: super::identifier::to_view(&room.identifier, true),
        descriptors: room.descriptors.clone(),
        room_type: room.room_type.clone(),
        fixture_positions,
        dimensions: room.dimensions.clone(),
        npc_positions,
        flavour: room.flavour.clone(),
    }
}