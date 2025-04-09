#[derive(Clone)]
pub struct Peer {
    pub alive: bool,
    pub last_seen: u64,
}