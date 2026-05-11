use std::cmp::min;
pub mod generator_list;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, LineGauge, Widget},
};

use crate::resources::{ResourceChange, ResourceType};

#[derive(Clone)]
pub struct Generator<'a> {
    resource_type: ResourceType,
    progress: usize,
    ticks_per: usize,
    purchase_resource_count: usize,
    purchase_costs: Vec<(ResourceType, f64)>, //Contains the resource type and the initial cost
    cost_coeff: f64,
    current_bought: usize,
    initial_bought: usize,
    amount_per_harvest: usize,
    generator_name: String,
    block: Option<Block<'a>>,
}
pub struct UpgradeCost {
    pub costs: Vec<(ResourceType, usize)>,
}

impl<'a> Generator<'a> {
    pub fn blank(
        resource_type: ResourceType,
        ticks_per: usize,
        cost_coeff: f64,
        starting_amount: usize,
        amount_per_harvest: usize,
        generator_name: String,
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
        }
    }
    pub fn get_count(&self) -> usize {
        self.current_bought
    }
    pub fn block(mut self, block_val: Block<'a>) -> Self {
        self.block = Some(block_val);
        self
    }
    pub fn clear_block(mut self) -> Self {
        self.block = None;
        self
    }
    pub fn tick(&mut self) -> ResourceChange {
        self.progress += 1;
        if self.progress >= self.ticks_per && self.current_bought > 0 {
            self.progress %= self.ticks_per;
            return ResourceChange::SingleIncrease {
                amt: self.amount_per_harvest * self.current_bought,
                resource_type: self.resource_type,
            };
        } else {
            return ResourceChange::None;
        }
    }
    pub fn get_cost(&self) -> UpgradeCost {
        let cost_vec = self
            .purchase_costs
            .iter()
            .map(|x| {
                (
                    x.0,
                    (x.1 * self
                        .cost_coeff
                        .powf((self.current_bought - self.initial_bought) as f64))
                        as usize,
                )
            })
            .collect();

        return UpgradeCost { costs: cost_vec };
    }
    pub fn buy_next(&mut self) -> ResourceChange {
        let cost_vec = self
            .purchase_costs
            .iter()
            .map(|x| {
                (
                    x.0,
                    (x.1 * self
                        .cost_coeff
                        .powf((self.current_bought - self.initial_bought) as f64))
                        as usize,
                )
            })
            .collect();

        self.current_bought += 1;
        let change = ResourceChange::Decrease {
            amts: cost_vec,
            resource_count: self.purchase_resource_count,
        };
        change
    }
    pub fn add_cost(mut self, cost: (ResourceType, f64)) -> Self {
        self.purchase_costs.push(cost);
        self.purchase_resource_count += 1;
        self
    }
}
impl<'a> Widget for Generator<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let vert = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]);
        let mut true_area = area;
        if let Some(b) = self.block {
            true_area = b.inner(area);
            b.render(area, buf);
        }
        let [resource_name_area, progress_area] = vert.areas(true_area);
        let resource_name_span = Span::from(format!(
            "{} x {}",
            self.generator_name.clone(),
            self.current_bought
        ))
        .style(Style::new().fg(self.resource_type.get_color()));
        let next_cost_line: Line;
        let mut cost_line_span_vec = Vec::new();
        cost_line_span_vec.push(resource_name_span);
        cost_line_span_vec.push(Span::from("    "));
        for x in self.purchase_costs {
            cost_line_span_vec.push(
                Span::from(format!(
                    "{}:{} ",
                    x.0.get_name(),
                    (x.1 * (self.cost_coeff)
                        .powf((self.current_bought - self.initial_bought) as f64))
                        as u64
                ))
                .style(Style::new().fg(x.0.get_color())),
            );
        }
        next_cost_line = Line::from(cost_line_span_vec);
        next_cost_line.render(resource_name_area, buf);
        let mut progress_ratio = 0.0;
        if self.current_bought > 0 {
            progress_ratio = min(self.progress, self.ticks_per) as f64 / self.ticks_per as f64;
        }
        let progress = LineGauge::default()
            .ratio(progress_ratio)
            .label(Span::styled(
                "",
                Style::default().fg(self.resource_type.get_color()),
            ))
            .filled_style(Style::default().fg(self.resource_type.get_color()))
            .filled_symbol("█")
            .unfilled_style(Style::default().fg(Color::White));

        progress.render(progress_area, buf);
    }
}

impl<'a> IntoIterator for &'a UpgradeCost {
    type Item = &'a (ResourceType, usize);

    type IntoIter = std::slice::Iter<'a, (ResourceType, usize)>;

    fn into_iter(self) -> Self::IntoIter {
        self.costs.iter()
    }
}
