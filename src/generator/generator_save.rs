use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{
    generator::{Generator, GeneratorID, GeneratorRefCellWrapper},
    resources::{RESOURCE_COUNT, RESOURCES, resource_array::ResValArray},
};
#[derive(Serialize, Deserialize)]
pub struct GeneratorSave {
    resource_type: usize,
    ticks_per: usize,
    purchase_costs: [usize; RESOURCE_COUNT],
    cost_coeff: f64,
    current_bought: usize,
    initial_bought: usize,
    amount_per_harvest: usize,
    generator_name: String,
    id: GeneratorIDSave,
    in_app: bool,
    pub(crate) idx: u64,
}

impl GeneratorSave {
    pub fn new(
        resource_type: usize,
        ticks_per: usize,
        purchase_costs: ResValArray,
        cost_coeff: f64,
        current_bought: usize,
        initial_bought: usize,
        amount_per_harvest: usize,
        generator_name: String,
        id: GeneratorID,
        in_app: bool,
        idx: u64,
    ) -> Self {
        GeneratorSave {
            resource_type,
            ticks_per,
            purchase_costs: purchase_costs.0,
            cost_coeff,
            current_bought,
            initial_bought,
            amount_per_harvest,
            generator_name,
            id: id.to_save(),
            in_app,
            idx,
        }
    }
    pub fn to_gen(&self) -> Generator {
        Generator::from_save(
            self.resource_type,
            self.ticks_per,
            self.purchase_costs,
            self.cost_coeff,
            self.current_bought,
            self.initial_bought,
            self.amount_per_harvest,
            self.generator_name.clone(),
            self.id,
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GeneratorIDSave {
    res_type: usize,
    idx: usize,
}
impl GeneratorIDSave {
    pub fn new(res_type: usize, idx: usize) -> Self {
        Self { res_type, idx }
    }
    pub fn to_id(&self) -> GeneratorID {
        GeneratorID {
            res_type: RESOURCES[self.res_type],
            idx: self.idx,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct GeneratorListSave {
    pub generators: Vec<GeneratorSave>,
}
impl GeneratorListSave {
    pub fn new(list: VecDeque<GeneratorRefCellWrapper>) -> Self {
        let mut new = GeneratorListSave {
            generators: Vec::new(),
        };
        for gener in list {
            new.generators
                .push(gener.borrow().to_save(false, gener.idx));
        }
        new
    }
}
