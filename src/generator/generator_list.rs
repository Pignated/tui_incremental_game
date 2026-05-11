use std::collections::VecDeque;

use crate::{generator::Generator, resources::ResourceType};

pub struct GeneratorList<'a> {
    not_yet_used: VecDeque<Generator<'a>>,
    initial_generators: (Generator<'a>, Generator<'a>),
}

impl<'a> GeneratorList<'a> {
    pub fn default() -> Self {
        let initial_generators: (Generator, Generator);
        initial_generators = (
            Generator::blank(
                ResourceType::WOOD,
                60,
                1.05,
                1,
                1,
                "Punching Tree".to_owned(),
            )
            .add_cost((ResourceType::WOOD, 25.0)),
            Generator::blank(
                ResourceType::STONE,
                120,
                1.07,
                0,
                1,
                "Hitting rocks with sticks".to_owned(),
            )
            .add_cost((ResourceType::WOOD, 50.0)),
        );
        let mut not_yet_used = VecDeque::new();
        not_yet_used.push_front(
            Generator::blank(
                ResourceType::IRON,
                180,
                1.09,
                0,
                1,
                "Hitting shinier rocks with more rocks".to_owned(),
            )
            .add_cost((ResourceType::STONE, 50.0)),
        );
        not_yet_used.push_front(
            Generator::blank(ResourceType::WOOD, 30, 1.04, 0, 2, "Actual Axes".to_owned())
                .add_cost((ResourceType::WOOD, 10.0))
                .add_cost((ResourceType::IRON, 5.0)),
        );
        not_yet_used.push_front(
            Generator::blank(
                ResourceType::STONE,
                30,
                1.04,
                0,
                2,
                "Actual Pickaxes".to_owned(),
            )
            .add_cost((ResourceType::WOOD, 20.0))
            .add_cost((ResourceType::IRON, 10.0)),
        );

        GeneratorList {
            not_yet_used,
            initial_generators,
        }
    }
    pub fn get_initials(&self) -> (Generator<'a>, Generator<'a>) {
        return self.initial_generators.clone();
    }
    pub fn get_next(&mut self) -> Option<Generator<'a>> {
        self.not_yet_used.pop_back()
    }
}
