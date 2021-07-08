use slcsp_parser::parse::{Load, PlanList, SlcspList, ZipcodeList};

fn main() {
    let files = ("prompt/slcsp.csv", "prompt/plans.csv", "prompt/zips.csv");

    let slcsps = SlcspList::load(files.0);
    let plans = PlanList::load(files.1);
    let zipcodes = ZipcodeList::load(files.2);

    println!(
        "{}, count: {}\n{}, count: {}\n{}, count: {}\n",
        files.0, slcsps.count, files.1, plans.count, files.2, zipcodes.count
    );
}
