use super::memory::*;
use super::offsets::*;

use derive_builder::Builder;
use derive_getters::Getters;
use strum_macros::{Display, FromRepr};

pub use super::offsets;

pub enum PlayerState {
    Standing = 65665,
    Crouching = 65667,
}

pub enum Modifier {
    Plus = 65537,
    Minus = 256,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Team {
    #[default]
    Unknown,
    Terrorist = 2,
    CounterStrike = 3,
}

impl From<u8> for Team {
    fn from(value: u8) -> Self {
        match value {
            2 => Self::Terrorist,
            3 => Self::CounterStrike,
            _ => Self::Unknown,
        }
    }
}

#[derive(Default, Getters, Clone, Copy)]
pub struct Player {
    health: u32,
    #[getter(rename = "is_spotted")]
    flags: u32,
    team: Team,
    weapon: Weapon,
    is_alive: bool,
    is_scopped: bool,
    entity_index: Option<i32>,
    base_address: usize,
}

impl Player {
    pub fn read(process: &Memory, local_player: usize) -> Option<Self> {
        let client = process.modules.get("client.dll").unwrap();
        let Ok(local_controller) = process.read::<usize>(client.address + DW_LOCAL_PAWN_CONTROLLER)
        else {
            return None;
        };

        let is_scopped = process
            .read::<bool>(local_player + C_CSPlayerPawnBase::m_bIsScoped)
            .unwrap_or_else(|_| false);

        let Ok(flags) = process.read::<u32>(local_player + C_BaseEntity::m_fFlags) else {
            return None;
        };

        let entity_index = match process
            .read::<i32>(local_player + C_CSPlayerPawnBase::m_iIDEntIndex)
            .ok()
        {
            Some(id) => {
                if id > 0 {
                    Some(id)
                } else {
                    None
                }
            }
            None => None,
        };

        let Ok(health) = process.read::<u32>(local_player + C_BaseEntity::m_iHealth) else {
            return None;
        };

        let Ok(is_alive) =
            process.read::<bool>(local_controller + CCSPlayerController::m_bPawnIsAlive)
        else {
            return None;
        };

        let Ok(team_id) = process.read::<u8>(local_player + C_BaseEntity::m_iTeamNum) else {
            return None;
        };

        let Ok(weapon_id) = process.read_pointer::<u8>(
            local_player + C_CSPlayerPawnBase::m_pClippingWeapon,
            Some(&[C_EconEntity::m_AttributeManager
                + C_EconItemView::m_iItemDefinitionIndex
                + C_AttributeContainer::m_Item]),
        ) else {
            return None;
        };

        Some(Self {
            health,
            is_alive,
            team: Team::from(team_id),
            weapon: Weapon::from(weapon_id),
            flags,
            is_scopped,
            entity_index,
            base_address: local_player,
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

#[derive(Getters, Builder, Clone, Copy, Debug)]
pub struct Entity {
    health: u32,
    alive: bool,
    spotted: bool,
    team: Team,
    weapon: Weapon,
    base_address: usize,
}

impl Entity {
    pub fn read_entities<'a>(process: &'a Memory, client: &'a Module) -> Option<Vec<Entity>> {
        let mut entities = Vec::new();

        let Ok(entity_list) = process.read::<usize>(client.address + DW_ENTITY_LIST) else {
            return None;
        };

        for i in 0..64 {
            let Ok(entry) = process.read::<usize>(entity_list + 0x8 * (i >> 9) + 0x10) else {
                continue;
            };

            let Ok(controller) = process.read::<usize>(entry + 120 * (i & 0x1FF)) else {
                continue;
            };

            let Ok(pawn_handle) =
                process.read::<usize>(controller + CCSPlayerController::m_hPlayerPawn)
            else {
                continue;
            };

            if let Ok(pawn_entry) =
                process.read::<usize>(entity_list + 0x8 * ((pawn_handle & 0x7FFF) >> 9) + 0x10)
            {
                match process.read::<usize>(pawn_entry + 120 * (pawn_handle & 0x1FF)) {
                    Ok(pawn) => {
                        // let Ok(health) = process.read::<u32>(pawn + C_BaseEntity::m_iHealth) else {
                        //     return None;
                        // };
                        let Ok(health) =
                            process.read::<u32>(controller + CCSPlayerController::m_iPawnHealth)
                        else {
                            return None;
                        };

                        let Ok(is_alive) =
                            process.read::<bool>(controller + CCSPlayerController::m_bPawnIsAlive)
                        else {
                            return None;
                        };

                        let Ok(spotted) = process.read::<bool>(
                            pawn + C_CSPlayerPawnBase::m_entitySpottedState
                                + EntitySpottedState_t::m_bSpotted,
                        ) else {
                            return None;
                        };

                        let Ok(team_id) = process.read::<u8>(pawn + C_BaseEntity::m_iTeamNum)
                        else {
                            return None;
                        };

                        let weapon_id = process
                            .read_pointer::<u8>(
                                pawn + C_CSPlayerPawnBase::m_pClippingWeapon,
                                Some(&[C_EconEntity::m_AttributeManager
                                    + C_EconItemView::m_iItemDefinitionIndex
                                    + C_AttributeContainer::m_Item]),
                            )
                            .unwrap_or_else(|_| 0);

                        // println!("{}", weapon_id);
                        let entity = EntityBuilder::default()
                            .health(health)
                            .alive(is_alive)
                            .spotted(spotted)
                            .team(Team::from(team_id))
                            .weapon(Weapon::from(weapon_id))
                            .base_address(pawn)
                            .build()
                            .unwrap();

                        entities.push(entity);
                    }
                    Err(_) => continue,
                }
            };
        }

        Some(entities)
    }
}

#[derive(Display, Clone, Copy, Debug, FromRepr)]
#[repr(u8)]
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
    // other???
    Grenade = 44,
    Smoke = 45,
    Molotov = 46,
    FlashBang = 43,
    Decoy = 47,
    Incendiary = 48,
    C4 = 49,
    Unknown(u8),
}

impl Default for Weapon {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl Weapon {
    pub fn name(&self) -> String {
        self.to_string()
    }

    pub fn id(&self) -> u8 {
        match self {
            Self::Unknown(id) => *id,
            _ => 0,
        }
    }

    pub fn is_throwable(&self) -> bool {
        match self {
            Self::Grenade | Self::Smoke | Self::FlashBang | Self::Molotov | Self::Incendiary => {
                true
            }
            _ => false,
        }
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
            44 => Self::Grenade,
            45 => Self::Smoke,
            46 => Self::Molotov,
            43 => Self::FlashBang,
            47 => Self::Decoy,
            48 => Self::Incendiary,
            49 => Self::C4,
            61 => Self::USP,
            63 => Self::CZ75,
            69 => Self::Revolver,
            _ => Self::Unknown(value),
        }
    }
}
