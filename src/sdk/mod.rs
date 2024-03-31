#![allow(non_snake_case, dead_code, non_upper_case_globals)]
pub mod client_dll {
    pub const dwEntityList: usize = 0x18C2D58;
    pub const dwForceAttack: usize = 0x1730020;
    pub const dwForceAttack2: usize = 0x17300B0;
    pub const dwForceBackward: usize = 0x17302F0;
    pub const dwForceCrouch: usize = 0x17305C0;
    pub const dwForceForward: usize = 0x1730260;
    pub const dwForceJump: usize = 0x1730530;
    pub const dwForceLeft: usize = 0x1730380;
    pub const dwForceRight: usize = 0x1730410;
    pub const dwGameEntitySystem: usize = 0x19E0790;
    pub const dwGameEntitySystem_getHighestEntityIndex: usize = 0x1510;
    pub const dwGameRules: usize = 0x191FCA0;
    pub const dwGlobalVars: usize = 0x172ABA0;
    pub const dwGlowManager: usize = 0x19200C0;
    pub const dwInterfaceLinkList: usize = 0x1A118D8;
    pub const dwLocalPlayerController: usize = 0x1912578;
    pub const dwLocalPlayerPawn: usize = 0x17371A8;
    pub const dwPlantedC4: usize = 0x1928AD8;
    pub const dwPrediction: usize = 0x1737070;
    pub const dwSensitivity: usize = 0x19209E8;
    pub const dwSensitivity_sensitivity: usize = 0x40;
    pub const dwViewAngles: usize = 0x19309B0;
    pub const dwViewMatrix: usize = 0x19241A0;
    pub const dwViewRender: usize = 0x1924A20;

    pub mod config {
        pub const m_iHealth: usize = 0x334;
        pub const m_pCameraServices: usize = 0x1138;
        pub const m_fFlags: usize = 0x3D4;
        pub const m_iFOV: usize = 0x210;
        pub const m_hActiveWeapon: usize = 0x58;
        pub const m_pClippingWeapon: usize = 0x1308;
        pub const m_hPlayerPawn: usize = 0x7E4;
        pub const m_bIsScoped: usize = 0x1400;
        pub const m_bPawnIsAlive: usize = 0x7EC;
        pub const m_iTeamNum: usize = 0x3CB;
        pub const m_entitySpottedState: usize = 0x1698;
        pub const m_bSpotted: usize = 0x8;
    }
}

pub mod engine2_dll {
    pub const dwBuildNumber: usize = 0x514574;
    pub const dwNetworkGameClient_deltaTick: usize = 0x258;
    pub const dwNetworkGameClient: usize = 0x513AC8;
    pub const dwNetworkGameClient_getLocalPlayer: usize = 0xF0;
    pub const dwNetworkGameClient_maxClients: usize = 0x250;
    pub const dwNetworkGameClient_signOnState: usize = 0x240;
    pub const dwWindowHeight: usize = 0x5CCCDC;
    pub const dwWindowWidth: usize = 0x5CCCD8;
}

pub mod matchmaking_dll {
    pub const dwGameTypes: usize = 0x1D21E0;
    pub const dwGameTypes_mapName: usize = 0x1D2300;
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
