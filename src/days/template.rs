fn parse_data(data: &Vec<String>) -> Option<Vec<u32>> {
    let mut result:Vec<u32> = Vec::new();

    for line in data {
        result.push(1);
    }
    Some(result)
}

pub fn get_day(data: &Vec<String>) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn fill_input() -> Vec<String> {
        let mut data = Vec::new();
        
        data.push("aaaa".to_string());
        data.push("bbbb".to_string());

        data
    }

    #[test]
    fn test_parse_data() {
        let data = fill_input();

        let parsed_data = parse_data(&data);

        assert_eq!(parsed_data.is_none(), false);

        let final_data=parsed_data.unwrap();
    }

    #[test]
    fn test_get_day() {
        let data = fill_input();

        let day = get_day(&data);

        assert_eq!(day.is_none(), false);

    }

}