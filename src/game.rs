use crate::{
    actions::action::Action,
    components::{games::game_state::GameState, player::PlayerCharacter},
    events::event::Event,
    handlers::handle::{handle, HandledAction},
};

pub struct Game {
    pub state: GameState,
    pub player: PlayerCharacter,
}

impl Game {
    pub fn update_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn handle_action(&mut self, action: &Action) -> Vec<Event> {
        let HandledAction { events, new_state } = handle(action, &self.state, &self.player);
        self.state = new_state;

        events
    }
}
