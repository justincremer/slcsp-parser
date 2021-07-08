use crate::parse::List;
use crate::parse::{Plan, PlanList, Slcsp, Zipcode, ZipcodeList};
use quickersort::sort_floats;

impl Slcsp {
    pub fn find_zipcode_in(&self, list: &ZipcodeList) -> Option<Zipcode> {
        for i in &list.items {
            if i.zipcode == self.zipcode {
                return Some((*i).clone());
            }
        }

        return None;
    }
}

impl Zipcode {
    pub fn find_plans_in(&self, list: &PlanList) -> PlanList {
        let mut items: Vec<Plan> = Vec::default();
        let mut count: usize = 0;
        for i in &list.items {
            if i.state == self.state && i.rate_area == self.rate_area && i.metal_level == "Silver" {
                items.push(i.clone());
                count += 1;
            }
        }

        List::<Plan>::new(items, count)
    }
}

impl PlanList {
    pub fn second_smallest_rate(&self) -> Option<f32> {
        match self.count {
            0 | 1 => None,
            _ => {
                let mut rates = self.items.iter().map(|i| i.rate).collect::<Vec<f32>>();
                sort_floats(&mut rates);
                Some(rates[1])
            }
        }
    }
}
