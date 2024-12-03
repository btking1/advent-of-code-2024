use std::cell::RefCell;

pub struct Cols(Vec<u32>, Vec<u32>);

pub fn day_one_p1() -> Cols {
    let s_file = std::fs::read_to_string("path").expect("file doesnt exist");
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
        //println!("index: {i} distance: {dist}");
    }
    //println!("total: {total}");

    Cols(l_col, r_col)
}

#[derive(Debug)]
struct ColInfo {
    number: u32,
    count: RefCell<u32>,
    sim_score: RefCell<u32>,
}
pub fn day_one_p2() {
    let cols = day_one_p1();
    let mut col_info_vec: Vec<std::sync::Arc<ColInfo>> = Vec::new();
    let lc = cols.0;
    let rc = cols.1;

    for x in lc.into_iter() {
        let col_info = std::sync::Arc::new(ColInfo {
            number: x,
            count: RefCell::new(1),
            sim_score: RefCell::new(0),
        });

        for y in rc.clone().into_iter() {
            if x == y {
                col_info.count.replace_with(|&mut old| old + 1);
            }
        }

        let count = col_info.count.clone().take();
        col_info.sim_score.replace(col_info.number * (count - 1));
        if count > 1 {
            col_info_vec.push(col_info.clone());
        }
    }

    let mut total = 0;
    for entry in col_info_vec.into_iter() {
        let sim_score = entry.sim_score.take();
        total += sim_score;
        println!("{:?}", entry);
    }
    println!("sim total: {total}");
}
