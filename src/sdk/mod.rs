pub mod offsets;

#[allow(unused_imports)]
pub mod prelude {
    use super::offsets::*;
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

    #[derive(Debug)]
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

            let Ok(flags) = process.read::<u32>(local_player + C_BaseEntity::m_fFlags) else {
                return None;
            };

            let scopped = process
                .read::<bool>(local_player + C_CSPlayerPawnBase::m_bIsScoped)
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

            let Ok(entity_list) = process.read::<usize>(client.address + DW_ENTITY_LIST) else {
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
                    process.read::<usize>(controller + CCSPlayerController::m_hPlayerPawn)
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
                process.read::<u32>(pawn + CCSPlayerController::m_iPawnHealth),
                process.read::<bool>(pawn + CCSPlayerController::m_bPawnIsAlive),
                process.read::<bool>(
                    pawn + C_CSPlayerPawnBase::m_entitySpottedState
                        + EntitySpottedState_t::m_bSpotted,
                ),
                process.read::<u8>(pawn + C_BaseEntity::m_iTeamNum),
            ) else {
                return None;
            };

            let Ok(weapon_id) = process.read_pointer::<u8>(
                pawn + C_CSPlayerPawnBase::m_pClippingWeapon,
                Some(&[C_EconEntity::m_AttributeManager
                    + C_EconItemView::m_iItemDefinitionIndex
                    + C_AttributeContainer::m_Item]),
            ) else {
                return None;
            };

            Some(Entity::new(
                health,
                is_alive,
                spotted,
                team_id,
                weapon_id as u8,
            ))
        }
    }

    #[derive(Display, Clone, Copy, Debug)]
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
        C4 = 49,
        Grenade = 44,
        Smoke = 45,
        FlashBang = 43,
        Decoy = 47,
        Unknown = 0,
    }

    impl Weapon {
        pub fn name(&self) -> String {
            self.to_string()
        }

        pub fn id(&self) -> u8 {
            *self as u8
        }

        pub fn is_throwable(&self) -> bool {
            match self {
                Self::Grenade | Self::Smoke | Self::FlashBang | Self::Decoy => true,
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
                49 => Self::C4,
                44 => Self::Grenade,
                45 => Self::Smoke,
                43 => Self::FlashBang,
                47 => Self::Decoy,
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
