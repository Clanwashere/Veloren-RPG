use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum BiomeKind {
    Void,
    Lake,
    Grassland,
    Ocean,
    Mountain,
    Snowland,
    Desert,
    Swamp,
    Jungle,
    Forest,
}
