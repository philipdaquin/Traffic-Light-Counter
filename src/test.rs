use crate::DataTable;


#[cfg(test)]
mod tests {
    use chrono::{NaiveDateTime, NaiveDate};  
    use super::*;
    
    #[test]
    fn test_empty_data() { 
        let table = DataTable::new()
            .insert_data(Vec::<(NaiveDateTime, i32)>::new())
            .raw_data;

        assert_eq!(table.len(), 0) 
    }
    #[test]
    fn test_adding_data() { 
        assert!(DataTable::load_data("./data/test.data").is_some(), "Something went wrong")
    }
    #[test]
    fn test_total_cars() { 
        let data = DataTable::load_data("./data/test.data").unwrap();
        let table = DataTable::new().insert_data(data);
        let sum = table.get_total_cars();

        assert!(sum.is_some(), "Unable to calculate the sum of cars");
        assert_eq!(sum.unwrap(), 17)
    }
    #[test]
    fn test_sum_correctness() { 
        let data = DataTable::load_data("./data/test.data").unwrap();
        let table = DataTable::new().insert_data(data);
        let sum = table.get_total_cars();

        assert_ne!(sum.unwrap(), 0, "testting if the sum: {sum:?} != 0")
    }
    #[test]
    fn test_get_sorted_table() { 
        let data = DataTable::load_data("./data/test.data").unwrap();
        let table = DataTable::new().insert_data(data);


        let dt = table.clone().get_table();
        assert_eq!(dt.is_some(), true, "testing if we can get the table in the yy-mm-dd format: {dt:?} ");
        assert_eq!(dt.unwrap().get(&NaiveDate::from_ymd(2022,11, 1)).is_some(), true)
    }

    #[test]
    fn test_get_top_five_cars() { 
        let data = DataTable::load_data("./data/test.data").unwrap();
        let table = DataTable::new().insert_data(data);
        let top = table.get_top_cars(1);
        println!("{top:?}");
        assert!(top.is_some(), "test failed, unable to get data from table")
    }

    #[test]
    fn test_get_least_traffic() { 
        let data = DataTable::load_data("./data/test.data").unwrap();
        let table = DataTable::new().insert_data(data);
        let interval = table.get_least_interval(1.5);

        assert!(interval.is_some(), "test failed, unable to get data from table")
    }






}