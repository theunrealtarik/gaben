use sdk::offsets::*;
use sdk::prelude::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// All continuous punishments joined together
pub struct ContinuousPunishments;

impl ContinuousPunishments {
    pub fn run(process: &Process, player: Arc<Option<Player>>, entities: Arc<Option<Vec<Entity>>>) {
        thread::sleep(Duration::from_millis(16u64));

        let modules = &process.modules;
        if let Some(player) = player.as_ref() {
            if *player.is_alive() {
                Self::apply_flame_punishment(process, modules, player);
                Self::apply_heavy_knife_punishment(player);
                Self::apply_cursed_snipers_punishment(player);
                Self::apply_slippery_nades_punishment(player);

                if let Some(entities) = entities.as_ref() {
                    Self::apply_slippery_weapons_punishment(player, entities);
                    Self::apply_fragile_trigger_punishment(process, modules, player);
                };
            }
        }
    }

    /// *Flame Grant Me Strength*:
    /// Whenever the cheater holds a molotov or an incendiary, he will automatically throw it
    /// with a short-throw (mouse right button hold)
    fn apply_flame_punishment(process: &Process, modules: &Modules, player: &Player) {
        let client = modules.get("client.dll").unwrap().address;

        if player.weapon().is_molotov() || player.weapon().is_incendiary() {
            process
                .write(client + offsets::buttons::attack2, Modifier::Plus)
                .unwrap();
            thread::sleep(Duration::from_millis(10));
            process
                .write(client + offsets::buttons::attack2, Modifier::Minus)
                .unwrap();
        }
    }

    /// *Heavy Knife*:
    /// Since melees in the source engine makes the player run faster, this punishment
    /// holds the LShfit key whenever the cheater has a knife in hands which will make him
    /// run solwer
    fn apply_heavy_knife_punishment(player: &Player) {
        if player.weapon().is_knife() {
            Keyboard::stroke(Key::LShiftKey);
        }
    }

    /// *Cursed Snipers*
    fn apply_cursed_snipers_punishment(player: &Player) {
        match *player.weapon() {
            Weapon::AWP | Weapon::SSG08 if *player.is_scopped() => {
                Keyboard::stroke(Key::QKey);
            }
            _ => {}
        }
    }

    /// *Slippery Nades*:
    /// Everytime the cheater tries to throw a nade ([Weapon::Grenade] [Weapon::FlashBang]
    /// [Weapon::Smoke] [Weapon::Decoy]) he will drop it instead
    fn apply_slippery_nades_punishment(player: &Player) {
        if player.weapon().is_throwable()
            && !(player.weapon().is_molotov() || player.weapon().is_incendiary())
            && (MouseButton::LeftButton.is_pressed() || MouseButton::RightButton.is_pressed())
        {
            Keyboard::stroke(Key::GKey);
        }
    }

    /// *Slippery Weapons*:
    /// If the cheater started firing with his weapon while his team or himself has spotted an
    /// enemy, there's a 50% chance at every tick that he will drop his weapon automatically
    /// if the previous two conditions are also met
    fn apply_slippery_weapons_punishment(player: &Player, entities: &Vec<Entity>) {
        let spotted_entities = entities
            .iter()
            .filter(|entity| *entity.spotted() && !entity.team().is_unknown())
            .count();

        if spotted_entities > 0
            && MouseButton::LeftButton.is_pressed()
            && rand::random::<bool>()
            && !player.weapon().is_throwable()
            && !player.weapon().is_knife()
        {
            Keyboard::stroke(Key::GKey);
        }
    }

    /// *Fragile Trigger*:
    /// If  the cheater aimed at his teammates, he will automatically shot at'em
    fn apply_fragile_trigger_punishment(process: &Process, modules: &Modules, player: &Player) {
        let client = modules.get("client.dll").unwrap();

        if let Some(entity_id) = player.entity_index() {
            let Ok(entity_list) = process.read::<usize>(client.address + DW_ENTITY_LIST) else {
                return;
            };

            let Ok(entry) = process.read::<usize>(entity_list + 0x8 * (entity_id >> 9) + 0x10)
            else {
                return;
            };

            let Ok(controller) = process.read::<usize>(entry + 120 * (entity_id & 0x1FF)) else {
                return;
            };

            let entity_team =
                match process.read::<u8>(controller + offsets::C_BaseEntity::m_iTeamNum) {
                    Ok(id) => Team::from(id),
                    Err(_) => return,
                };

            dbg!(&player.team());
            dbg!(&entity_team);

            match (player.team(), entity_team) {
                (Team::CounterStrike, Team::Terrorist)
                | (Team::CounterStrike, Team::CounterStrike) => {
                    Mouse::click(MouseButton::LeftButton)
                }
                _ => return,
            }
        }
    }
}
