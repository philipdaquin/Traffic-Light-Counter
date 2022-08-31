use chrono::{NaiveDateTime, DateTime, Utc, FixedOffset, NaiveDate};
use std::{fs::File, collections::{HashMap, BTreeMap}, io::{BufReader, BufRead}};

type Table = Vec<(NaiveDateTime, i32)>;

#[derive(Debug, Clone)]
struct DataTable { 
    raw_data: Table,
    aggregated_table: HashMap<NaiveDate, i32>   
}

impl DataTable { 
    fn new(raw_data: Table) -> Self { 
        
        Self {  
            raw_data,
            aggregated_table: HashMap::new()
        }
    }
    fn get_total_cars(&self) -> i32 { 

        let total = self.raw_data
            .iter()
            .fold(0,|mut sum, (_, count) | {
                sum += count;
                sum
            }
        );

        println!("The total number of cars seen: {total:?}");
        return total 
        
    }
    fn get_table(&mut self) { 
        let mut table: HashMap<NaiveDate, i32> = HashMap::new();
        let data_set = &self.raw_data;

        for (_, (date, count)) in data_set.iter().enumerate() { 
            *table.entry(date.date()).or_default() += count;
        }
        self.aggregated_table = table.clone();

        println!("The Date-CountOfCars in sorted order: {:#?}", table);

    }
    fn get_top_cars(&self, num: usize) { 
        let mut v: Vec<(NaiveDateTime, i32)> = self.raw_data.clone().into_iter().collect();
        v.sort_by(|(_, a), (_, b)| b.cmp(&a));
        println!("{v:?}");

        for i in 0..num { 
            let res = v[i];
            println!("{i} : {res:?}")
        }
    }
    /// Returns the contigous records 
    fn get_least_cars(&self, time_period: f32) { 
        let mut v: Vec<(NaiveDateTime, i32)> = self.raw_data.clone().into_iter().collect();
        


        println!("{v:#?}")
    }


}



fn main() {

    //  Load the data
    let data = load_data("./data/data.item");
    // Insert the data 
    let mut table = DataTable::new(data);
    

    let total_cars = table.get_total_cars();
    let data_table = table.get_table();
    let top_three = table.get_top_cars(3);
    let least_cars = table.get_least_cars(2.0);
  

}


fn load_data(path: &str) -> Vec<(NaiveDateTime, i32)>
 { 
    let file = File::open(path).expect("Unable to open file");
    let file_reader = BufReader::new(file);
    let mut table = Vec::new();
    
    for line in file_reader.lines() { 
        let res = line.unwrap();
        let mut iter = res.trim().split_whitespace();

        let date = NaiveDateTime::parse_from_str(
            iter.next().expect(""), "%Y-%m-%dT%H:%M:%S").expect("");
        let count = iter.next().expect("").parse::<i32>().expect("");

        table.push((date, count));
    }
    return table
}
