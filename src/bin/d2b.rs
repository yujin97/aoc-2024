use std::fs::read_to_string;

fn is_out_of_range(previous_level: usize, current_level: usize) -> bool {
    let difference;
    if previous_level < current_level {
        difference = current_level - previous_level;
    } else {
        difference = previous_level - current_level;
    }

    difference == 0 || difference > 3
}

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
        let mut up_stack = Vec::new();
        let mut down_stack = Vec::new();

        let mut up_tolerated = false;
        let mut up_iter = report.iter();
        let mut up_item = up_iter.next();

        let mut down_tolerated = false;
        let mut down_iter = report.iter();
        let mut down_item = down_iter.next();

        while up_item.is_some() {
            let up_value = up_item.unwrap();

            let top = up_stack.pop();

            if let Some(top_value) = top {
                if top_value > *up_value {
                    if up_tolerated {
                        up_stack.push(top_value);
                        break;
                    }
                    up_tolerated = true;
                    let next_item = up_iter.next();

                    if let Some(next_value) = next_item {
                        if top_value < *next_value && !is_out_of_range(top_value, *next_value) {
                            up_stack.push(top_value);
                            up_stack.push(*next_value);
                        } else if up_value < next_value && !is_out_of_range(*up_value, *next_value)
                        {
                            let previous_item = up_stack.pop();

                            if let Some(previous_value) = previous_item {
                                if *up_value > previous_value {
                                    up_stack.push(previous_value);
                                    up_stack.push(*up_value);
                                    up_stack.push(*next_value);
                                } else {
                                    up_stack.push(previous_value);
                                    break;
                                }
                            } else {
                                up_stack.push(*up_value);
                                up_stack.push(*next_value);
                            }
                        } else {
                            up_stack.push(top_value);
                            break;
                        }
                    } else {
                        up_stack.push(top_value);
                    }
                } else {
                    if is_out_of_range(top_value, *up_value) {
                        if up_tolerated {
                            up_stack.push(top_value);
                            break;
                        }
                        up_tolerated = true;
                        if up_stack.len() == 0 {
                            let next_item = up_iter.next();

                            if let Some(next_value) = next_item {
                                if !is_out_of_range(top_value, *next_value)
                                    && top_value < *next_value
                                {
                                    up_stack.push(top_value);
                                    up_stack.push(*next_value);
                                } else if !is_out_of_range(*up_value, *next_value)
                                    && up_value < next_value
                                {
                                    up_stack.push(*up_value);
                                    up_stack.push(*next_value);
                                } else {
                                    up_stack.push(top_value);
                                }
                            } else {
                                up_stack.push(top_value);
                            }
                        } else {
                            up_stack.push(top_value);
                        }
                    } else {
                        up_stack.push(top_value);
                        up_stack.push(*up_value);
                    }
                }
            } else {
                up_stack.push(*up_value);
            }

            up_item = up_iter.next();
        }

        while down_item.is_some() {
            let down_value = down_item.unwrap();

            let top = down_stack.pop();

            if let Some(top_value) = top {
                if top_value < *down_value {
                    if down_tolerated {
                        down_stack.push(top_value);
                        break;
                    }
                    down_tolerated = true;
                    let next_item = down_iter.next();

                    if let Some(next_value) = next_item {
                        if top_value > *next_value && !is_out_of_range(top_value, *next_value) {
                            down_stack.push(top_value);
                            down_stack.push(*next_value);
                        } else if down_value > next_value
                            && !is_out_of_range(*down_value, *next_value)
                        {
                            let previous_item = down_stack.pop();

                            if let Some(previous_value) = previous_item {
                                if *down_value < previous_value {
                                    down_stack.push(previous_value);
                                    down_stack.push(*down_value);
                                    if *next_value < previous_value {
                                        down_stack.push(*next_value);
                                    }
                                } else {
                                    down_stack.push(previous_value);
                                    break;
                                }
                            } else {
                                down_stack.push(*down_value);
                                down_stack.push(*next_value);
                            }
                        } else {
                            down_stack.push(top_value);
                            break;
                        }
                    } else {
                        down_stack.push(top_value);
                    }
                } else {
                    if is_out_of_range(top_value, *down_value) {
                        if down_tolerated {
                            down_stack.push(top_value);
                            break;
                        }
                        down_tolerated = true;
                        if down_stack.len() == 0 {
                            let next_item = down_iter.next();

                            if let Some(next_value) = next_item {
                                if !is_out_of_range(top_value, *next_value)
                                    && top_value > *next_value
                                {
                                    down_stack.push(top_value);
                                    down_stack.push(*next_value);
                                } else if !is_out_of_range(*down_value, *next_value)
                                    && down_value > next_value
                                {
                                    down_stack.push(*down_value);
                                    down_stack.push(*next_value);
                                } else {
                                    down_stack.push(top_value);
                                }
                            } else {
                                down_stack.push(top_value);
                            }
                        } else {
                            down_stack.push(top_value);
                        }
                    } else {
                        down_stack.push(top_value);
                        down_stack.push(*down_value);
                    }
                }
            } else {
                down_stack.push(*down_value);
            }

            down_item = down_iter.next();
        }

        if report.len() - up_stack.len() <= 1 || report.len() - down_stack.len() <= 1 {
            number_of_safe_report += 1;
            println!("{:?} -> {:?}/ {:?} O", report, up_stack, down_stack);
        } else {
            println!("{:?} -> {:?}/ {:?} X", report, up_stack, down_stack);
        }
    }

    println!("number of safe report: {}", number_of_safe_report);
}
