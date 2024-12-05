use std::fs::read_to_string;

pub fn p1() {
    let reports_file = read_to_string("path").expect("failed to read file");

    let reports = reports_file.lines();

    let mut report_count = 0;

    let mut filtered_reports = Vec::new();
    for report in reports.into_iter() {
        let f_report: Vec<&str> = report.split(char::is_whitespace).collect();

        filtered_reports.push(f_report);
    }

    let mut f_report = (&filtered_reports).into_iter();

    while let Some(levels) = f_report.next() {
        report_count += 1;
        let mut inc_counter = 0;
        let mut dec_counter = 0;

        levels.into_iter().enumerate().for_each(|(pos, level)| {
            if pos != levels.len() - 1 {
                let curr: usize = levels[pos].parse().expect("parse error");

                let next: usize = levels[pos + 1].parse().expect("parse error");
                //println!("{report_count} | length {}, next {}", levels.len(), pos + 1);
                if next > curr {
                    inc_counter += 1;
                    println!("INC | {report_count} | current: {curr} next: {next}");
                } else {
                    dec_counter += 1;
                    println!("DEC | {report_count} | current: {curr} next: {next}");
                }
            } else {
                let curr: usize = levels[pos].parse().expect("parse error");

                let prev: usize = levels[pos - 1].parse().expect("parse error");

                if curr > prev {
                    inc_counter += 1;

                    println!(
                        "INC | {report_count} | current: {curr} prev
: {prev}"
                    );
                } else {
                    dec_counter += 1;

                    println!("DEC | {report_count} | current: {curr} prev: {prev}");
                }
            }
        });

        println!(
            "\n{} | inc: {} dec: {} length: {}\n",
            report_count,
            inc_counter,
            dec_counter,
            levels.len()
        );
    }
}
