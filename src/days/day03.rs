use regex::Regex;

fn parse_data(data: &Vec<String>) -> Option<Vec<(u32,u32)>> {
    let mut result = Vec::new();

    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    for line in data {
        for (_, [factor_a, factor_b]) in re.captures_iter(line).map(|c| c.extract()) {
            result.push((factor_a.parse::<u32>().ok()?, factor_b.parse::<u32>().ok()?));
        }
    }
    //println!("{:?}", result);
    Some(result)
}

pub fn get_day03(data: &Vec<String>) -> Option<u32> {
    let parsed_data = parse_data(data);

    if parsed_data.is_none() {
        return None;
    }

    let mut result = 0;

    for (factora, factorb) in parsed_data.unwrap() {
        result += factora * factorb;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn fill_input() -> Vec<String> {
        let mut data = Vec::new();
        
        data.push("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string());

        data
    }

    #[test]
    fn test_parse_data() {
        let data = fill_input();

        let parsed_data = parse_data(&data);

        assert_eq!(parsed_data.is_none(), false);

        let final_data=parsed_data.unwrap();

        assert_eq!(final_data.len(), 4);

        assert_eq!(final_data[0], (2, 4));
        assert_eq!(final_data[1], (5, 5));
        assert_eq!(final_data[2], (11, 8));
        assert_eq!(final_data[3], (8, 5));
    }

    #[test]
    fn test_get_day03() {
        let data = fill_input();

        let day = get_day03(&data);

        assert_eq!(day.is_none(), false);

        assert_eq!(day.unwrap(), 161);
    }

}