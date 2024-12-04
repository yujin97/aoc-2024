use std::fs::read_to_string;

fn main() {
    let raw_reports = read_to_string("src/inputs/day2.txt").expect("Failed to load input file!");
    let mut reports: Vec<Vec<usize>> = Vec::new();

    let mut number_of_safe_report = 0usize;

    for raw_report in raw_reports.lines() {
        let mut report: Vec<usize> = Vec::new();

        for level in raw_report.split_whitespace() {
            let level: usize = level.parse().expect("Failed to parse level to usize");
            report.push(level);
        }

        reports.push(report);
    }

    for report in reports {
        let mut report_iter = report.iter();

        let mut is_safe = true;

        let first = report_iter.next().unwrap();
        let second = report_iter.next().unwrap();

        let mut previous_level = second;

        let is_increasing = first < second;

        if is_increasing {
            let difference = second - first;
            if difference > 3 {
                continue;
            }
        } else {
            let difference = first - second;
            if difference == 0 || difference > 3 {
                continue;
            }
        }

        for level in report_iter {
            let is_increasing_now = previous_level < level;

            if is_increasing != is_increasing_now {
                is_safe = false;
                break;
            }

            // make sure the difference is 1 - 3
            let difference = if previous_level < level {
                level - previous_level
            } else {
                previous_level - level
            };

            if !(difference > 0 && difference <= 3) {
                is_safe = false;
                break;
            }

            previous_level = level;
        }

        if is_safe {
            number_of_safe_report += 1
        };
    }

    println!("Number of safe reports: {number_of_safe_report}");
}
