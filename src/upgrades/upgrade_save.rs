use ratatui::style::Style;
use serde::{Deserialize, Serialize};

use crate::{
    generator::generator_save::GeneratorIDSave,
    resources::{RESOURCE_COUNT, resource_array::ResValArray},
    upgrades::Upgrade,
};

#[derive(Serialize, Deserialize)]
pub struct UpgradeSave {
    effected_generator: GeneratorIDSave,
    speed_modifier: Option<u64>,
    output_modifier: Option<u64>,
    generator_name: String,
    description: String,
    name: String,
    cost: [usize; RESOURCE_COUNT],
    requirements: [usize; RESOURCE_COUNT],
}

impl UpgradeSave {
    pub fn new(
        effected_generator: GeneratorIDSave,
        speed_modifier: Option<usize>,
        output_modifier: Option<usize>,
        generator_name: String,
        description: String,
        name: String,
        cost: ResValArray,
        requirements: ResValArray,
    ) -> Self {
        UpgradeSave {
            effected_generator,
            speed_modifier: speed_modifier.map(|v| v as u64),
            output_modifier: output_modifier.map(|v| v as u64),
            generator_name,
            description,
            name,
            cost: cost.0,
            requirements: requirements.0,
        }
    }
    pub fn to_upgr(&self) -> Upgrade {
        Upgrade::from_save(
            self.effected_generator.to_id(),
            self.speed_modifier.map(|v| v as usize),
            self.output_modifier.map(|v| v as usize),
            self.generator_name.clone(),
            self.description.clone(),
            self.name.clone(),
            ResValArray(self.cost),
            ResValArray(self.requirements),
            Style::new(),
        )
    }
}
#[derive(Serialize, Deserialize)]
pub struct UpgradeListSave {
    pub ready_upgrades: Vec<UpgradeSave>,
    pub pending_upgrades: Vec<UpgradeSave>,
}
impl UpgradeListSave {
    pub fn new(ready: Vec<Upgrade>, pending: Vec<Upgrade>) -> Self {
        UpgradeListSave {
            ready_upgrades: ready.iter().map(|a| a.to_save()).collect(),
            pending_upgrades: pending.iter().map(|a| a.to_save()).collect(),
        }
    }
}
