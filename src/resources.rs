mod new_resource_manager;
pub mod resource_amount;
pub mod resource_array;
pub mod resource_change;
pub use crate::resources::new_resource_manager::ResManager;
use ratatui::style::Color::{self};

pub const RESOURCE_COUNT: usize = 7;
#[derive(Clone, Copy, Hash)]
pub struct ResourceType {
    pub name: &'static str,
    pub color: Color,
    pub id: usize,
}
macro_rules! generate_resource {
    ($($name:ident, $color:expr );* $(;)?) => {
        pub const TOTAL_ITEMS: usize = generate_resource!(@count $($name)*);
        generate_resource!(@expand 0; $($name, $color);*);
        paste::paste! {
            pub const RESOURCES: [ResourceType; TOTAL_ITEMS] = [
                $([<$name:upper>]),*
            ];
        }
    };
    (@count) => { 0 };
    (@count $head:ident $($tail:ident)*) => {1 + generate_resource!(@count $($tail)*)};

    (@expand $idx:expr; $name:ident, $color:expr; $($tail_name:ident, $tail_color:expr);*) => {
        paste::paste! {
        pub const [<$name:upper>]: ResourceType = ResourceType {
            name: stringify!($name),
            color: $color,
            id: $idx,
        };
        }
        generate_resource!(@expand $idx + 1; $($tail_name, $tail_color );*);
    };
    (@expand $idx:expr; $name:ident, $color:expr) => {
        paste::paste! {
            const [<$name:upper>]: ResourceType = ResourceType {
                name: stringify!($name),
                color: $color,
                id: $idx,
            };

        }
    };
}

generate_resource! {
    Wood, Color::White;
    Stone, Color::White;
    Iron, Color::White;
    Copper, Color::White;
    Gold, Color::White;
    Ruby, Color::White;
    Diamond,Color::White;
}

impl PartialEq for ResourceType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for ResourceType {}
