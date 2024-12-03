fn parse_data(data: &Vec<String>) -> Option<Vec<Vec<i32>>> {
    let mut result:Vec<Vec<i32>> = Vec::new();

    for line in data {
        let numbers: Result<Vec<i32>, _> = line.split(" ").map(|x| x.parse()).collect();

        match numbers {
            Ok(i) => result.push(i),
            Err(_) => return None,
        }
    }
    Some(result)
}

fn is_report_valid(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return false
    }

    let mut ascending = false;
    if report[1] > report[0] {
        ascending = true;
    } else if report[1] == report[0] {
        return false;
    }

    println!("{:?}", report);
    let diffs = report.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    println!("{:?}", diffs);

    for diff in diffs {
        if diff == 0 {
            return false;
        }
        if diff.abs() > 3 {
            return false;
        }
        if ascending && diff < 0 {
            return false;
        }
        if !ascending && diff > 0 {
            return false;
        }
    }
    true
}

pub fn get_day02(data: &Vec<String>) -> Option<i32> {
    let parsed_data = parse_data(data);

    if parsed_data.is_none() {
        return None;
    }

    let mut valid_reports:i32 = 0;

    let reports = parsed_data.unwrap();

    for report in reports {
        if is_report_valid(&report) {
            valid_reports += 1;
        }
    }

    Some(valid_reports)
}

pub fn get_day02_part2(data: &Vec<String>) -> Option<i32> {
    let parsed_data = parse_data(data);

    if parsed_data.is_none() {
        return None;
    }

    let mut valid_reports:i32 = 0;

    let reports = parsed_data.unwrap();

    for report in reports {
        if is_report_valid(&report) {
            valid_reports += 1;
        } else {
            for index in 0..report.len() {
                let mut omg_cloned_vector = report.clone();
                omg_cloned_vector.remove(index);
                if is_report_valid(&omg_cloned_vector) {
                    valid_reports += 1;
                    break;
                }
            }
        }
    }

    Some(valid_reports)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn fill_input() -> Vec<String> {
        let mut data = Vec::new();

        data.push("7 6 4 2 1".to_string());
        data.push("1 2 7 8 9".to_string());
        data.push("9 7 6 2 1".to_string());
        data.push("1 3 2 4 5".to_string());
        data.push("8 6 4 4 1".to_string());
        data.push("1 3 6 7 9".to_string());

        data
    }

    #[test]
    fn test_parse_data() {
        let data = fill_input();

        let parsed_data = parse_data(&data);

        assert_eq!(parsed_data.is_none(), false);

        let reports = parsed_data.unwrap();
        assert_eq!(reports.len(), 6);

        {
            let report = &reports[0];
            assert_eq!(report.len(), 5);

            assert_eq!(report[0], 7);
            assert_eq!(report[1], 6);
            assert_eq!(report[2], 4);
            assert_eq!(report[3], 2);
            assert_eq!(report[4], 1);
        }

        {
            let report = &reports[1];
            assert_eq!(report.len(), 5);

            assert_eq!(report[0], 1);
            assert_eq!(report[1], 2);
            assert_eq!(report[2], 7);
            assert_eq!(report[3], 8);
            assert_eq!(report[4], 9);
        }

        {
            let report = &reports[2];
            assert_eq!(report.len(), 5);

            assert_eq!(report[0], 9);
            assert_eq!(report[1], 7);
            assert_eq!(report[2], 6);
            assert_eq!(report[3], 2);
            assert_eq!(report[4], 1);
        }

        {
            let report = &reports[3];
            assert_eq!(report.len(), 5);

            assert_eq!(report[0], 1);
            assert_eq!(report[1], 3);
            assert_eq!(report[2], 2);
            assert_eq!(report[3], 4);
            assert_eq!(report[4], 5);

        }

        {
            let report = &reports[4];
            assert_eq!(report.len(), 5);

            assert_eq!(report[0], 8);
            assert_eq!(report[1], 6);
            assert_eq!(report[2], 4);
            assert_eq!(report[3], 4);
            assert_eq!(report[4], 1);
        }

        {
            let report = &reports[5];
            assert_eq!(report.len(), 5);

            assert_eq!(report[0], 1);
            assert_eq!(report[1], 3);
            assert_eq!(report[2], 6);
            assert_eq!(report[3], 7);
            assert_eq!(report[4], 9);
        }

    }

    #[test]
    fn test_is_valid_report() {
        let mut report:Vec<i32> = Vec::new();

        report.push(1);
        report.push(3);
        report.push(6);
        report.push(7);
        report.push(9);

        assert_eq!(is_report_valid(&report), true);

        report.clear();


        report.push(8);
        report.push(6);
        report.push(4);
        report.push(4);
        report.push(1);

        assert_eq!(is_report_valid(&report), false);

    }

    #[test]
    fn test_get_day02() {
        let data = fill_input();

        let day = get_day02(&data);

        assert_eq!(day.is_none(), false);

        assert_eq!(day.unwrap(), 2);

    }

    #[test]
    fn test_get_day02_part2() {
        let data = fill_input();

        let day = get_day02_part2(&data);

        assert_eq!(day.is_none(), false);

        assert_eq!(day.unwrap(), 4);

    }

}