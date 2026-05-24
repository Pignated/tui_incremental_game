use std::{
    cell::{Ref, RefCell, RefMut},
    cmp::min,
    rc::Rc,
};
pub mod generator_list;
pub mod generator_save;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::{
    generator::generator_save::{GeneratorIDSave, GeneratorSave},
    resources::{
        RESOURCE_COUNT, RESOURCES, ResourceType, resource_array::ResValArray,
        resource_change::ResourceChange,
    },
    upgrades::Upgrade,
};

pub struct Generator {
    pub resource_type: ResourceType,
    pub progress: usize,
    ticks_per: usize,
    purchase_costs: ResValArray, //Contains the resource type and the initial cost
    cost_coeff: f64,
    current_bought: usize,
    initial_bought: usize,
    amount_per_harvest: usize,
    generator_name: String,
    id: GeneratorID,
    style: Style,
    selected: bool,
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
    pub fn to_save(&self) -> GeneratorIDSave {
        GeneratorIDSave::new(self.res_type.id, self.idx)
    }
}

impl Generator {
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
            purchase_costs: ResValArray::new(),
            cost_coeff,
            current_bought: starting_amount,
            initial_bought: starting_amount,
            amount_per_harvest,
            generator_name,
            id: GeneratorID {
                res_type: resource_type,
                idx,
            },
            style: Style::default(),
            selected: false,
        }
    }
    pub fn select(&mut self) {
        if !self.selected {
            self.style = self.style.fg(Color::Cyan).bold();
            self.selected = true;
        }
    }
    pub fn deselect(&mut self) {
        if self.selected {
            self.style = self.style.fg(Color::Reset).not_bold();
            self.selected = false;
        }
    }
    pub fn tick(&mut self) -> ResourceChange {
        if self.current_bought > 0 {
            self.progress += 1;
            if self.progress >= self.ticks_per && self.current_bought > 0 {
                self.progress %= self.ticks_per;
                let mut res_change = ResValArray::new();
                res_change[self.resource_type.id] = self.current_bought * self.amount_per_harvest;
                return ResourceChange::Increase { val: res_change };
            }
        }
        ResourceChange::None
    }
    pub fn get_cost(&self) -> ResValArray {
        let multiplier = self
            .cost_coeff
            .powf((self.current_bought - self.initial_bought) as f64);
        self.purchase_costs.mult_by(multiplier)
    }
    pub fn get_count(&self) -> usize {
        self.current_bought
    }
    pub fn buy_next(&mut self) -> ResourceChange {
        let change = ResourceChange::Decrease {
            val: self.get_cost(),
        };
        self.current_bought += 1;
        change
    }
    pub fn add_cost(mut self, cost: (ResourceType, usize)) -> Self {
        self.purchase_costs[cost.0.id] += cost.1;
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
    pub fn draw_gauge(&self, ratio: f64, width: usize, area: Rect, buf: &mut Buffer) {
        let clamped = ratio.clamp(0.0, 1.0);
        let filled_count = (clamped * (width - 2) as f64).round() as usize;
        let empty_count = width - 2 - filled_count;
        let filled_str = "█".repeat(filled_count);
        //▒
        let empty_str = "░".repeat(empty_count);
        let gause_string = format!("[{}{}]", filled_str, empty_str);
        Paragraph::new(gause_string)
            .style(self.style)
            .render(area, buf);
    }
    pub fn to_save(&self, in_app: bool, idx: u64) -> GeneratorSave {
        GeneratorSave::new(
            self.resource_type.id,
            self.ticks_per,
            self.purchase_costs,
            self.cost_coeff,
            self.current_bought,
            self.initial_bought,
            self.amount_per_harvest,
            self.generator_name.clone(),
            self.id,
            in_app,
            idx,
        )
    }
    pub fn from_save(
        resource_type: usize,
        ticks_per: usize,
        purchase_costs: [usize; RESOURCE_COUNT],
        cost_coeff: f64,
        current_bought: usize,
        initial_bought: usize,
        amount_per_harvest: usize,
        generator_name: String,
        id: GeneratorIDSave,
    ) -> Self {
        Self {
            resource_type: RESOURCES[resource_type],
            progress: 0,
            ticks_per,
            purchase_costs: ResValArray(purchase_costs),
            cost_coeff,
            current_bought,
            initial_bought,
            amount_per_harvest,
            generator_name,
            id: id.to_id(),
            style: Style::new(),
            selected: false,
        }
    }
}
#[derive(Clone)]
pub struct GeneratorRefCellWrapper {
    pub gener: Rc<RefCell<Generator>>,
    pub idx: u64,
}
impl<'a> GeneratorRefCellWrapper {
    pub fn new(gener: Generator, idx: u64) -> Self {
        GeneratorRefCellWrapper {
            gener: Rc::new(RefCell::new(gener)),
            idx,
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            gener: self.gener.clone(),
            idx: self.idx,
        }
    }
    pub fn borrow(&self) -> Ref<'_, Generator> {
        self.gener.borrow()
    }
    pub fn borrow_mut(&mut self) -> RefMut<'_, Generator> {
        self.gener.borrow_mut()
    }
    pub fn upgrade(&mut self, upgrade: Upgrade) {
        self.borrow_mut().upgrade(upgrade);
    }
    pub fn select(&mut self) {
        self.borrow_mut().select();
    }
    pub fn deselect(&mut self) {
        self.borrow_mut().deselect();
    }
}
impl Widget for GeneratorRefCellWrapper {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let style = if self.borrow().current_bought == 0 {
            self.borrow().style.dim()
        } else {
            self.borrow().style
        };
        let vert = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]);
        let true_area = area;
        let [resource_name_area, progress_area] = vert.areas(true_area);
        let resource_name_span = Span::from(format!(
            "{} x {}",
            self.gener.borrow().generator_name.clone(),
            self.gener.borrow().current_bought
        ))
        .style(style);
        let next_cost_line: Line;
        let mut cost_line_span_vec = Vec::new();
        cost_line_span_vec.push(resource_name_span);
        cost_line_span_vec.push(Span::from("    "));
        for (i, x) in self.gener.borrow().get_cost().into_iter().enumerate() {
            if x > 0 {
                cost_line_span_vec
                    .push(Span::from(format!("{}:{} ", RESOURCES[i].name, x)).style(style))
            }
        }
        cost_line_span_vec.push(
            Span::from(format!(
                "  ->  {}:{}",
                self.borrow().resource_type.name,
                self.borrow().current_bought * self.borrow().amount_per_harvest
            ))
            .style(style),
        );
        next_cost_line = Line::from(cost_line_span_vec);

        next_cost_line.render(resource_name_area, buf);
        let mut progress_ratio = 0.0;
        if self.gener.borrow().current_bought > 0 {
            progress_ratio = min(self.gener.borrow().progress, self.gener.borrow().ticks_per)
                as f64
                / self.gener.borrow().ticks_per as f64;
        }
        self.borrow().draw_gauge(
            progress_ratio,
            progress_area.width as usize,
            progress_area,
            buf,
        );
    }
}
