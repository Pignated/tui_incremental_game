use crate::resources::resource_array::ResValArray;

pub enum ResourceChange {
    Increase { val: ResValArray },
    Decrease { val: ResValArray },
    None,
}
