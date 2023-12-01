use std::fs;

fn main() {
    day1();
}

fn day1() {
    let content = fs::read_to_string("input1.txt").expect("Unable to read the input file");

    let mut calibrations: Vec<u32> = vec![];

    let model = vec![
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    for line in content.lines() {
        let mut cal_vec = vec![(usize::MAX, 'x'), (usize::MIN, 'x')];
        for pat in &model {
            if let Some(index) = line.find(pat.0) {
                if index < cal_vec[0].0 {
                    cal_vec[0].0 = index;
                    cal_vec[0].1 = pat.1;
                }
            }
            if let Some(index) = line.rfind(pat.0) {
                if index >= cal_vec[1].0 {
                    cal_vec[1].0 = index;
                    cal_vec[1].1 = pat.1;
                }
            }
        }
        let calibration: String = cal_vec.iter().map(|(_i, c)| c).collect();
        calibrations.push(calibration.parse().unwrap());
    }

    let sum: u32 = calibrations.iter().sum();
    println!("{sum}")
}
