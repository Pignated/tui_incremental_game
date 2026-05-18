use ratatui::style::Color;

use crate::{
    generator::GeneratorID,
    resources::{ResourceType, resource_array::ResValArray},
    upgrades::Upgrade,
};

pub struct UpgradeManager<'a> {
    pub ready_upgrades: Vec<Upgrade<'a>>,
    pending_upgrades: Vec<Upgrade<'a>>,
}
impl<'a> UpgradeManager<'a> {
    pub fn new() -> Self {
        let ready_upgrades = Vec::new();
        let mut pending_upgrades = Vec::new();
        pending_upgrades.push(
            Upgrade::new(
                GeneratorID::new(ResourceType::WOOD, 0),
                Some(2),
                None,
                Color::Red,
                String::from("I mean c'mon, just punch faster man"),
                String::from("Punch trees harder"),
            )
            .add_cost(100, ResourceType::WOOD)
            .add_requirement(500, ResourceType::WOOD),
        );
        UpgradeManager {
            ready_upgrades,
            pending_upgrades,
        }
    }
    pub fn poll_requirement_reached(&mut self, totals: ResValArray) {
        self.pending_upgrades.retain(|item| {
            if totals.gte_all(item.requirements) {
                self.ready_upgrades.push(item.clone());
                false
            } else {
                true
            }
        })
    }
    pub fn get_upgrade(&'a self, idx: usize) -> Upgrade<'a> {
        self.ready_upgrades[idx].clone()
    }
    pub fn remove_upgrade(&mut self, idx: usize) {
        self.ready_upgrades.remove(idx);
    }
}
