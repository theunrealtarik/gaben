#![allow(non_snake_case, dead_code, non_upper_case_globals)]
pub mod client_dll {
    pub const dwEntityList: isize = 0x18C2D58;
    pub const dwForceAttack: isize = 0x1730020;
    pub const dwForceAttack2: isize = 0x17300B0;
    pub const dwForceBackward: isize = 0x17302F0;
    pub const dwForceCrouch: isize = 0x17305C0;
    pub const dwForceForward: isize = 0x1730260;
    pub const dwForceJump: isize = 0x1730530;
    pub const dwForceLeft: isize = 0x1730380;
    pub const dwForceRight: isize = 0x1730410;
    pub const dwGameEntitySystem: isize = 0x19E0790;
    pub const dwGameEntitySystem_getHighestEntityIndex: isize = 0x1510;
    pub const dwGameRules: isize = 0x191FCA0;
    pub const dwGlobalVars: isize = 0x172ABA0;
    pub const dwGlowManager: isize = 0x19200C0;
    pub const dwInterfaceLinkList: isize = 0x1A118D8;
    pub const dwLocalPlayerController: isize = 0x1912578;
    pub const dwLocalPlayerPawn: isize = 0x17371A8;
    pub const dwPlantedC4: isize = 0x1928AD8;
    pub const dwPrediction: isize = 0x1737070;
    pub const dwSensitivity: isize = 0x19209E8;
    pub const dwSensitivity_sensitivity: isize = 0x40;
    pub const dwViewAngles: isize = 0x19309B0;
    pub const dwViewMatrix: isize = 0x19241A0;
    pub const dwViewRender: isize = 0x1924A20;

    pub mod entity {
        pub const m_iHealth: isize = 0x334; // int32_t
        pub const m_fFlags: isize = 0x3D4;
    }
}

pub mod engine2_dll {
    pub const dwBuildNumber: isize = 0x514574;
    pub const dwNetworkGameClient_deltaTick: isize = 0x258;
    pub const dwNetworkGameClient: isize = 0x513AC8;
    pub const dwNetworkGameClient_getLocalPlayer: isize = 0xF0;
    pub const dwNetworkGameClient_maxClients: isize = 0x250;
    pub const dwNetworkGameClient_signOnState: isize = 0x240;
    pub const dwWindowHeight: isize = 0x5CCCDC;
    pub const dwWindowWidth: isize = 0x5CCCD8;
}

pub mod matchmaking_dll {
    pub const dwGameTypes: isize = 0x1D21E0;
    pub const dwGameTypes_mapName: isize = 0x1D2300;
}

pub mod types {

    pub enum PlayerTeam {
        CounterTerrorist = 2,
        Terrorist = 3,
        Unknown,
    }

    pub enum PlayerState {
        Standing = 65665,
        Crouching = 65667,
    }

    pub enum PlayerJump {
        Plus = 65537,
        Minus = 256,
    }
}

#[allow(unused_imports)]
pub mod prelude {
    pub use super::client_dll;
    pub use super::engine2_dll;
    pub use super::matchmaking_dll;
    pub use super::types::*;
}
