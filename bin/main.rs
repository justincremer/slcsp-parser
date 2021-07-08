use slcsp_parser::parse::{Load, PlanList, SlcspList, ZipcodeList};

fn main() {
    let files = ("prompt/slcsp.csv", "prompt/zips.csv", "prompt/plans.csv");

    let mut slcsps = SlcspList::load(files.0);
    let zipcodes = Box::from(ZipcodeList::load(files.1));
    let plans = Box::from(PlanList::load(files.2));

    for i in &mut slcsps.items {
        i.rate = match i.find_zipcode_in(zipcodes.as_ref()) {
            Some(z) => z.find_plans_in(plans.as_ref()).second_smallest_rate(),
            None => None,
        };
    }

    print!("zipcode,rate\n");
    slcsps.items.iter().for_each(|i| println!("{}", i));
}
