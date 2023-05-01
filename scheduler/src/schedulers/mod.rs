// Exposem els diferents schedulers per a que puguin ser utilitzats fora del crate.
pub mod cyclic;
pub mod monotonic;
pub mod edf;

// Import global per a que tots els membres d'aquest modul puguin usar lcm i gcd.
pub use num::integer::{lcm, gcd};

// Import global per a que tots els membres d'aquest modul puguin usar els resultats d'schedulability
pub use crate::SchedulabilityResult;

/**
 * Trait queassgura que qui l'implementi, te un mètode per a poder
 * comprovar la seva planificabilitat.
 */
pub trait CheckSchedulable {
    fn is_schedulable(&mut self) -> SchedulabilityResult;
}
