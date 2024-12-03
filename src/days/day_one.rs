pub fn day_one() {
    let s_file = std::fs::read_to_string(
        "/Users/godsiom/coding/rust/advent-of-code-2024/data/location_id.txt",
    )
    .expect("file doesnt exist");
    // space = 32 new line = 10

    let filtered_s_file: Vec<&str> = s_file.split(char::is_whitespace).collect();

    let mut f_list = Vec::new();
    let mut l_col = Vec::new();
    let mut r_col = Vec::new();

    let mut counter: usize = 0;

    for string in filtered_s_file.iter() {
        if *string != "" {
            f_list.push(string);
        }
    }
    for num in f_list.iter() {
        let int: u32 = num.parse().unwrap();
        counter += 1;

        if counter % 2 != 0 {
            l_col.push(int);
        } else {
            r_col.push(int);
        }
    }

    l_col.sort();
    r_col.sort();
    println!("{}", l_col.len());

    let mut distances: Vec<u32> = Vec::new();

    let mut l_counter = 0;
    let mut r_counter = 0;

    for i in 0..l_col.len() {
        if l_col.get(i).is_some() && r_col.get(i).is_some() {
            let l_num = l_col.get(i).unwrap();
            let r_num = r_col.get(i).unwrap();

            if l_num >= r_num {
                l_counter += 1;
                let d = l_num - r_num;
                //println!("l: {l_num} r: {r_num} l - r: {d}");
                distances.push(d);
            } else {
                r_counter += 1;
                let d = r_num - l_num;
                //println!("l: {l_num} r: {r_num} r - l: {d}");
                distances.push(d);
            }
        }
    }

    let mut total = 0;
    for (i, dist) in distances.into_iter().enumerate() {
        total += dist;
        println!("index: {i} distance: {dist}");
    }
    println!("total: {total}");
    //println!("l num bigger: {l_counter}, r num bigger: {r_counter}");
}
