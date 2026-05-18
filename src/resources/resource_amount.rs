use crate::resources::{RESOURCE_COUNT, ResourceType};
#[derive(Clone, Copy)]
pub struct ResourceAmount {
    cost: [usize; RESOURCE_COUNT],
}
impl ResourceAmount {
    pub fn new() -> Self {
        ResourceAmount {
            cost: [0; RESOURCE_COUNT],
        }
    }
    pub fn add_cost(mut self, amt: usize, res_type: ResourceType) -> Self {
        self.cost[res_type as usize] = amt;
        self
    }
    pub fn get_arr(self) -> [usize; RESOURCE_COUNT] {
        self.cost
    }
    pub fn get_val(self, idx: usize) -> usize {
        if idx > RESOURCE_COUNT {
            return 0;
        }
        return self.cost[idx];
    }
    pub fn gte_all(self, other: Self) -> bool {
        let mut greater = true;
        for i in 0..RESOURCE_COUNT {
            if self.cost[i] < other.cost[i] {
                greater = false;
                break;
            }
        }
        greater
    }
}
