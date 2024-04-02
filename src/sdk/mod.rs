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

    pub mod entity {
        pub const m_iHealth: usize = 0x334;
        pub const m_fFlags: usize = 0x3D4;
        pub const m_iFOV: usize = 0x210;
        pub const m_hPlayerPawn: usize = 0x7E4;
        pub const m_iTeamNum: usize = 0x3CB;
        pub const m_AttributeManager: usize = 0x1098;
    }

    pub mod pawn {
        pub const m_pCameraServices: usize = 0x1138;
        pub const m_pClippingWeapon: usize = 0x1308;
        pub const m_entitySpottedState: usize = 0x1698;
        pub const m_bSpotted: usize = 0x8;
        pub const m_iPawnHealth: usize = 0x7F0;
        pub const m_bPawnIsAlive: usize = 0x7EC;
        pub const m_bIsScoped: usize = 0x1400;
    }

    pub mod weapon {
        pub const m_hActiveWeapon: usize = 0x58;
        pub const m_iItemDefinitionIndex: usize = 0x1BA;
    }

    pub mod attribute {
        pub const m_Item: usize = 0x50;
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

#[allow(unused_imports)]
pub mod prelude {
    pub use super::client_dll;
    pub use super::engine2_dll;
    pub use super::matchmaking_dll;

    use crate::lib::prelude::*;
    use derive_getters::Getters;
    use strum_macros::Display;

    #[derive(Getters)]
    pub struct Player {
        health: u32,
        alive: bool,
        spotted: bool,
        flags: u32,
        team: Team,
        weapon: Weapon,
        scopped: bool,
    }

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

    impl Player {
        pub fn new(process: &Memory, local_player: usize) -> Option<Self> {
            let Some(entity) = Entity::get_entity(process, local_player) else {
                return None;
            };

            let Ok(flags) = process.read::<u32>(local_player + client_dll::entity::m_fFlags) else {
                return None;
            };

            let scopped = process
                .read::<bool>(local_player + client_dll::pawn::m_bIsScoped)
                .unwrap_or_else(|_| false);

            Some(Self {
                health: entity.health,
                alive: entity.alive,
                spotted: entity.spotted,
                team: entity.team,
                weapon: entity.weapon,
                flags,
                scopped,
            })
        }

        pub fn is_standing(&self) -> bool {
            self.flags == PlayerState::Standing as u32
        }

        pub fn is_crouching(&self) -> bool {
            self.flags == PlayerState::Crouching as u32
        }

        pub fn is_grounded(&self) -> bool {
            self.flags & (1 << 0) != 0
        }
    }

    #[derive(Getters)]
    pub struct Entity {
        health: u32,
        alive: bool,
        spotted: bool,
        team: Team,
        weapon: Weapon,
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

        pub fn get_entities<'a>(process: &'a Memory, client: &'a Module) -> Option<Vec<Entity>> {
            let mut entities = Vec::new();

            let Ok(entity_list) = process.read::<usize>(client.address + client_dll::dwEntityList)
            else {
                return None;
            };

            for i in 0..64 {
                let Ok(entry) = process.read::<usize>(entity_list + 0x8 * (i >> 9) + 0x10) else {
                    continue;
                };

                let Ok(controller) = process.read::<usize>(entry + 120 * (i & 0x7FFF)) else {
                    continue;
                };

                let Ok(pawn_handle) =
                    process.read::<usize>(controller + client_dll::entity::m_hPlayerPawn)
                else {
                    continue;
                };

                if let Ok(pawn_entry) =
                    process.read::<usize>(entity_list + 0x8 * ((pawn_handle & 0x7FFF) >> 9) + 0x10)
                {
                    match process.read::<usize>(pawn_entry + 120 * (pawn_handle & 0x1FFF)) {
                        Ok(pawn) => {
                            if let Some(entity) = Entity::get_entity(process, pawn) {
                                entities.push(entity);
                            };
                        }
                        Err(_) => continue,
                    }
                };
            }

            Some(entities)
        }

        pub fn get_entity(process: &Memory, pawn: usize) -> Option<Entity> {
            let (Ok(health), Ok(is_alive), Ok(spotted), Ok(team_id)) = (
                process.read::<u32>(pawn + client_dll::pawn::m_iPawnHealth),
                process.read::<bool>(pawn + client_dll::pawn::m_bPawnIsAlive),
                process.read::<bool>(
                    pawn + client_dll::pawn::m_entitySpottedState + client_dll::pawn::m_bSpotted,
                ),
                process.read::<u8>(pawn + client_dll::entity::m_iTeamNum),
            ) else {
                return None;
            };

            let Ok(weapon_id) = process.read_pointer::<u8>(
                pawn,
                Some(&[
                    client_dll::pawn::m_pClippingWeapon,
                    client_dll::entity::m_AttributeManager
                        + client_dll::weapon::m_iItemDefinitionIndex
                        + client_dll::attribute::m_Item,
                ]),
            ) else {
                return None;
            };

            Some(Entity::new(health, is_alive, spotted, team_id, weapon_id))
        }
    }

    #[derive(Display, Clone, Copy)]
    pub enum Weapon {
        // pistols
        USP = 61,
        Glock = 4,
        Tec9 = 30,
        Revolver = 69,
        HKP2000 = 32,
        FiveSeven = 3,
        Elite = 2,
        Deagle = 1,
        CZ75 = 63,
        // assult rfiles
        M4A4 = 16,
        M4A1S = 60,
        Galil = 13,
        Famas = 10,
        AUG = 8,
        AK47 = 7,
        // smgs
        P90 = 19,
        PPBizon = 26,
        UMP45 = 24,
        MP5 = 23,
        MP9 = 34,
        MP7 = 33,
        MAC10 = 17,
        // rfiles
        SSG08 = 40,
        SCAR20 = 38,
        G38G1 = 11,
        AWP = 9,
        // shotguns
        XM1014 = 25,
        Nova = 35,
        SawedOff = 29,
        Mag7 = 27,
        // heavy assult rifles
        M249 = 14,
        Negev = 28,
        Unknown = 0,
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
                1 => Self::Deagle,
                2 => Self::Elite,
                3 => Self::FiveSeven,
                4 => Self::Glock,
                7 => Self::AK47,
                8 => Self::AUG,
                9 => Self::AWP,
                10 => Self::Famas,
                11 => Self::G38G1,
                13 => Self::Galil,
                14 => Self::M249,
                16 => Self::M4A4,
                17 => Self::MAC10,
                19 => Self::P90,
                23 => Self::MP5,
                24 => Self::UMP45,
                25 => Self::XM1014,
                26 => Self::PPBizon,
                27 => Self::Mag7,
                28 => Self::Negev,
                29 => Self::SawedOff,
                30 => Self::Tec9,
                32 => Self::HKP2000,
                33 => Self::MP7,
                34 => Self::MP9,
                35 => Self::Nova,
                38 => Self::SCAR20,
                40 => Self::SSG08,
                61 => Self::USP,
                63 => Self::CZ75,
                69 => Self::Revolver,
                _ => Self::Unknown,
            }
        }
    }

    impl Into<u8> for Weapon {
        fn into(self) -> u8 {
            self as u8
        }
    }
}
