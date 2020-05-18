use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, System, WriteStorage},
};

use crate::component::energy_component::Energy;

pub struct EnergyRecoverySystem;

impl<'s> System<'s> for EnergyRecoverySystem {
    type SystemData = (WriteStorage<'s, Energy>, Read<'s, Time>);

    fn run(&mut self, (mut energies, time): Self::SystemData) {
        for energy in (&mut energies).join() {
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
