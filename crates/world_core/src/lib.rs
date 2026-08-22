#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorldId(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RegionId(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ChunkId(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EntityId(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EventId(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorldSeed(u64);

impl WorldId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl RegionId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl ChunkId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl EntityId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl EventId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl WorldSeed {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_id_preserves_value() {
        let id = WorldId::new(42);

        assert_eq!(id.value(), 42);
    }

    #[test]
    fn region_id_preserves_value() {
        let id = RegionId::new(91);

        assert_eq!(id.value(), 91);
    }

    #[test]
    fn chunk_id_preserves_value() {
        let id = ChunkId::new(89513);

        assert_eq!(id.value(), 89513);
    }

    #[test]
    fn entity_id_preserves_value() {
        let id = EntityId::new(6);

        assert_eq!(id.value(), 6);
    }

    #[test]
    fn event_id_preserves_value() {
        let id = EventId::new(384);

        assert_eq!(id.value(), 384);
    }

    #[test]
    fn world_seed_preserves_value() {
        let seed = WorldSeed::new(384);

        assert_eq!(seed.value(), 384);
    }

    #[test]
    fn equal_world_seeds_are_equal() {
        assert_eq!(WorldSeed::new(1234), WorldSeed::new(1234));
        assert_ne!(WorldSeed::new(1234), WorldSeed::new(5678));
    }
}
