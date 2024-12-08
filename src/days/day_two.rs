use std::{fs::read_to_string, ops::Index};

pub fn p1() {
    let reports_file = read_to_string("path").expect("failed to read file");

    let reports = reports_file
        .lines()
        .into_iter()
        .map(|report| {
            report
                .split(" ")
                .into_iter()
                .map(|level| level.parse::<u16>().unwrap())
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<_>>();

    let dec_reports = reports
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, levels)| {
            levels
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, level)| -> std::cmp::Ordering {
                    if i != 0 {
                        level.cmp(&levels[i - 1])
                    } else {
                        levels[i + 1].cmp(&level)
                    }
                })
                .all(|level| level == std::cmp::Ordering::Greater)
        })
        .collect::<Vec<(usize, Vec<u16>)>>();

    let inc_reports = reports
        .into_iter()
        .enumerate()
        .filter(|(_, levels)| {
            levels
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, level)| -> std::cmp::Ordering {
                    if i != 0 {
                        level.cmp(&levels[i - 1])
                    } else {
                        levels[i + 1].cmp(&level)
                    }
                })
                .all(|level| level == std::cmp::Ordering::Less)
        })
        .collect::<Vec<(usize, Vec<u16>)>>();
    let mut inc_or_dec = Vec::new();

    for report in inc_reports.clone().into_iter() {
        inc_or_dec.push(report.1);
    }
    for report in dec_reports.clone().into_iter() {
        inc_or_dec.push(report.1);
    }

    /*
    for report in stable_reports(inc_or_dec).into_iter() {
            println!("{:?}", report.1)
        }
    */

    println!("{}", stable_reports(inc_or_dec).len());
}

fn stable_reports(reports: Vec<Vec<u16>>) -> Vec<(usize, Vec<u16>)> {
    reports
        .into_iter()
        .enumerate()
        .filter(|(_, levels)| {
            levels.clone().into_iter().enumerate().all(|(j, level)| {
                if j != 0 {
                    let next = levels.get(j - 1).unwrap();
                    let diff = level.abs_diff(*next);
                    diff >= 1 && diff <= 3
                } else {
                    j == 0
                }
            })
        })
        .collect::<Vec<_>>()
}
