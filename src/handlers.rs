use crate::{
    actions::action::Action,
    components::{games::game_state::GameState, player::PlayerCharacter},
    events::event::{apply_events, Event},
};

use self::{attack_npc::handle_attack_npc, exit_room::handle_exit_room, view_npc::handle_view_npc};

mod attack_npc;
mod exit_room;
mod view_npc;

pub struct HandledAction {
    pub events: Vec<Event>,
    pub new_player: PlayerCharacter,
    pub new_state: GameState,
}

pub fn handle(action: &Action, state: &GameState, player: &PlayerCharacter) -> HandledAction {
    let events = match action {
        Action::LookAtTarget(_) => Vec::new(),
        Action::LookAtRoom(_) => Vec::new(),
        Action::QuickLookRoom(_) => Vec::new(),
        Action::ExitRoom(exit_room) => handle_exit_room(exit_room, state),
        Action::AttackNpc(attack_npc) => handle_attack_npc(attack_npc, state, player),
        Action::LootNpc(_) => Vec::new(),
        Action::LookAtNpc(look_at_npc) => handle_view_npc(state, look_at_npc),
    };

    let (new_state, new_player) = apply_events(&events, state, player);

    HandledAction {
        new_state,
        new_player,
        events,
    }
}
