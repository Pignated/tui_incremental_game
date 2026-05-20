pub mod upgrade_manager;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    generator::{GeneratorID, generator_list::GeneratorList},
    resources::{RESOURCE_COUNT, ResourceType, resource_array::ResValArray},
};
#[derive(Clone)]
pub struct Upgrade<'a> {
    pub effected_generator: GeneratorID,
    pub speed_multiplier: Option<usize>,
    pub output_multiplier: Option<usize>,
    generator_name: String,
    gener_color: Color,
    color: Color,
    pub description: String,
    name: String,
    pub cost: ResValArray,
    requirements: ResValArray,
    block: Option<Block<'a>>,
}
impl<'a> Upgrade<'a> {
    pub fn new(
        effected_generator: GeneratorID,
        speed_multiplier: Option<usize>,
        output_multiplier: Option<usize>,
        color: Color,
        description: String,
        name: String,
        generator_list: &GeneratorList,
    ) -> Self {
        Upgrade {
            effected_generator,
            speed_multiplier,
            output_multiplier,
            color,
            description,
            name,
            cost: ResValArray::new(),
            requirements: ResValArray::new(),
            block: None,
            generator_name: generator_list
                .get_gener_name(effected_generator)
                .unwrap_or(String::from("Generator Not Found")),
            gener_color: generator_list.get_gener_color(effected_generator),
        }
    }
    pub fn new_speed(
        effected_generator: GeneratorID,
        speed_multiplier: usize,
        color: Color,
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
            color,
            description,
            name,
            cost,
            requirements,
            block: None,
            generator_name: generator_list
                .get_gener_name(effected_generator)
                .unwrap_or(String::from("Generator Not Found")),
            gener_color: generator_list.get_gener_color(effected_generator),
        }
    }
    pub fn new_output(
        effected_generator: GeneratorID,
        output_multiplier: usize,
        color: Color,
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
            color,
            description,
            name,
            cost,
            requirements,
            block: None,
            generator_name: generator_list
                .get_gener_name(effected_generator)
                .unwrap_or(String::from("Generator Not Found")),
            gener_color: generator_list.get_gener_color(effected_generator),
        }
    }
    pub fn block(&mut self, block_val: Block<'a>) {
        self.block = Some(block_val);
    }
    pub fn clear_block(&mut self) {
        self.block = None;
    }
    pub fn add_cost(mut self, amt: usize, res: ResourceType) -> Self {
        self.cost = self.cost.add_cost(amt, res);
        self
    }
    pub fn add_requirement(mut self, amt: usize, res: ResourceType) -> Self {
        self.requirements = self.requirements.add_cost(amt, res);
        self
    }
}
impl Widget for Upgrade<'_> {
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
        let upgr_block;
        if let Some(block) = self.block {
            upgr_block = block;
        } else {
            upgr_block = Block::new();
        }
        let [title_area, gener_area, cost_area, description_area] =
            vert.areas(upgr_block.inner(area));

        let name_line = Text::from(
            Line::from(self.name.clone())
                .alignment(Alignment::Center)
                .style(Style::new().fg(self.color)),
        );
        name_line.render(title_area, buf);
        let gener_line = Text::from(
            Line::from(self.generator_name.clone()).style(Style::new().fg(self.gener_color)),
        );
        gener_line.render(gener_area, buf);
        let mut cost_line_vec = Vec::new();
        for i in 0..RESOURCE_COUNT {
            if self.cost.get_val(i) > 0 {
                let var = ResourceType::VARIANTS[i];
                cost_line_vec.push(
                    Span::from(format!(
                        "{}:{} ",
                        ResourceType::NAMES[var as usize],
                        self.cost.get_val(i)
                    ))
                    .style(Style::new().fg(ResourceType::COLORS[var as usize])),
                )
            }
        }
        upgr_block.render(area, buf);
        let cost_line = Line::from(cost_line_vec);
        cost_line.render(cost_area, buf);
        let description_para = Paragraph::new(self.description.clone())
            .wrap(Wrap { trim: true })
            .style(Style::new().fg(Color::White));
        description_para.render(description_area, buf);
    }
}
