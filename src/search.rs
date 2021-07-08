use crate::parse::List;
use crate::parse::{Plan, PlanList, Slcsp, SlcspList, Zipcode, ZipcodeList};
use std::fmt;

// pub struct List<T> {
//     pub items: Vec<T>,
//     pub count: u64,
// }

// pub struct Slcsp {
//     pub zipcode: u32,
//     pub rate: Option<f32>,
// }

// pub struct Plan {
//     pub plan_id: String,
//     pub state: String,
//     pub metal_level: String,
//     pub rate: f32,
//     pub rate_area: u8,
// }

// pub struct Zipcode {
//     pub zipcode: u32,
//     pub state: String,
//     pub county_code: String,
//     pub name: String,
//     pub rate_area: u8,
// }

// pub fn format_second_lowest(
//     s_list: &SlcspList,
//     p_list: &PlanList,
//     z_list: ZipcodeList,
// ) -> fmt::Result {
//     for s in s_list.items {}
// }

impl Slcsp {
    pub fn find_zipcode_in<'a>(&self, list: &'a ZipcodeList) -> Option<&'a Zipcode> {
        for i in list.items {
            if i.zipcode == self.zipcode {
                return Some(i);
            }
        }

        None
    }
}

impl Zipcode {
    pub fn find_plans_in<'a>(&self, list: &'a PlanList) -> PlanList {
        let mut items: Vec<Plan> = Vec::default();
        let mut count: usize = 0;
        for i in list.items {
            if i.state == self.state && i.rate_area == self.rate_area {
                items.push(i.clone());
                count += 1;
            }
        }

        List::<Plan>::new(&items, count)
    }
}
