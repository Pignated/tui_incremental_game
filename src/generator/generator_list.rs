use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::{generator::Generator, resources::ResourceType};

pub struct GeneratorList<'a> {
    not_yet_used: VecDeque<Rc<RefCell<Generator<'a>>>>,
    initial_generators: (Rc<RefCell<Generator<'a>>>, Rc<RefCell<Generator<'a>>>),
    all_generators: HashMap<String, Rc<RefCell<Generator<'a>>>>,
}

impl<'a> GeneratorList<'a> {
    pub fn default() -> Self {
        let initial_generators: (Rc<RefCell<Generator>>, Rc<RefCell<Generator>>);
        let mut all_generators: HashMap<String, Rc<RefCell<Generator>>> = HashMap::new();
        let punch_tree = Rc::new(RefCell::new(
            Generator::blank(
                ResourceType::WOOD,
                60,
                1.05,
                1,
                1,
                "Punching Tree".to_owned(),
            )
            .add_cost((ResourceType::WOOD, 25.0)),
        ));
        let hit_rock = Rc::new(RefCell::new(
            Generator::blank(
                ResourceType::STONE,
                120,
                1.07,
                0,
                1,
                "Hitting rocks with sticks".to_owned(),
            )
            .add_cost((ResourceType::WOOD, 50.0)),
        ));
        all_generators.insert(
            punch_tree.borrow().generator_name.clone(),
            punch_tree.clone(),
        );
        all_generators.insert(hit_rock.borrow().generator_name.clone(), hit_rock.clone());
        let not_yet_used = VecDeque::new();
        initial_generators = (punch_tree.clone(), hit_rock.clone());
        let mut future_self = Self {
            not_yet_used,
            initial_generators,
            all_generators,
        };
        future_self.add_gen(
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
        future_self.add_gen(
            Generator::blank(ResourceType::WOOD, 30, 1.04, 0, 2, "Actual Axes".to_owned())
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
            )
            .add_cost((ResourceType::COPPER, 30.0))
            .add_cost((ResourceType::IRON, 20.0)),
        );
        future_self
    }
    pub fn get_initials(&self) -> (Rc<RefCell<Generator<'a>>>, Rc<RefCell<Generator<'a>>>) {
        return self.initial_generators.clone();
    }
    pub fn get_next(&mut self) -> Option<Rc<RefCell<Generator<'a>>>> {
        self.not_yet_used.pop_back()
    }
    fn add_gen(&mut self, gener: Generator<'a>) {
        let gen_ref = Rc::new(RefCell::new(gener));
        self.all_generators
            .insert(gen_ref.borrow().generator_name.clone(), gen_ref.clone());
        self.not_yet_used.push_front(gen_ref.clone());
    }
}
