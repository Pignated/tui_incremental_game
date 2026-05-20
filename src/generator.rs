use std::{
    cell::{Ref, RefCell, RefMut},
    cmp::min,
    rc::Rc,
};
pub mod generator_list;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, LineGauge, Widget},
};

use crate::{
    resources::{
        RESOURCE_COUNT, ResourceType, resource_array::ResValArray, resource_change::ResourceChange,
    },
    upgrades::Upgrade,
};

pub struct Generator<'a> {
    pub resource_type: ResourceType,
    pub progress: usize,
    ticks_per: usize,
    purchase_resource_count: usize,
    purchase_costs: Vec<(ResourceType, f64)>, //Contains the resource type and the initial cost
    cost_coeff: f64,
    current_bought: usize,
    initial_bought: usize,
    amount_per_harvest: usize,
    generator_name: String,
    block: Option<Block<'a>>,
    id: GeneratorID,
}
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct GeneratorID {
    res_type: ResourceType,
    idx: usize,
}
impl GeneratorID {
    pub fn new(res_type: ResourceType, idx: usize) -> Self {
        GeneratorID { res_type, idx }
    }
}
#[derive(Clone)]
pub struct GeneratorRefCellWrapper<'a> {
    pub gener: Rc<RefCell<Generator<'a>>>,
}
impl<'a> GeneratorRefCellWrapper<'a> {
    pub fn new(gener: Generator<'a>) -> Self {
        GeneratorRefCellWrapper {
            gener: Rc::new(RefCell::new(gener)),
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            gener: self.gener.clone(),
        }
    }
    pub fn borrow(&self) -> Ref<'_, Generator<'a>> {
        self.gener.borrow()
    }
    pub fn borrow_mut(&mut self) -> RefMut<'_, Generator<'a>> {
        self.gener.borrow_mut()
    }
    pub fn upgrade(&mut self, upgrade: Upgrade) {
        self.borrow_mut().upgrade(upgrade);
    }
}

impl<'a> Generator<'a> {
    pub fn blank(
        resource_type: ResourceType,
        ticks_per: usize,
        cost_coeff: f64,
        starting_amount: usize,
        amount_per_harvest: usize,
        generator_name: String,
        idx: usize,
    ) -> Self {
        Self {
            resource_type,
            progress: 0,
            ticks_per,
            purchase_resource_count: 0,
            purchase_costs: Vec::new(),
            cost_coeff,
            current_bought: starting_amount,
            initial_bought: starting_amount,
            amount_per_harvest,
            generator_name,
            block: None,
            id: GeneratorID {
                res_type: resource_type,
                idx,
            },
        }
    }
    pub fn get_count(&self) -> usize {
        self.current_bought
    }
    pub fn block(&mut self, block_val: Block<'a>) {
        self.block = Some(block_val);
    }
    pub fn clear_block(&mut self) {
        self.block = None;
    }
    pub fn tick(&mut self) -> ResourceChange {
        self.progress += 1;
        if self.progress >= self.ticks_per && self.current_bought > 0 {
            self.progress %= self.ticks_per;
            let mut res_change = ResValArray::new();
            res_change[self.resource_type as usize] = self.current_bought * self.amount_per_harvest;
            return ResourceChange::Increase { val: res_change };
        } else {
            return ResourceChange::None;
        }
    }
    pub fn get_cost(&self) -> ResValArray {
        let mut costs = [0usize; RESOURCE_COUNT];
        for rec in &self.purchase_costs {
            costs[rec.0 as usize] = (rec.1
                * self
                    .cost_coeff
                    .powf((self.current_bought - self.initial_bought) as f64))
                as usize;
        }
        return ResValArray(costs);
    }
    pub fn buy_next(&mut self) -> ResourceChange {
        let mut changes = ResValArray::new();
        for rec in &self.purchase_costs {
            changes[rec.0 as usize] = (rec.1
                * self
                    .cost_coeff
                    .powf((self.current_bought - self.initial_bought) as f64))
                as usize
        }

        self.current_bought += 1;
        ResourceChange::Decrease { val: changes }
    }
    pub fn add_cost(mut self, cost: (ResourceType, f64)) -> Self {
        self.purchase_costs.push(cost);
        self.purchase_resource_count += 1;
        self
    }
    pub fn upgrade(&mut self, upgrade: Upgrade) {
        let Upgrade {
            speed_multiplier,
            output_multiplier,
            ..
        } = upgrade;
        if let Some(speed_mult) = speed_multiplier {
            self.ticks_per /= speed_mult;
        }
        if let Some(out_mult) = output_multiplier {
            self.amount_per_harvest *= out_mult;
        }
    }
}
impl<'a> Widget for GeneratorRefCellWrapper<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let vert = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]);
        let mut true_area = area;
        if let Some(b) = self.gener.borrow().block.clone() {
            true_area = b.inner(area);
            b.render(area, buf);
        }
        let [resource_name_area, progress_area] = vert.areas(true_area);
        let resource_name_span = Span::from(format!(
            "{} x {}",
            self.gener.borrow().generator_name.clone(),
            self.gener.borrow().current_bought
        ))
        .style(Style::new().fg(ResourceType::COLORS[self.gener.borrow().resource_type as usize]));
        let next_cost_line: Line;
        let mut cost_line_span_vec = Vec::new();
        cost_line_span_vec.push(resource_name_span);
        cost_line_span_vec.push(Span::from("    "));
        for x in self.gener.borrow().purchase_costs.clone() {
            cost_line_span_vec.push(
                Span::from(format!(
                    "{}:{} ",
                    ResourceType::NAMES[x.0 as usize],
                    (x.1 * (self.gener.borrow().cost_coeff).powf(
                        (self.gener.borrow().current_bought - self.gener.borrow().initial_bought)
                            as f64
                    )) as u64
                ))
                .style(Style::new().fg(ResourceType::COLORS[x.0 as usize])),
            );
        }
        next_cost_line = Line::from(cost_line_span_vec);
        next_cost_line.render(resource_name_area, buf);
        let mut progress_ratio = 0.0;
        if self.gener.borrow().current_bought > 0 {
            progress_ratio = min(self.gener.borrow().progress, self.gener.borrow().ticks_per)
                as f64
                / self.gener.borrow().ticks_per as f64;
        }
        let progress = LineGauge::default()
            .ratio(progress_ratio)
            .label(Span::styled(
                "",
                Style::default()
                    .fg(ResourceType::COLORS[self.gener.borrow().resource_type as usize]),
            ))
            .filled_style(
                Style::default()
                    .fg(ResourceType::COLORS[self.gener.borrow().resource_type as usize]),
            )
            .filled_symbol("█")
            .unfilled_style(Style::default().fg(Color::White));

        progress.render(progress_area, buf);
    }
}
