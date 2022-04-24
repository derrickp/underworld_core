use crate::{
    actions::exit_room::ExitRoom,
    components::games::game_state::GameState,
    events::{event::Event, room_exited::RoomExited, room_generated::RoomGenerated},
    generators::{generator::Generator, rooms::random_room_generator},
    utils::ids::parse_id,
};

pub fn handle_exit_room(exit_room: &ExitRoom, state: &GameState) -> Vec<Event> {
    // We need to check the exit maps for one with the room_id and exit.
    // If there's another exit id then find the room with that exit id and move
    // the player to that room.
    // If there is no room id as the other one, then we will need to generate
    // a room with this exit ID as its entrance.
    // Then we will need to move the player to that room and exit their
    // current room.

    let mut events: Vec<Event> = Vec::new();

    let exit_id = parse_id(&exit_room.exit_id).unwrap();
    let exit_map = state
        .world
        .exit_graph
        .iter()
        .find(|exit_map| exit_map.exit_id.eq(&exit_id))
        .unwrap();

    let other_room_id = exit_map.other_room_id(state.current_room_id);

    let room_id = match other_room_id {
        Some(id) => id,
        None => {
            let room_generator = random_room_generator(Some(exit_id));
            let room = room_generator.generate();
            let room_id = room.identifier.id;
            events.push(Event::RoomGenerated(RoomGenerated {
                room,
                entrance_id: exit_id,
            }));
            room_id
        }
    };

    events.push(Event::RoomExited(RoomExited {
        exit_id,
        old_room_id: state.current_room_id,
        new_room_id: room_id,
    }));

    events
}