pub mod upgrade_manager;
pub mod upgrade_save;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::{
    generator::{GeneratorID, generator_list::GeneratorList},
    resources::{RESOURCE_COUNT, RESOURCES, ResourceType, resource_array::ResValArray},
    upgrades::upgrade_save::UpgradeSave,
};
#[derive(Clone)]
pub struct Upgrade {
    pub effected_generator: GeneratorID,
    pub speed_multiplier: Option<usize>,
    pub output_multiplier: Option<usize>,
    generator_name: String,
    pub description: String,
    name: String,
    pub cost: ResValArray,
    requirements: ResValArray,
    selected: bool,
    style: Style,
}
impl Upgrade {
    pub fn new(
        effected_generator: GeneratorID,
        speed_multiplier: Option<usize>,
        output_multiplier: Option<usize>,
        description: String,
        name: String,
        generator_list: &GeneratorList,
    ) -> Self {
        Upgrade {
            effected_generator,
            speed_multiplier,
            output_multiplier,
            description,
            name,
            cost: ResValArray::new(),
            requirements: ResValArray::new(),
            generator_name: generator_list
                .get_gener_name(effected_generator)
                .unwrap_or(String::from("Generator Not Found")),
            style: Style::new(),
            selected: false,
        }
    }
    pub fn new_speed(
        effected_generator: GeneratorID,
        speed_multiplier: usize,
        description: String,
        name: String,
        cost: ResValArray,
        requirements: ResValArray,
        generator_list: &GeneratorList,
    ) -> Self {
        Upgrade {
            effected_generator,
            speed_multiplier: Some(speed_multiplier),
            output_multiplier: None,
            description,
            name,
            cost,
            requirements,
            generator_name: generator_list
                .get_gener_name(effected_generator)
                .unwrap_or(String::from("Generator Not Found")),
            style: Style::new(),
            selected: false,
        }
    }
    pub fn new_output(
        effected_generator: GeneratorID,
        output_multiplier: usize,
        description: String,
        name: String,
        cost: ResValArray,
        requirements: ResValArray,
        generator_list: &GeneratorList,
    ) -> Self {
        Upgrade {
            effected_generator,
            speed_multiplier: None,
            output_multiplier: Some(output_multiplier),
            description,
            name,
            cost,
            requirements,
            generator_name: generator_list
                .get_gener_name(effected_generator)
                .unwrap_or(String::from("Generator Not Found")),
            style: Style::new(),
            selected: false,
        }
    }
    pub fn from_save(
        effected_generator: GeneratorID,
        speed_modifier: Option<usize>,
        output_modifier: Option<usize>,
        generator_name: String,
        description: String,
        name: String,
        cost: ResValArray,
        requirements: ResValArray,
        style: Style,
    ) -> Self {
        Self {
            effected_generator: effected_generator,
            speed_multiplier: speed_modifier,
            output_multiplier: output_modifier,
            generator_name,
            description,
            name,
            cost,
            requirements,
            selected: false,
            style,
        }
    }
    pub fn add_cost(mut self, amt: usize, res: ResourceType) -> Self {
        self.cost = self.cost.add_cost(amt, res);
        self
    }
    pub fn add_requirement(mut self, amt: usize, res: ResourceType) -> Self {
        self.requirements = self.requirements.add_cost(amt, res);
        self
    }
    pub fn select(&mut self) {
        if !self.selected {
            self.style = self.style.fg(Color::Cyan).bold();
            self.selected = true;
        }
    }
    pub fn deselect(&mut self) {
        if self.selected {
            self.style = self.style.fg(Color::LightRed).not_bold();
            self.selected = false;
        }
    }
    pub fn to_save(&self) -> UpgradeSave {
        UpgradeSave::new(
            self.effected_generator.to_save(),
            self.speed_multiplier,
            self.output_multiplier,
            self.generator_name.clone(),
            self.description.clone(),
            self.name.clone(),
            self.cost,
            self.requirements,
        )
    }
}
impl Widget for Upgrade {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vert = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Min(1),
        ]);
        let [title_area, gener_area, cost_area, description_area] = vert.areas(area);

        let name_line = Text::from(Line::from(self.name.clone()).alignment(Alignment::Center))
            .style(self.style);
        name_line.render(title_area, buf);
        let gener_line = Text::from(Line::from(self.generator_name.clone()).style(self.style));
        gener_line.render(gener_area, buf);
        let mut cost_line_vec = Vec::new();
        for i in 0..RESOURCE_COUNT {
            if self.cost.get_val(i) > 0 {
                let var = RESOURCES[i];
                cost_line_vec.push(Span::from(format!(
                    "{}:{} ",
                    var.name,
                    self.cost.get_val(i)
                )))
            }
        }
        let cost_line = Line::from(cost_line_vec).style(self.style);
        cost_line.render(cost_area, buf);
        let description_para = Paragraph::new(self.description.clone())
            .wrap(Wrap { trim: true })
            .style(self.style);
        description_para.render(description_area, buf);
    }
}
