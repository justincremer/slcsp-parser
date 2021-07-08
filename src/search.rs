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

#[cfg(test)]
mod test {
    use crate::parse::{Load, PlanList, SlcspList, ZipcodeList};

    #[test]
    fn find_zipcode_in_passes() {
        let files = ("test_data/slcsp.csv", "test_data/zips.csv");
        let test_data = vec![
            64148, 67118, 40813, 18229, 51012, 79168, 54923, 67651, 49448, 27702,
        ];

        let slcsps = SlcspList::load(files.0);
        let zipcodes = ZipcodeList::load(files.1);

        assert_eq!(
            test_data,
            slcsps.items[0..10]
                .iter()
                .map(|i| i.find_zipcode_in(&zipcodes).unwrap().zipcode)
                .collect::<Vec<u32>>()
        );
    }

    #[test]
    fn second_smallest_rate_passes() {
        let files = (
            "test_data/slcsp.csv",
            "test_data/zips.csv",
            "test_data/plans.csv",
        );
        let test_data = vec![
            Some(245.2),
            Some(212.35),
            None,
            Some(231.48),
            Some(252.76),
            Some(243.68),
            Some(255.26),
            Some(249.44),
            Some(221.63),
            Some(283.08),
        ];

        let mut slcsps = SlcspList::load(files.0);
        let zipcodes = Box::from(ZipcodeList::load(files.1));
        let plans = Box::from(PlanList::load(files.2));

        for i in &mut slcsps.items {
            i.rate = match i.find_zipcode_in(zipcodes.as_ref()) {
                Some(z) => z.find_plans_in(plans.as_ref()).second_smallest_rate(),
                None => None,
            };
        }

        assert_eq!(
            test_data,
            slcsps.items[0..10]
                .iter()
                .map(|i| i.rate)
                .collect::<Vec<Option<f32>>>()
        );
    }
}
