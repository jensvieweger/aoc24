use regex::Regex;
use std::iter::zip;

fn parse_data(data: &Vec<String>) -> Option<(Vec<usize>,Vec<usize>,Vec<(usize, u32, u32)>)> {
    let mut factors = Vec::new();

    let re_fac = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let singleline = data.concat();

    for (_, [factor_a, factor_b]) in re_fac.captures_iter(&singleline).map(|c| c.extract()) {
        factors.push((0, factor_a.parse::<u32>().ok()?, factor_b.parse::<u32>().ok()?));
    }
    let mut f_iter = zip (factors.iter_mut(), re_fac.find_iter(&singleline).map(|f| f.start()));
    for (factor, start) in f_iter {
        factor.0 = start;
    }
    let dos:Vec<usize> = re_do.find_iter(&singleline).map(|f| f.start()).collect();
    let donts:Vec<usize> = re_dont.find_iter(&singleline).map(|f| f.start()).collect();

    //println!("{:?}", result);
    Some((dos, donts, factors))
}

fn filter_factors(dos: &Vec<usize>, donts: &Vec<usize>, factors: &Vec<(usize,u32,u32)>) -> Option<Vec<(u32,u32)>>{

    let mut valid_range = Vec::new();
    let mut start_valid = 0;
    let mut do_iter = dos.iter().peekable();
    let mut dont_iter = donts.iter();

    for dont in dont_iter {
        if do_iter.peek().is_some() {
            let next_do = do_iter.peek().unwrap(); 
            if dont < *next_do {
                valid_range.push((start_valid, *dont));
                start_valid = **next_do;   
                do_iter.next();     
            }
        }
    }
    // assume that there is at least one entry each
    if dos.last().unwrap() > donts.last().unwrap() {
        valid_range.push((*dos.last().unwrap(), usize::MAX));
    }

    //println!("{:?}", valid_range);

    // of course the rust way would probably be to create a filtering iterator
    let mut result = Vec::new();

    for factor in factors {
        for vr in &valid_range {
            if factor.0 > vr.0 && factor.0 < vr.1 {
                result.push((factor.1, factor.2));
                break;
            }
        }
    }

    Some(result)
}

pub fn get_day03_part2(data: &Vec<String>) -> Option<u32> {
    let parsed_data = parse_data(data);

    if parsed_data.is_none() {
        return None;
    }

    let mut result = 0;
    let (dos, donts, factors) = parsed_data.unwrap();

    let filtered_factors = filter_factors(&dos, &donts, &factors);

    if filtered_factors.is_none() {
        return None;
    }

    for (factora, factorb) in filtered_factors.unwrap() {
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
        
        data.push("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string());

        data
    }

    #[test]
    fn test_parse_data() {
        let data = fill_input();

        let parsed_data = parse_data(&data);

        assert_eq!(parsed_data.is_none(), false);

        let (dos, donts, factors) = parsed_data.unwrap();

        assert_eq!(dos.len(), 1);

        assert_eq!(dos[0], 59);

        assert_eq!(donts.len(), 1);

        assert_eq!(donts[0], 20);

        assert_eq!(factors.len(), 4);

        assert_eq!(factors[0], (1, 2, 4));
        assert_eq!(factors[1], (28, 5, 5));
        assert_eq!(factors[2], (48, 11, 8));
        assert_eq!(factors[3], (64, 8, 5));

    }

    #[test]
    fn test_filter_factors() {
        let dos = vec![59];
        let donts = vec![20];
        let factors = vec![(1, 2, 4), (28, 5, 5), (48, 11, 8), (64, 8, 5)];

        let filtered_factors = filter_factors(&dos, &donts, &factors);

        assert_eq!(filtered_factors.is_none(), false);

        let final_data=filtered_factors.unwrap();

        assert_eq!(final_data.len(), 2);

        assert_eq!(final_data[0], (2, 4));
        assert_eq!(final_data[1], (8, 5));

    }

    #[test]
    fn test_get_day03_part2() {
        let data = fill_input();

        let day = get_day03_part2(&data);

        assert_eq!(day.is_none(), false);

        assert_eq!(day.unwrap(), 48);
    }

}