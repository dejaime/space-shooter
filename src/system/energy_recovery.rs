use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::component::{
    energy_component::Energy,
    player_component::{Player, PlayerSeat},
};

pub struct EnergyRecoverySystem;

impl<'s> System<'s> for EnergyRecoverySystem {
    type SystemData = (
        WriteStorage<'s, Energy>,
        ReadStorage<'s, Player>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut energies, players, time): Self::SystemData) {
        for (energy, player) in (&mut energies, &players).join() {
            if energy.time_since_last_activation < energy.recover_cooldown {
                continue;
            }

            energy.energy = energy.energy_recovery_per_second * time.delta_seconds();
            if energy.energy > energy.max_energy {
                energy.energy = energy.max_energy
            }
        }
    }
}
