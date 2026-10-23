pub mod ecology;
pub mod engine;
pub mod fitness;
pub mod ga;
pub mod scenario;
pub mod trainer;

pub use ecology::{EcologyReport, EcologyTrainer};
pub use engine::{Engine, Inspect};
pub use osmos_core::brain::GENOME_LENGTH;
pub use scenario::Scenario;
pub use trainer::{ProbeReport, Trainer, TrainerReport};
