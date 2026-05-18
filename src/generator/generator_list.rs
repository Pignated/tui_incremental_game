use std::collections::{HashMap, VecDeque};

use crate::{
    generator::{Generator, GeneratorID, GeneratorRefCellWrapper},
    resources::ResourceType,
    upgrades::Upgrade,
};

pub struct GeneratorList<'a> {
    not_yet_used: VecDeque<GeneratorRefCellWrapper<'a>>,
    all_generators: HashMap<GeneratorID, GeneratorRefCellWrapper<'a>>,
}

impl<'a> GeneratorList<'a> {
    pub fn default() -> Self {
        let all_generators: HashMap<GeneratorID, GeneratorRefCellWrapper<'a>> = HashMap::new();
        let punch_tree = Generator::blank(
            ResourceType::WOOD,
            60,
            1.05,
            1,
            1,
            "Punching Tree".to_owned(),
            0,
        )
        .add_cost((ResourceType::WOOD, 25.0));
        let hit_rock = Generator::blank(
            ResourceType::STONE,
            120,
            1.07,
            0,
            1,
            "Hitting rocks with sticks".to_owned(),
            0,
        )
        .add_cost((ResourceType::WOOD, 50.0));
        let not_yet_used = VecDeque::new();
        let mut future_self = Self {
            not_yet_used,
            all_generators,
        };
        future_self.add_gen(punch_tree);
        future_self.add_gen(hit_rock);
        future_self.add_gen(
            Generator::blank(
                ResourceType::IRON,
                180,
                1.09,
                0,
                1,
                "Hitting shinier rocks with more rocks".to_owned(),
                0,
            )
            .add_cost((ResourceType::STONE, 50.0)),
        );
        future_self.add_gen(
            Generator::blank(
                ResourceType::WOOD,
                30,
                1.04,
                0,
                2,
                "Actual Axes".to_owned(),
                1,
            )
            .add_cost((ResourceType::WOOD, 10.0))
            .add_cost((ResourceType::IRON, 5.0)),
        );
        future_self.add_gen(
            Generator::blank(
                ResourceType::STONE,
                30,
                1.04,
                0,
                2,
                "Actual Pickaxes".to_owned(),
                1,
            )
            .add_cost((ResourceType::WOOD, 20.0))
            .add_cost((ResourceType::IRON, 10.0)),
        );
        future_self.add_gen(
            Generator::blank(
                ResourceType::COPPER,
                60,
                1.12,
                0,
                1,
                "Breaking into Houses to Steal their Wiring".to_owned(),
                0,
            )
            .add_cost((ResourceType::IRON, 15.0)),
        );
        future_self.add_gen(
            Generator::blank(
                ResourceType::IRON,
                60,
                1.09,
                0,
                1,
                "Electric Drills".to_owned(),
                0,
            )
            .add_cost((ResourceType::COPPER, 30.0))
            .add_cost((ResourceType::IRON, 20.0)),
        );
        future_self.add_gen(
            Generator::blank(
                ResourceType::GOLD,
                240,
                1.19,
                0,
                1,
                "Sieging the Vault".to_owned(),
                0,
            )
            .add_cost((ResourceType::IRON, 100.0))
            .add_cost((ResourceType::STONE, 100.0))
            .add_cost((ResourceType::WOOD, 100.0))
            .add_cost((ResourceType::COPPER, 100.0)),
        );
        future_self.add_gen(
            Generator::blank(
                ResourceType::RUBY,
                300,
                1.23,
                0,
                1,
                "Artisanal Ruby Mine".to_owned(),
                0,
            )
            .add_cost((ResourceType::WOOD, 150.0))
            .add_cost((ResourceType::STONE, 150.0))
            .add_cost((ResourceType::COPPER, 200.0)),
        );
        future_self
    }
    pub fn get_next(&mut self) -> Option<GeneratorRefCellWrapper<'a>> {
        self.not_yet_used.pop_back()
    }
    fn add_gen(&mut self, gener: Generator<'a>) {
        let gen_ref = GeneratorRefCellWrapper::new(gener);
        self.all_generators
            .insert(gen_ref.borrow().id.clone(), gen_ref.clone());
        self.not_yet_used.push_front(gen_ref.clone());
    }
    pub fn apply_upgrade(&mut self, upgrade: Upgrade) {
        if let Some(gener) = self.all_generators.get_mut(&upgrade.effected_generator) {
            gener.upgrade(upgrade);
        }
    }
}
