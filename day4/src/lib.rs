use std::{collections::HashMap, iter};

fn parse_passports(input: &str) -> impl Iterator<Item = HashMap<&str, &str>> {
    let mut passports = Vec::new();

    input
        .lines()
        .chain(iter::once(""))
        .fold(HashMap::new(), |mut state, line| {
            if line.is_empty() {
                passports.push(state);
                return HashMap::new();
            }

            state.extend(line.split_whitespace().map(|segment| {
                if let [key, value] = segment.split(':').collect::<Vec<&str>>()[..] {
                    (key, value)
                } else {
                    panic!("Invalid segment \"{}\"", segment);
                }
            }));

            state
        });

    passports.into_iter()
}

macro_rules! filter_invalid_fields {
    ($ex:expr, $($validator:ident),*) => {
        $ex
        $(
            .filter(|passport| {
                passport
                    .get(stringify!($validator))
                    .copied()
                    .map(validators::$validator)
                    .unwrap_or(false)
            })
        )*
    };
}

macro_rules! filter_missing_fields {
    ($ex:expr, $($validator:ident),*) => {
        $ex
        $(
            .filter(|passport| passport.get(stringify!($validator)).is_some())
        )*
    };
}

mod validators {
    pub fn byr(value: &str) -> bool {
        value
            .parse::<u16>()
            .map(|value| 1920 <= value && value <= 2002)
            .unwrap_or(false)
    }

    pub fn iyr(value: &str) -> bool {
        value
            .parse::<u16>()
            .map(|value| 2010 <= value && value <= 2020)
            .unwrap_or(false)
    }

    pub fn eyr(value: &str) -> bool {
        value
            .parse::<u16>()
            .map(|value| 2010 <= value && value <= 2030)
            .unwrap_or(false)
    }

    pub fn hgt(value: &str) -> bool {
        if value.ends_with("cm") {
            value[..value.len() - 2]
                .parse::<u16>()
                .map(|value| 150 <= value && value <= 193)
                .unwrap_or(false)
        } else if value.ends_with("in") {
            value[..value.len() - 2]
                .parse::<u16>()
                .map(|value| 59 <= value && value <= 76)
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn hcl(value: &str) -> bool {
        value.starts_with('#') && value.len() == 7 && value[1..].chars().all(|c| c.is_digit(16))
    }

    pub fn ecl(value: &str) -> bool {
        matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    }

    pub fn pid(value: &str) -> bool {
        value.len() == 9 && value.parse::<u32>().is_ok()
    }
}

pub fn challenge1(input: &str) -> usize {
    filter_missing_fields!(parse_passports(input), byr, iyr, eyr, hgt, hcl, ecl, pid).count()
}

pub fn challenge2(input: &str) -> usize {
    filter_invalid_fields!(parse_passports(input), byr, iyr, eyr, hgt, hcl, ecl, pid).count()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    #[test]
    fn test_challenge1() {
        assert_eq!(super::challenge1(INPUT), 2);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(super::challenge2(INPUT), 2);
    }

    #[test]
    fn test_validators() {
        use super::validators::*;

        assert_eq!(byr("2002"), true, "byr valid:   2002");
        assert_eq!(byr("2003"), false, "byr invalid: 2003");
        assert_eq!(hgt("60in"), true, "hgt valid:   60in");
        assert_eq!(hgt("190cm"), true, "hgt valid:   190cm");
        assert_eq!(hgt("190in"), false, "hgt invalid: 190in");
        assert_eq!(hgt("190"), false, "hgt invalid: 190");
        assert_eq!(hcl("#123abc"), true, "hcl valid:   #123abc");
        assert_eq!(hcl("#123abz"), false, "hcl invalid: #123abz");
        assert_eq!(hcl("123abc"), false, "hcl invalid: 123abc");
        assert_eq!(ecl("brn"), true, "ecl valid:   brn");
        assert_eq!(ecl("wat"), false, "ecl invalid: wat");
        assert_eq!(pid("000000001"), true, "pid valid:   000000001");
        assert_eq!(pid("0123456789"), false, "pid invalid: 0123456789");
    }
}
