use crate::{
    generator::{GeneratorID, generator_list::GeneratorList},
    resources::{resource_array::ResValArray, *},
    upgrades::{Upgrade, upgrade_save::UpgradeListSave},
};

pub struct UpgradeManager {
    pub ready_upgrades: Vec<Upgrade>,
    pending_upgrades: Vec<Upgrade>,
}
impl UpgradeManager {
    pub fn new(generator_list: &GeneratorList) -> Self {
        let ready_upgrades = Vec::new();
        let mut pending_upgrades = Vec::new();
        pending_upgrades.push(
            Upgrade::new(
                GeneratorID::new(WOOD, 0),
                Some(2),
                None,
                String::from("I mean c'mon, just punch faster man"),
                String::from("Punch trees faster"),
                generator_list,
            )
            .add_cost(100, WOOD)
            .add_requirement(500, WOOD),
        );
        pending_upgrades.push(Upgrade::new_output(
            GeneratorID::new(STONE, 0),
            2,
            String::from("Those are some thick ass branches"),
            String::from("Use bigger branches"),
            ResValArray::new().add_cost(1000, WOOD),
            ResValArray::new().add_cost(500, STONE),
            generator_list,
        ));
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
    pub fn get_upgrade(self, idx: usize) -> Upgrade {
        self.ready_upgrades[idx].clone()
    }
    pub fn remove_upgrade(&mut self, idx: usize) {
        self.ready_upgrades.remove(idx);
    }
    pub fn from_save(upgrade_save_list: UpgradeListSave) -> Self {
        UpgradeManager {
            ready_upgrades: upgrade_save_list
                .ready_upgrades
                .iter()
                .map(|a| a.to_upgr())
                .collect(),
            pending_upgrades: upgrade_save_list
                .pending_upgrades
                .iter()
                .map(|a| a.to_upgr())
                .collect(),
        }
    }
    pub fn to_save(self) -> UpgradeListSave {
        UpgradeListSave::new(self.ready_upgrades, self.pending_upgrades)
    }
}
