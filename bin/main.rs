use slcsp_parser::parse::{Load, PlanList, SlcspList, ZipcodeList};
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let files = (
        "test_data/slcsp.csv",
        "test_data/zips.csv",
        "test_data/plans.csv",
    );

    let mut slcsps = SlcspList::load(files.0);
    let zipcodes = Box::from(ZipcodeList::load(files.1));
    let plans = Box::from(PlanList::load(files.2));

    for i in &mut slcsps.items {
        i.rate = match i.find_zipcode_in(zipcodes.as_ref()) {
            Some(z) => z.find_plans_in(plans.as_ref()).second_smallest_rate(),
            None => None,
        };
    }
    let total_time = Instant::now() - start_time;

    println!("zipcode,rate");
    slcsps.items.iter().for_each(|i| println!("{}", i));
    println!(
        "\nSolved {} slcsps in {} nanoseconds.",
        slcsps.count,
        total_time.as_nanos()
    );
}
