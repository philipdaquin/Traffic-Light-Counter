mod test;

use chrono::{NaiveDateTime, DateTime, Utc, FixedOffset, NaiveDate};
use std::{fs::File, collections::{HashMap, BTreeMap, HashSet, VecDeque}, io::{BufReader, BufRead}};

type Table = Vec<(NaiveDateTime, i32)>;

#[derive(Debug, Clone, Default)]
pub struct DataTable { 
    pub raw_data: Table
}

impl DataTable { 
    pub fn new() -> Self { 
        DataTable::default()
    }
    pub fn insert_data(&mut self, raw_data: Table) -> Self { 
        Self { 
            raw_data
        }
    }
    /// Time: O(n)
    /// Space: O(n)
    pub fn get_total_cars(&self) -> Option<i32> { 

        let total = self.raw_data
            .iter()
            .fold(0,|mut sum, (_, count) | {
                sum += count;
                sum
            }
        );

        println!("The total number of cars seen: {total:?}");
        return Some(total) 
        
    }
    //  Time: O(n)
    //  Space O(n)
    pub fn get_table(&mut self) -> Option<HashMap<NaiveDate, i32>> { 
        let mut table: HashMap<NaiveDate, i32> = HashMap::new();
        let data_set = &self.raw_data;

        for (_, (date, count)) in data_set.iter().enumerate() { 
            *table.entry(date.date()).or_default() += count;
        }

        println!("The Date-CountOfCars in sorted order: {:#?}", table);
        return Some(table)
    }

    /// Time: O(logN)
    /// Space: O(n)
    pub fn get_top_cars(&self, num: usize) -> Option<Table> { 
        let mut v: Vec<(NaiveDateTime, i32)> = self.raw_data
            .clone()
            .into_iter()
            .collect();
        v.sort_by(|(_, a), (_, b)| b.cmp(&a));
        
        let mut ret = vec![];
        for i in 0..num { 
            let res = v[i];
            ret.push(res);
        }
        println!("Top {num} half hours with most cars: {ret:?}");
        Some(ret)
    }

    /// Time: O(n)  
    /// Space: O(n)
    /// Example: get_least_interval(1.5hours) = 3 consecutive
    pub fn get_least_interval(&self, time_period: f32) -> Option<Table>  { 

        
        let nums = self
            .raw_data
            .clone();
        let mut interval = (time_period * 2.0) as usize;
        
        let window_sum = (0..interval)
            .fold(0, |mut sum, i| {
                sum += nums[i].1; 
                sum
        });
        let mut window = window_sum;
        let mut curr_sum = window;
        let mut deque = VecDeque::new();
        
        for i in interval..nums.len() { 
            let k = i - interval;
            let (one, two) = (nums[i].1, nums[k].1);
            curr_sum += one - two;

            if curr_sum < window { 
                deque.push_back((k, i));
                window = i32::min(window, curr_sum);
            }
        }
        // let mut res = vec![];
        let (k, i) = *deque
            .iter()
            .last()
            .expect("No Solution");
        
        return Some((k..i).fold(vec![], 
            |mut res, i|{
                res.push(nums[i]);
                res
        }))

    
    }
    pub fn load_data(path: &str) -> Option<Vec<(NaiveDateTime, i32)>> { 
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
        return Some(table)
    }
    fn add_data(&mut self, data: (NaiveDateTime, i32)) { 
        self.raw_data.push(data);
    }
    
}



fn main() {
    //  Load the data
    let data = DataTable::load_data("./data/data.item").expect("Unable to parse Data");
    // Insert the data 
    let mut table = DataTable::new().insert_data(data);
    let total_cars = table.get_total_cars();
    let data_table = table.get_table();
    let top_three = table.get_top_cars(3);
    let least_cars = table.get_least_interval(1.5);
    println!("{least_cars:?}");
}


