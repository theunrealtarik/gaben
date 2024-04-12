#![allow(dead_code)]
mod generated;
use generated::{client, offsets};

pub const DW_LOCAL_PAWN: usize = offsets::cs2_dumper::offsets::client_dll::dwLocalPlayerPawn;
pub const DW_LOCAL_PAWN_CONTROLLER: usize =
    offsets::cs2_dumper::offsets::client_dll::dwLocalPlayerController;
pub const DW_ENTITY_LIST: usize = offsets::cs2_dumper::offsets::client_dll::dwEntityList;

use client::cs2_dumper::schemas::client_dll;
pub use client_dll::CCSPlayerController;
pub use client_dll::C_AttributeContainer;
pub use client_dll::C_BaseEntity;
pub use client_dll::C_CSPlayerPawnBase;
pub use client_dll::C_EconEntity;
pub use client_dll::C_EconItemView;
pub use client_dll::EntitySpottedState_t;

pub mod buttons {
    pub use super::generated::buttons::cs2_dumper::buttons::*;
}
