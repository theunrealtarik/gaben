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
        pub const m_iTeamNum: usize = 0x3CB;
        pub const m_entitySpottedState: usize = 0x1698;
        pub const m_bSpotted: usize = 0x8;
        pub const m_iPawnHealth: usize = 0x7F0;
        pub const m_bPawnIsAlive: usize = 0x7EC;
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
    use strum_macros::Display;

    pub enum PlayerState {
        Standing = 65665,
        Crouching = 65667,
    }

    pub enum Modifier {
        Plus = 65537,
        Minus = 256,
    }

    pub enum Team {
        Unknown,
        Terrorist = 2,
        CounterStrike = 3,
    }

    pub struct Entity {
        pub health: u32,
        pub alive: bool,
        pub spotted: bool,
        pub team: Team,
        pub weapon: Weapon,
    }

    impl Entity {
        pub fn new(health: u32, alive: bool, spotted: bool, team_num: u8, weapon_id: u8) -> Self {
            Self {
                health,
                alive,
                spotted,
                team: match team_num {
                    2 => Team::Terrorist,
                    3 => Team::CounterStrike,
                    _ => Team::Unknown,
                },
                weapon: Weapon::from(weapon_id),
            }
        }
    }

    #[derive(Display, Clone, Copy)]
    pub enum Weapon {
        USP = 61,
        Glock = 4,
        Tec9 = 30,
        Revolver = 69,
        HKP2000 = 32,
        FiveSeven = 3,
        Elite = 2,
        Deagle = 1,
        CZ75 = 63,
        M4A4 = 16,
        M4A1S = 60,
        Galil = 13,
        Famas = 10,
        AUG = 8,
        AK47 = 7,

        P90 = 19,
        PPBizon = 26,
        UMP45 = 24,
        MP5 = 23,
        MP9 = 34,
        MP7 = 33,
        MAC10 = 17,

        SSG08 = 40,
        SCAR20 = 38,
        G38G1 = 11,
        AWP = 9,

        XM1014 = 25,
        Nova = 35,
        SawedOff = 29,
        Mag7 = 27,

        M249 = 14,
        Negev = 28,
    }

    impl Weapon {
        pub fn name(&self) -> String {
            self.to_string()
        }

        pub fn id(&self) -> u8 {
            *self as u8
        }
    }

    impl From<u8> for Weapon {
        fn from(value: u8) -> Self {
            match value {
                61 => Weapon::USP,
                4 => Weapon::Glock,
                30 => Weapon::Tec9,
                69 => Weapon::Revolver,
                32 => Weapon::HKP2000,
                3 => Weapon::FiveSeven,
                2 => Weapon::Elite,
                1 => Weapon::Deagle,
                63 => Weapon::CZ75,
                16 => Weapon::M4A4,
                60 => Weapon::M4A1S,
                13 => Weapon::Galil,
                10 => Weapon::Famas,
                8 => Weapon::AUG,
                7 => Weapon::AK47,
                19 => Weapon::P90,
                26 => Weapon::PPBizon,
                24 => Weapon::UMP45,
                23 => Weapon::MP5,
                34 => Weapon::MP9,
                33 => Weapon::MP7,
                17 => Weapon::MAC10,
                40 => Weapon::SSG08,
                38 => Weapon::SCAR20,
                11 => Weapon::G38G1,
                9 => Weapon::AWP,
                25 => Weapon::XM1014,
                35 => Weapon::Nova,
                29 => Weapon::SawedOff,
                27 => Weapon::Mag7,
                14 => Weapon::M249,
                28 => Weapon::Negev,
                _ => panic!("Unknown weapon ID: {}", value),
            }
        }
    }

    impl Into<u8> for Weapon {
        fn into(self) -> u8 {
            self as u8
        }
    }
}

#[allow(unused_imports)]
pub mod prelude {
    pub use super::client_dll;
    pub use super::engine2_dll;
    pub use super::matchmaking_dll;
    pub use super::types::*;
}
