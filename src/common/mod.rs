pub mod packets;
pub mod constants;

pub mod lobby {
    pub struct Lobby {
        id: u16,
        players: Vec<u16>,
    }

    impl Lobby {
        pub fn new(id: u16, max_players: usize) -> Self {
            Self {
                id,
                players: Vec::with_capacity(max_players),
            }
        }
    }
}