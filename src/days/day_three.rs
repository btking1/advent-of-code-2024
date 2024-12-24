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

    verified_mul.into_iter().for_each(|arr| {
        let s = arr
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        println!("{}", s.concat())
    });
}
