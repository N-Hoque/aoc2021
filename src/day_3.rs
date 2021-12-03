use crate::read_lines;

#[derive(Debug, Default, Copy, Clone)]
struct Frequency(i64);

fn create_bit_counter(data: &[String]) -> Vec<Frequency> {
    let binary_length = &data[0].len();
    let mut bit_counter = vec![Frequency::default(); *binary_length];
    for x in data {
        for (idx, c) in x.chars().enumerate() {
            if c == '0' {
                bit_counter[idx].0 -= 1;
            } else if c == '1' {
                bit_counter[idx].0 += 1;
            };
        }
    }
    bit_counter
}

pub fn part_1() {
    let data = read_lines("res/day_3.txt");

    let bit_counter = create_bit_counter(&data);

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for frequency in bit_counter {
        if frequency.0 > 0 {
            gamma += "1";
        } else {
            gamma += "0";
        }

        if frequency.0 > 0 {
            epsilon += "0";
        } else {
            epsilon += "1";
        }
    }

    let gamma = u64::from_str_radix(&gamma, 2).expect("Cannot convert to binary string");
    let epsilon = u64::from_str_radix(&epsilon, 2).expect("Cannot convert to binary string");

    println!("{} x {} = {}", gamma, epsilon, gamma * epsilon);
}

pub fn part_2() {
    let data = read_lines("res/day_3.txt");

    let bit_length = data[0].len();

    let mut msb_data = data.clone();

    for idx in 0..bit_length {
        let bit_counter = create_bit_counter(&msb_data);
        msb_data = msb_data
            .iter()
            .filter(|x| {
                let char = x.chars().nth(idx).unwrap();

                if bit_counter[idx].0 == 0 {
                    char == '1'
                } else {
                    char == '1' && bit_counter[idx].0 > 0 || char == '0' && bit_counter[idx].0 < 0
                }
            })
            .cloned()
            .collect();
        if msb_data.len() == 1 {
            break;
        }
    }

    let mut lsb_data = data;

    for idx in 0..bit_length {
        let bit_counter = create_bit_counter(&lsb_data);
        lsb_data = lsb_data
            .iter()
            .filter(|x| {
                let char = x.chars().nth(idx).unwrap();

                if bit_counter[idx].0 == 0 {
                    char == '0'
                } else {
                    char == '0' && bit_counter[idx].0 > 0 || char == '1' && bit_counter[idx].0 < 0
                }
            })
            .cloned()
            .collect();
        if lsb_data.len() == 1 {
            break;
        }
    }

    let oxygen_generator = u64::from_str_radix(&msb_data[0], 2).unwrap();
    let co2_scrubber = u64::from_str_radix(&lsb_data[0], 2).unwrap();

    println!(
        "{} x {} = {}",
        oxygen_generator,
        co2_scrubber,
        oxygen_generator * co2_scrubber
    );
}
