use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

pub struct List<T> {
    pub items: Vec<T>,
    pub count: usize,
}

impl<T> List<T> {
    pub fn new(items: Vec<T>, count: usize) -> List<T> {
        List {
            items: items,
            count: count,
        }
    }
}

pub trait Load {
    fn load(path: &str) -> Self;
}

pub type SlcspList = List<Slcsp>;

impl Load for SlcspList {
    fn load(path: &str) -> Self {
        let mut items: Vec<Slcsp> = Vec::default();
        let mut count: usize = 0;

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut csv_reader = csv::Reader::from_reader(reader);
        for record in csv_reader.records() {
            let record = record.expect("malformed record");
            items.push(Slcsp::from(record));
            count += 1;
        }

        List::<Slcsp>::new(items, count)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Slcsp {
    pub zipcode: u32,
    pub rate: Option<f32>,
}

impl fmt::Display for Slcsp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = format!(
            "{},{}",
            self.zipcode,
            match self.rate {
                Some(r) => format!("{:.2}", r),
                None => String::new(),
            }
        );
        write!(f, "{}", res)
    }
}

impl From<csv::StringRecord> for Slcsp {
    fn from(i: csv::StringRecord) -> Self {
        i.deserialize(None)
            .expect("failed to parse record (malformed)")
    }
}

pub type ZipcodeList = List<Zipcode>;

impl Load for ZipcodeList {
    fn load(path: &str) -> Self {
        let mut items: Vec<Zipcode> = Vec::default();
        let mut count: usize = 0;

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut csv_reader = csv::Reader::from_reader(reader);
        for record in csv_reader.records() {
            let record = record.expect("malformed record");
            items.push(Zipcode::from(record));
            count += 1;
        }

        List::<Zipcode>::new(items, count)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Zipcode {
    pub zipcode: u32,
    pub state: String,
    pub county_code: String,
    pub name: String,
    pub rate_area: u8,
}

impl From<csv::StringRecord> for Zipcode {
    fn from(i: csv::StringRecord) -> Self {
        i.deserialize(None)
            .expect("failed to parse record (malformed)")
    }
}

impl Clone for Zipcode {
    fn clone(&self) -> Self {
        Zipcode {
            zipcode: self.zipcode,
            state: String::from(self.state.as_str()),
            county_code: String::from(self.county_code.as_str()),
            name: String::from(self.name.as_str()),
            rate_area: self.rate_area,
        }
    }
}

pub fn read_zipcodes_into(zipcodes: &mut Vec<Zipcode>, path: &str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);
    for record in csv_reader.records() {
        let record = record.unwrap();
        zipcodes.push(Zipcode::from(record));
    }
}

pub type PlanList = List<Plan>;

impl Load for PlanList {
    fn load<'a>(path: &str) -> Self {
        let mut items: Vec<Plan> = Vec::default();
        let mut count: usize = 0;

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut csv_reader = csv::Reader::from_reader(reader);
        for record in csv_reader.records() {
            let record = record.expect("malformed record");
            items.push(Plan::from(record));
            count += 1;
        }

        List::<Plan>::new(items, count)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Plan {
    pub plan_id: String,
    pub state: String,
    pub metal_level: String,
    pub rate: f32,
    pub rate_area: u8,
}

impl From<csv::StringRecord> for Plan {
    fn from(i: csv::StringRecord) -> Self {
        i.deserialize(None).expect("failed to parse record")
    }
}

impl Clone for Plan {
    fn clone(&self) -> Self {
        Plan {
            plan_id: String::from(self.plan_id.as_str()),
            state: String::from(self.state.as_str()),
            metal_level: String::from(self.metal_level.as_str()),
            rate: self.rate,
            rate_area: self.rate_area,
        }
    }
}
