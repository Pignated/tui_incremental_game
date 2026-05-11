use ratatui::style::Color::Rgb;
use ratatui::{
    style::Color,
    text::{Line, Span},
};
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ResourceType {
    WOOD,
    STONE,
    IRON,
    COPPER,
    GOLD,
    RUBY,
    DIAMOND,
}
impl ResourceType {
    pub fn get_color(&self) -> Color {
        match self {
            ResourceType::WOOD => Rgb(130, 76, 9),
            ResourceType::STONE => Rgb(115, 120, 119),
            ResourceType::IRON => Rgb(167, 171, 171),
            ResourceType::COPPER => Rgb(240, 169, 17),
            ResourceType::GOLD => Rgb(245, 242, 51),
            ResourceType::RUBY => Rgb(153, 2, 2),
            ResourceType::DIAMOND => Rgb(240, 169, 17),
        }
    }
    pub fn get_name(&self) -> String {
        match self {
            ResourceType::WOOD => "Wood".to_owned(),
            ResourceType::STONE => "Stone".to_owned(),
            ResourceType::IRON => "Iron".to_owned(),
            ResourceType::COPPER => "Copper".to_owned(),
            ResourceType::GOLD => "Gold".to_owned(),
            ResourceType::RUBY => "Ruby".to_owned(),
            ResourceType::DIAMOND => "Diamond".to_owned(),
        }
    }
}
#[derive(Debug)]
pub struct Resource {
    name: String,
    pub(crate) count: usize,
    color: Color,
    display_cache: String,
    updated: bool,
    pub resource_type: ResourceType,
}
impl Resource {
    pub fn new(name: String, color: Color, resource_type: ResourceType) -> Self {
        Self {
            name,
            count: 0,
            color,
            display_cache: String::new(),
            updated: false,
            resource_type,
        }
    }
    pub fn increase(&mut self, to_add: usize) {
        self.count = self.count.saturating_add(to_add as usize);
        self.updated = true
    }
    pub fn decrease(&mut self, to_remove: usize) {
        self.count = self.count.saturating_sub(to_remove as usize);
        self.updated = true
    }
    pub fn tick(&mut self) {
        if self.updated {
            self.display_cache = format!("Current {0}: {1}", self.name, self.count);
            self.updated = false;
        }
    }
    pub fn get_str(&self) -> Line<'_> {
        Line::from(Span::styled(&self.display_cache, self.color))
    }
    pub fn new_from_type(resource_type: ResourceType) -> Self {
        return Resource::new(
            resource_type.get_name(),
            resource_type.get_color(),
            resource_type,
        );
    }
    pub fn change(&mut self, change: &ResourceChange) {
        match change {
            ResourceChange::Increase { amts, .. } => {
                for x in amts {
                    if x.0 == self.resource_type {
                        self.increase(x.1);
                    }
                }
            }
            ResourceChange::Decrease { amts, .. } => {
                for x in amts {
                    if x.0 == self.resource_type {
                        self.decrease(x.1);
                    }
                }
            }
            ResourceChange::SingleIncrease { amt, resource_type } => {
                if *resource_type == self.resource_type {
                    self.increase(*amt);
                }
            }
            ResourceChange::None => {}
        }
    }
}
#[derive(Clone, Debug)]
pub enum ResourceChange {
    Increase {
        amts: Vec<(ResourceType, usize)>,
        resource_count: usize,
    },
    Decrease {
        amts: Vec<(ResourceType, usize)>,
        resource_count: usize,
    },
    SingleIncrease {
        amt: usize,
        resource_type: ResourceType,
    },
    None,
}
