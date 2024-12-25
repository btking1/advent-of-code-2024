use regex::Regex;

pub fn p1() {
    let file = std::fs::read("file").unwrap();

    let mul = b"mul";

    let mut possible_mul = Vec::new();
    file.clone().into_iter().enumerate().for_each(|(i, _byte)| {
        if i > 1 {
            let buffer = &file[(i - 2)..=i];

            if buffer == mul {
                let next_buffer = &file[(i + 1)..=(i + 9)];
                let c_buffer = next_buffer
                    .into_iter()
                    .map(|byte| char::from(byte.to_owned()))
                    .collect::<Vec<_>>();

                possible_mul.push(c_buffer);
            }
        }
    });
    let has_comma = possible_mul.into_iter().filter(|c| c.contains(&','));

    let no_letter = has_comma.into_iter().map(|arr| {
        arr.into_iter()
            .filter(|c| !c.is_alphabetic())
            .collect::<Vec<char>>()
    });

    let no_special_char = no_letter
        .into_iter()
        .map(|arr| {
            arr.into_iter()
                .filter(|c| c.is_numeric() || *c == '(' || *c == ')' || *c == ',')
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();

    let verified_mul = no_special_char.into_iter().filter(|arr| {
        let mut c_counter = 0;
        let mut lp_counter = 0;
        let mut rp_counter = 0;

        arr.into_iter().for_each(|c| {
            if *c == '(' {
                rp_counter += 1;
            } else if *c == ',' {
                c_counter += 1;
            } else if *c == ')' {
                lp_counter += 1;
            }
        });
        rp_counter == 1 && c_counter == 1 && lp_counter == 1
    });
    let mut operations = Vec::new();

    for arr in verified_mul.into_iter() {
        for i in 1..(arr.len() - 1) {
            if arr[i] == ',' {
                let first_num = arr[1..i]
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>();
                let first_num: u32 = first_num
                    .concat()
                    .parse()
                    .expect("error parse string to u32");

                let second_num = arr[(i + 1)..(arr.len() - 1)]
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>();
                let second_num: u32 = second_num
                    .concat()
                    .parse()
                    .expect("error parse string to u32");

                operations.push((first_num, second_num));
            }
        }
    }
    let mut sum = 0;
    for operation in operations.into_iter() {
        let num_1 = operation.0;
        let num_2 = operation.1;
        sum += num_1 * num_2;
        println!("{} {}", num_1, num_2);
    }

    println!("{}", sum);
}

pub fn p1_v2() {
    let file = std::fs::read_to_string("file").unwrap();

    let mul_regex = Regex::new(r###"mul\(\d{1,3}\,\d{1,3}\)"###).unwrap();

    let mul_matches = mul_regex
        .find_iter(&file)
        .map(|mul| mul.as_str())
        .collect::<Vec<&str>>();

    let mut verified_mul = Vec::new();
    for mul in mul_matches.into_iter() {
        let mul_char = mul.chars().into_iter().collect::<Vec<_>>();

        for i in 4..(mul_char.len() - 1) {
            //print!("{}", mul_char[i]);
            if mul_char[i] == ',' {
                let first_num: u32 = mul_char[4..i]
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .concat()
                    .parse()
                    .unwrap();
                let second_num: u32 = mul_char[(i + 1)..mul_char.len() - 1]
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .concat()
                    .parse()
                    .unwrap();

                verified_mul.push((first_num, second_num))
            }
        }
    }
    let mut sum = 0;
    for m in verified_mul.into_iter() {
        sum += m.0 * m.1;
    }
    println!("{sum}");
}
