use chrono::{NaiveDateTime, DateTime, Utc, FixedOffset, NaiveDate};
use std::{fs::File, collections::{HashMap, BTreeMap}, io::{BufReader, BufRead}};

type Table = BTreeMap<NaiveDateTime, i32>;

#[derive(Debug, Clone)]
struct DataTable { 
    data: Table   
}

impl DataTable { 
    fn new(data: Table) -> Self { 
        Self { 
            data
        }
    }
    fn get_total_cars(&self) -> i32 { 
        let total = self.data.values().sum::<i32>();
        println!("The number of cars seen in total: {total}");
        
        return total 
        
    }
    fn get_table(&self) -> &Table { 
        let table = &self.data;
        println!("The Date-CountOfCars in sorted order: {table:#?}");

        return table 
    }
    fn get_top_cars(&self, num: usize) { 
        let mut v: Vec<(NaiveDateTime, i32)> = self.data.clone().into_iter().collect();
        v.sort_by(|(_, a), (_, b)| b.cmp(&a));
        println!("{v:#?}");
        for i in 0..num { 
            let res = v[i];
            println!("{res:?}")
        }
    }
    /// Returns the contigous records 
    fn get_least_cars(&self, time_period: f32) -> Vec<Table> { 
        todo!()
    }


}



fn main() {

    //  Load the data
    let data = load_data("./data/data.item");
    // Insert the data 
    let table = DataTable::new(data);
    

    let total_cars = table.get_total_cars();
    let data_table = table.get_table();
    let top_three = table.get_top_cars(3);
  

}


fn load_data(path: &str) -> BTreeMap<NaiveDateTime, i32>
 { 
    let file = File::open(path).expect("Unable to open file");
    let file_reader = BufReader::new(file);
    let mut table: BTreeMap<NaiveDateTime, i32> = BTreeMap::new();

    for line in file_reader.lines() { 
        let res = line.unwrap();
        let mut iter = res.trim().split_whitespace();

        let date = NaiveDateTime::parse_from_str(
            iter.next().expect(""), "%Y-%m-%dT%H:%M:%S").expect("");
        let count = iter.next().expect("").parse::<i32>().expect("");

        table.insert(date, count);
    }
    return table
}
