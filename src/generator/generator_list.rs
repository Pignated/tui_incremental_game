use std::collections::{HashMap, VecDeque};

use ratatui::style::Color;

use crate::{
    generator::{
        Generator, GeneratorID, GeneratorRefCellWrapper, generator_save::GeneratorListSave,
    },
    resources::*,
    upgrades::Upgrade,
};
pub struct GeneratorList {
    not_yet_used: VecDeque<GeneratorRefCellWrapper>,
    all_generators: HashMap<GeneratorID, GeneratorRefCellWrapper>,
    count: u64,
}

impl GeneratorList {
    pub fn default() -> Self {
        let all_generators: HashMap<GeneratorID, GeneratorRefCellWrapper> = HashMap::new();
        let punch_tree = Generator::blank(WOOD, 20, 1.05, 1, 1, "Punching Tree".to_owned(), 0)
            .add_cost((WOOD, 25));
        let hit_rock = Generator::blank(
            STONE,
            40,
            1.07,
            0,
            1,
            "Hitting rocks with sticks".to_owned(),
            0,
        )
        .add_cost((WOOD, 50));
        let not_yet_used = VecDeque::new();
        let mut future_self = Self {
            not_yet_used,
            all_generators,
            count: 0,
        };
        future_self.add_gen(punch_tree);
        future_self.add_gen(hit_rock);
        future_self.add_gen(
            Generator::blank(
                IRON,
                60,
                1.09,
                0,
                1,
                "Hitting shinier rocks with more rocks".to_owned(),
                0,
            )
            .add_cost((STONE, 50)),
        );
        future_self.add_gen(
            Generator::blank(WOOD, 10, 1.04, 0, 2, "Actual Axes".to_owned(), 1)
                .add_cost((WOOD, 10))
                .add_cost((IRON, 5)),
        );
        future_self.add_gen(
            Generator::blank(STONE, 10, 1.04, 0, 2, "Actual Pickaxes".to_owned(), 1)
                .add_cost((WOOD, 20))
                .add_cost((IRON, 10)),
        );
        future_self.add_gen(
            Generator::blank(
                COPPER,
                20,
                1.12,
                0,
                1,
                "Breaking into Houses to Steal their Wiring".to_owned(),
                0,
            )
            .add_cost((IRON, 15)),
        );
        future_self.add_gen(
            Generator::blank(IRON, 20, 1.09, 0, 1, "Electric Drills".to_owned(), 0)
                .add_cost((COPPER, 30))
                .add_cost((IRON, 20)),
        );
        future_self.add_gen(
            Generator::blank(GOLD, 80, 1.19, 0, 1, "Sieging the Vault".to_owned(), 0)
                .add_cost((IRON, 100))
                .add_cost((STONE, 100))
                .add_cost((WOOD, 100))
                .add_cost((COPPER, 100)),
        );
        future_self.add_gen(
            Generator::blank(RUBY, 100, 1.23, 0, 1, "Artisanal Ruby Mine".to_owned(), 0)
                .add_cost((WOOD, 150))
                .add_cost((STONE, 150))
                .add_cost((COPPER, 200)),
        );
        future_self
    }
    pub fn get_next(&mut self) -> Option<GeneratorRefCellWrapper> {
        self.not_yet_used.pop_back()
    }
    fn add_gen(&mut self, gener: Generator) {
        let gen_ref = GeneratorRefCellWrapper::new(gener, self.count);
        self.count += 1;
        self.all_generators
            .insert(gen_ref.borrow().id.clone(), gen_ref.clone());
        self.not_yet_used.push_front(gen_ref.clone());
    }
    pub fn apply_upgrade(&mut self, upgrade: Upgrade) {
        if let Some(gener) = self.all_generators.get_mut(&upgrade.effected_generator) {
            gener.upgrade(upgrade);
        }
    }
    pub fn get_gener_name(&self, id: GeneratorID) -> Option<String> {
        if let Some(gener) = self.all_generators.get(&id) {
            Some(gener.borrow().generator_name.clone())
        } else {
            None
        }
    }

    pub fn get_gener_color(&self, id: GeneratorID) -> Color {
        if let Some(gener) = self.all_generators.get(&id) {
            gener.borrow().resource_type.color
        } else {
            Color::White
        }
    }
    pub fn add_in_app_gen(&mut self, gener: GeneratorRefCellWrapper) {
        self.all_generators.insert(gener.borrow().id, gener.clone());
    }
    pub fn from_save(mut saves: GeneratorListSave) -> Self {
        let mut new_me = GeneratorList {
            not_yet_used: VecDeque::new(),
            all_generators: HashMap::new(),
            count: 0,
        };
        saves.generators.sort_by(|a, b| a.idx.cmp(&b.idx));
        for save in saves.generators {
            new_me.add_gen(save.to_gen());
            new_me.count += 1;
        }
        new_me
    }
    pub fn to_save(&self) -> GeneratorListSave {
        GeneratorListSave::new(self.not_yet_used.clone())
    }
}
