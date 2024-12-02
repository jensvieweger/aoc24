fn parse_data(data: &Vec<String>) -> Option<Vec<Vec<u32>>> {
    let mut result_1:Vec<u32> = Vec::new();
    let mut result_2:Vec<u32> = Vec::new();
    let mut result:Vec<Vec<u32>> = Vec::new();

    for line in data {
        let locations: Vec<_> = line.split_whitespace().collect();

        let location1: u32 = locations[0].parse().unwrap();
        result_1.push(location1);

        let location2: u32 = locations[1].parse().unwrap();
        result_2.push(location2);
    }
    result.push(result_1);
    result.push(result_2);
    Some(result)
}

fn calc_dists(lists: &Vec<Vec<u32>>) -> Option<Vec<u32>> {
    let mut result:Vec<u32> = Vec::new();
    let list1 = &lists[0];
    let list2 = &lists[1];

    for it in list1.iter().zip(list2.iter()) {
        let (loc1, loc2) = it;
        result.push(loc1.abs_diff(*loc2));
    }

    Some(result)
}

pub fn get_day01(data: &Vec<String>) -> Option<u32> {
    let parsed_data = parse_data(data);

    if parsed_data.is_none() {
        return None;
    }

    let mut lists = parsed_data.unwrap();
    lists[0].sort();
    lists[1].sort();

    let dists = calc_dists(&lists);

    if dists.is_none() {
        return None;
    }

    let result:u32 = dists.unwrap().iter().map(|&b| b as u32).sum();

    Some(result)
}

fn calc_similarities(lists: &Vec<Vec<u32>>) -> Option<Vec<u32>> {
    let mut result:Vec<u32> = Vec::new();
    let list1 = &lists[0];
    let list2 = &lists[1];

    for loc1 in list1.iter() {
        result.push(*loc1 * u32::try_from(list2.iter().filter(|&x| *x == *loc1).count()).unwrap());
    }

    Some(result)
}

pub fn get_day01_part2(data: &Vec<String>) -> Option<u32> {
    let parsed_data = parse_data(data);

    if parsed_data.is_none() {
        return None;
    }

    let mut lists = parsed_data.unwrap();
    lists[0].sort();
    lists[1].sort();

    let similarities = calc_similarities(&lists);

    if similarities.is_none() {
        return None;
    }

    let result:u32 = similarities.unwrap().iter().map(|&b| b as u32).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn fill_input() -> Vec<String> {
        let mut data = Vec::new();

        data.push("3   4".to_string());
        data.push("4   3".to_string());
        data.push("2   5".to_string());
        data.push("1   3".to_string());
        data.push("3   9".to_string());
        data.push("3   3".to_string());

        data
    }

    #[test]
    fn test_parse_data() {
        let mut list1:Vec<u32> = Vec::new();
        let mut list2:Vec<u32> = Vec::new();
        let mut lists:Vec<Vec<u32>> = Vec::new();

        list1.push(1);
        list1.push(2);
        list1.push(3);
        list1.push(3);
        list1.push(3);
        list1.push(4);

        list2.push(3);
        list2.push(3);
        list2.push(3);
        list2.push(4);
        list2.push(5);
        list2.push(9);

        lists.push(list1);
        lists.push(list2);


        let dists = calc_dists(&lists);

        assert_eq!(dists.is_none(), false);
        let distsv = dists.unwrap();
        assert_eq!(distsv.len(), 6);

        assert_eq!(distsv[0], 2);
        assert_eq!(distsv[1], 1);
        assert_eq!(distsv[2], 0);
        assert_eq!(distsv[3], 1);
        assert_eq!(distsv[4], 2);
        assert_eq!(distsv[5], 5);
    }

    #[test]
    fn test_calc_dist() {
        let data = fill_input();

        let data = parse_data(&data);

        assert_eq!(data.is_none(), false);
        let datav = data.unwrap();
        assert_eq!(datav.len(), 2);

        let first_list = &datav[0];
        let second_list = &datav[1];

        assert_eq!(first_list.len(), 6);
        assert_eq!(first_list[0], 3);
        assert_eq!(first_list[1], 4);
        assert_eq!(first_list[2], 2);
        assert_eq!(first_list[3], 1);
        assert_eq!(first_list[4], 3);
        assert_eq!(first_list[5], 3);

        assert_eq!(second_list.len(), 6);
        assert_eq!(second_list[0], 4);
        assert_eq!(second_list[1], 3);
        assert_eq!(second_list[2], 5);
        assert_eq!(second_list[3], 3);
        assert_eq!(second_list[4], 9);
        assert_eq!(second_list[5], 3);
    }

    #[test]
    fn test_get_day01() {
        let data = fill_input();

        let day = get_day01(&data);

        assert_eq!(day.is_none(), false);

        assert_eq!(day.unwrap(), 11);

    }


    #[test]
    fn test_get_day01_part2() {
        let data = fill_input();

        let day = get_day01_part2(&data);

        assert_eq!(day.is_none(), false);

        assert_eq!(day.unwrap(), 31);

    }
}