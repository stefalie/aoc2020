use std::collections::HashMap;

pub fn run() {
    let input_bytes = include_bytes!("day4_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    fn split_line_or_entry(line: &str) -> HashMap<&str, &str> {
        // Split each line into (key, value) tuples and create a HashMap from it.
        let entry: HashMap<&str, &str> = line
            .split_whitespace()
            .map(|f| {
                let mut key_val = f.split(':');
                return (key_val.next().unwrap(), key_val.next().unwrap());
            })
            .collect();
        return entry;
    }

    let mut entries = Vec::new();
    let mut curr_entry = HashMap::new();

    for line in input_string.lines() {
        if line.is_empty() {
            entries.push(curr_entry.clone());
            curr_entry.clear();
        } else {
            curr_entry.extend(split_line_or_entry(line));
        }
    }
    entries.push(curr_entry);

    // This is an alternative, but it hardcodes the platform-depending newlines.
    //let entries: Vec<HashMap<&str, &str>> = input_string
    //    .split("\r\n\r\n")  // Ugh
    //    .map(|l| split_line_or_entry(l))
    //    .collect();

    part1(&entries);
    part2(&entries);
}

fn contains_required_fields(entry: &HashMap<&str, &str>) -> bool {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /*"cid"*/];
    return required_fields.iter().all(|f| entry.contains_key(f));
}

fn part1(entries: &[HashMap<&str, &str>]) {
    let mut num_valid_passports = 0;

    for e in entries {
        if contains_required_fields(e) {
            num_valid_passports += 1;
        }
    }

    println!("Day 4, part 1: {}", num_valid_passports);
}

fn part2(entries: &[HashMap<&str, &str>]) {
    let mut num_valid_passports = 0;

    fn is_valid(entry: &HashMap<&str, &str>) -> bool {
        if !contains_required_fields(entry) {
            return false;
        }

        fn in_range(maybe_str: &str, min: i32, max: i32) -> bool {
            match maybe_str.parse::<i32>() {
                Ok(val) if (val < min) || (val > max) => return false,
                Ok(_val) => return true,
                Err(..) => return false,
            }
        }

        if !in_range(entry.get("byr").unwrap(), 1920, 2002) {
            return false;
        }
        if !in_range(entry.get("iyr").unwrap(), 2010, 2020) {
            return false;
        }
        if !in_range(entry.get("eyr").unwrap(), 2020, 2030) {
            return false;
        }

        let hgt = entry.get("hgt").unwrap();
        if let Some(cm) = hgt.strip_suffix("cm") {
            if !in_range(&cm, 150, 193) {
                return false;
            }
        } else if let Some(inches) = hgt.strip_suffix("in") {
            if !in_range(&inches, 59, 76) {
                return false;
            }
        } else {
            return false;
        }

        let hcl = entry.get("hcl").unwrap();
        if let Some(color) = hcl.strip_prefix('#') {
            if (color.len() != 6) || color.contains(|c: char| !c.is_ascii_hexdigit()) {
                return false;
            }
        } else {
            return false;
        }

        let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !valid_eye_colors.contains(entry.get("ecl").unwrap()) {
            return false;
        }

        let pid = entry.get("pid").unwrap();
        if (pid.len() != 9) || pid.contains(|c: char| !c.is_numeric()) {
            return false;
        }

        return true;
    }

    for e in entries {
        if is_valid(e) {
            num_valid_passports += 1;
        }
    }

    println!("Day 4, part 2: {}", num_valid_passports);
}
