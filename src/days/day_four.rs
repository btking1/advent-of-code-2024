pub fn p1() {
    let file = std::fs::read("/Users/godsiom/coding/rust/advent-of-code-2024/data/wordsearch.txt")
        .unwrap();
    let sfile = std::fs::read_to_string("file").unwrap();

    //println!("{:?}", sfile);

    let mut new_lines_pos = Vec::new();
    //let mut liness = Vec::new();
    let mut counter = 0;
    println!("{:?}", char::from(file[&file.len() - 1]));
    println!("{:?}", char::from(file[&file.len() - 2]));
    println!("{:?}", char::from(file[file.len()]));
    for i in 0..file.len() - 1 {
        let c = char::from(file[i]);
        if c == '\n' {
            new_lines_pos.push(i);
            counter += 1;
        }
    }

    //for lines in lines.into_iter() {
    //  let lines = line
    //    .into_iter()
    //  .map(|byte| char::from(byte.to_owned()).to_string())
    //.collect::<Vec<String>>();
    //println!("{:?}", lines.concat());
    //}
}

pub fn p1_v2() {
    let file = std::fs::read("file").unwrap();
    let lines = file
        .split(|byte| *byte == b"\n"[0])
        .map(|arr| {
            arr.into_iter()
                .map(|byte| byte.to_owned())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let lines = &mut lines
        .into_iter()
        .take_while(|line| line.len() > 0)
        .map(|arr| {
            arr.into_iter()
                .map(|byte| char::from(byte.to_owned()))
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let mut xmas_counter = 0;
    let xmas = ['X', 'M', 'A', 'S'];
    let mut r_xmas = xmas.clone();
    r_xmas.reverse();

    for y in 0..lines.len() {
        let mut arr = Vec::new();
        if y + 3 <= lines.len() - 1 {
            for x in 0..lines[y].len() {
                let mut v_slice = [' '; 4];
                v_slice = [
                    lines[y][x],
                    lines[y + 1][x],
                    lines[y + 2][x],
                    lines[y + 3][x],
                ];

                if v_slice == xmas || v_slice == r_xmas {
                    xmas_counter += 1;
                    arr.push(v_slice);
                }
            }
        }
    }

    for (y, line) in lines.clone().into_iter().enumerate() {
        line.clone().into_iter().enumerate().for_each(|(i, byte)| {
            let mut h_slice = [' '; 4];

            if i + 3 <= &line.len() - 1 {
                h_slice = [byte, line[i + 1], line[i + 2], line[i + 3]];
            }
            if h_slice == xmas || h_slice == r_xmas {
                //println!("{y}: {:?}", h_slice);
                xmas_counter += 1;
            }
        });
    }
    println!("{xmas_counter}")
}
