use crate::read_lines;

fn create_bit_counter(data: &[String]) -> Vec<i64> {
    let binary_length = &data[0].len();
    let mut bit_counter = vec![0; *binary_length];
    for x in data {
        for (idx, c) in x.chars().enumerate() {
            if c == '0' {
                bit_counter[idx] -= 1;
            } else if c == '1' {
                bit_counter[idx] += 1;
            };
        }
    }
    bit_counter
}

pub fn part_1() {
    let data = read_lines("res/day_3.txt");

    let bit_counter = create_bit_counter(&data);

    let gamma = find_gamma_rating(&bit_counter);

    let epsilon = find_epsilon_rating(&bit_counter);

    println!(
        "Part 1: {} ({:#0b}) x {} ({:#0b}) = {}",
        gamma,
        gamma,
        epsilon,
        epsilon,
        gamma * epsilon
    );
}

fn find_epsilon_rating(bit_counter: &[i64]) -> u64 {
    let mut epsilon = String::new();

    for frequency in bit_counter {
        if frequency > &0 {
            epsilon += "0";
        } else {
            epsilon += "1";
        }
    }

    u64::from_str_radix(&epsilon, 2).expect("Cannot convert to binary string")
}

fn find_gamma_rating(bit_counter: &[i64]) -> u64 {
    let mut gamma = String::new();

    for frequency in bit_counter {
        if frequency > &0 {
            gamma += "1";
        } else {
            gamma += "0";
        }
    }

    u64::from_str_radix(&gamma, 2).expect("Cannot convert to binary string")
}

pub fn part_2() {
    let data = read_lines("res/day_3.txt");

    let bit_length = data[0].len();

    let oxygen_generator = find_oxygen_generator_rating(data.clone(), bit_length);

    let co2_scrubber = find_co2_scrubber_rating(data, bit_length);

    println!(
        "Part 2: {} ({:#0b}) x {} ({:#0b}) = {}",
        oxygen_generator,
        oxygen_generator,
        co2_scrubber,
        co2_scrubber,
        oxygen_generator * co2_scrubber
    );
}

fn find_co2_scrubber_rating(data: Vec<String>, bit_length: usize) -> u64 {
    let mut lsb_data = data;

    for idx in 0..bit_length {
        let bit_counter = create_bit_counter(&lsb_data);
        lsb_data = lsb_data
            .iter()
            .filter(|x| {
                matches!(
                    (x.chars().nth(idx).unwrap(), bit_counter[idx].cmp(&0)),
                    ('0', std::cmp::Ordering::Equal | std::cmp::Ordering::Greater)
                        | ('1', std::cmp::Ordering::Less)
                )
            })
            .cloned()
            .collect();
        if lsb_data.len() == 1 {
            break;
        }
    }

    u64::from_str_radix(&lsb_data[0], 2).unwrap()
}

fn find_oxygen_generator_rating(data: Vec<String>, bit_length: usize) -> u64 {
    let mut msb_data = data;

    for idx in 0..bit_length {
        let bit_counter = create_bit_counter(&msb_data);
        msb_data = msb_data
            .iter()
            .filter(|x| {
                matches!(
                    (x.chars().nth(idx).unwrap(), bit_counter[idx].cmp(&0)),
                    ('1', std::cmp::Ordering::Equal | std::cmp::Ordering::Greater)
                        | ('0', std::cmp::Ordering::Less)
                )
            })
            .cloned()
            .collect();
        if msb_data.len() == 1 {
            break;
        }
    }

    u64::from_str_radix(&msb_data[0], 2).unwrap()
}
