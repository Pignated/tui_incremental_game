mod new_resource_manager;
pub mod resource_amount;
pub mod resource_array;
pub mod resource_change;
pub use crate::resources::new_resource_manager::ResManager;
use ratatui::style::Color::{self, Rgb};

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
    pub const NAMES: [&'static str; RESOURCE_COUNT] =
        ["Wood", "Stone", "Iron", "Copper", "Gold", "Ruby", "Diamond"];
    pub const VARIANTS: &'static [ResourceType] = &[
        Self::WOOD,
        Self::STONE,
        Self::IRON,
        Self::COPPER,
        Self::GOLD,
        Self::RUBY,
        Self::DIAMOND,
    ];
    pub const COLORS: &'static [Color; RESOURCE_COUNT] = &[
        Rgb(130, 76, 9),
        Rgb(115, 120, 119),
        Rgb(167, 171, 171),
        Rgb(240, 169, 17),
        Rgb(245, 242, 51),
        Rgb(153, 2, 2),
        Rgb(240, 169, 17),
    ];
}
