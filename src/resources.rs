use ratatui::style::Color::Rgb;
use ratatui::{
    style::Color,
    text::{Line, Span},
};

pub const RESOURCE_COUNT: usize = 7;
#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum ResourceType {
    WOOD = 0,
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
    pub const VARIANTS: &'static [ResourceType] = &[
        Self::WOOD,
        Self::STONE,
        Self::IRON,
        Self::COPPER,
        Self::GOLD,
        Self::RUBY,
        Self::DIAMOND,
    ];
}
#[derive(Debug)]
pub struct Resource {
    name: String,
    pub(crate) count: usize,
    color: Color,
    updated: bool,
    pub resource_type: ResourceType,
    total_count: usize,
}
impl Resource {
    pub fn new(name: String, color: Color, resource_type: ResourceType) -> Self {
        Self {
            name,
            count: 0,
            color,
            updated: false,
            resource_type,
            total_count: 0,
        }
    }
    pub fn increase(&mut self, to_add: usize) {
        self.count = self.count.saturating_add(to_add);
        self.total_count = self.total_count.saturating_add(to_add);
        self.updated = true
    }
    pub fn decrease(&mut self, to_remove: usize) {
        self.count = self.count.saturating_sub(to_remove);
        self.updated = true
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
            ResourceChange::SingleIncrease { amt, .. } => {
                self.increase(*amt);
            }
            ResourceChange::SingleDecrease { amt, .. } => {
                self.decrease(*amt);
            }
            ResourceChange::None => {}
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub enum ResourceChange {
    SingleDecrease {
        amt: usize,
        resource_type: ResourceType,
    },
    SingleIncrease {
        amt: usize,
        resource_type: ResourceType,
    },
    None,
}

pub struct ResourceManager<'a> {
    resources: Vec<Resource>,
    pub resource_lines: Vec<Line<'a>>,
}
impl<'a> ResourceManager<'a> {
    pub fn new() -> Self {
        let mut vect = Vec::new();
        for res_type in ResourceType::VARIANTS {
            vect.push(Resource::new_from_type(*res_type));
        }
        let res_lines = vec![Line::from(""); RESOURCE_COUNT];
        ResourceManager {
            resources: vect,
            resource_lines: res_lines,
        }
    }
    pub fn get_count(&self, res_type: ResourceType) -> usize {
        self.resources[res_type as usize].count
    }
    pub fn get_total_count(&self, res_type: ResourceType) -> usize {
        self.resources[res_type as usize].total_count
    }
    pub fn change(&mut self, change: ResourceChange) {
        match change {
            ResourceChange::SingleDecrease { resource_type, .. }
            | ResourceChange::SingleIncrease { resource_type, .. } => {
                self.resources[resource_type as usize].change(&change);
            }
            ResourceChange::None => {}
        }
    }
    pub fn tick(&mut self) {
        for res in &mut self.resources {
            if res.updated {
                res.updated = false;
                self.resource_lines[res.resource_type as usize] = Line::from(Span::styled(
                    format!("Current {0}: {1}", res.name, res.count),
                    res.color,
                ));
            }
        }
    }
    pub fn get_mut_resource(&mut self, res_type: ResourceType) -> &mut Resource {
        self.resources.get_mut(res_type as usize).unwrap()
    }
    pub fn get_resources_arr(&self) -> [usize; RESOURCE_COUNT] {
        let mut arr = [0; RESOURCE_COUNT];
        for (i, v) in self.resources.iter().enumerate() {
            arr[i] = v.count;
        }
        arr
    }
}
