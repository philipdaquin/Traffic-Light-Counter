use crate::DataTable;


#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;  
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

        assert!(sum.is_some(), "Unable to calculate the sum of cars") 
    }
    #[test]
    fn test_sum_correctness() { 
        let data = DataTable::load_data("./data/test.data").unwrap();
        let table = DataTable::new().insert_data(data);
        let sum = table.get_total_cars();

        assert_ne!(sum.unwrap(), 0, "testting if the sum: {sum:?} != 0")
    }
}