use slcsp_parser::parse::{Load, PlanList, SlcspList, ZipcodeList};
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
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
    let total_time = Instant::now() - start_time;

    println!("zipcode,rate");
    slcsps.items.iter().for_each(|i| println!("{}", i));
    println!(
        "\nSolved {} slcsps in {} nanoseconds.",
        slcsps.count,
        total_time.as_nanos()
    );
}

#[test]
fn test_correct_answers() {
    let files = ("prompt/slcsp.csv", "prompt/zips.csv", "prompt/plans.csv");
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
