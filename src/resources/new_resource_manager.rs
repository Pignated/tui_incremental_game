use std::{array, iter::zip};

use ratatui::text::{Line, Span};

use crate::resources::{
    RESOURCE_COUNT, ResourceType, resource_array::ResValArray, resource_change::ResourceChange,
};
pub struct ResManager<'a> {
    resource_counts: ResValArray,
    resource_total_earned: ResValArray,
    pub resource_lines: [Line<'a>; RESOURCE_COUNT],
    has_changed: usize,
}
impl<'a> ResManager<'a> {
    pub fn new() -> Self {
        let resource_counts = ResValArray::new();
        let resource_total_earned = ResValArray::new();
        let resource_lines = array::from_fn(|_| Line::from(""));
        ResManager {
            resource_counts,
            resource_total_earned,
            resource_lines,
            has_changed: 0,
        }
    }
    pub fn get_count(&self, res_type: ResourceType) -> usize {
        self.resource_counts[res_type as usize]
    }
    pub fn get_total_count(&self, res_type: ResourceType) -> usize {
        self.resource_total_earned[res_type as usize]
    }
    pub fn get_all_total_counts(&self) -> ResValArray {
        self.resource_total_earned
    }
    pub fn change(&mut self, change: ResourceChange) {
        match change {
            ResourceChange::Increase { val } => {
                for i in 0..RESOURCE_COUNT {
                    self.resource_counts[i] = self.resource_counts[i].saturating_add(val[i]);
                    self.resource_total_earned[i] =
                        self.resource_total_earned[i].saturating_add(val[i]);
                    self.has_changed |= ((val[i] != 0) as usize) << i;
                }
            }
            ResourceChange::Decrease { val } => {
                for i in 0..RESOURCE_COUNT {
                    self.resource_counts[i] = self.resource_counts[i].saturating_sub(val[i]);
                    self.has_changed |= ((val[i] != 0) as usize) << i;
                }
            }
            ResourceChange::None => {}
        }
    }
    pub fn tick(&mut self) {
        let mut mask = self.has_changed;
        while mask != 0 {
            let idx = mask.trailing_zeros() as usize;

            self.resource_lines[idx] = Line::from(Span::styled(
                format!(
                    "Current {0}: {1}",
                    ResourceType::NAMES[idx],
                    self.resource_counts[idx]
                ),
                ResourceType::COLORS[idx],
            ));
            mask &= !(1 << idx);
        }
    }
    pub fn can_afford(&self, cost: ResValArray) -> bool {
        zip(self.resource_counts, cost).all(|(a, b)| a >= b)
    }
}
