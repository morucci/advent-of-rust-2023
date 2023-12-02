use std::{collections::HashMap, fs};

fn main() {
    // day1();
    day2();
}

fn load_file(input_name: &str) -> String {
    fs::read_to_string(input_name).expect("Unable to read the input file")
}

fn day2() {
    let content = load_file("input2.txt");
    let model = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut possible_game_sum: u32 = 0;

    for line in content.lines() {
        let mut game_ok = true;
        println!("{line}");
        let s1: Vec<&str> = line.split(':').collect();
        let game_id: u32 = {
            let v: Vec<&str> = s1[0].split(' ').collect();
            v[1].trim().parse().unwrap()
        };
        let sets: Vec<&str> = s1[1].split(';').collect();

        for set in sets {
            println!("set: {:?}", set);
            let cubes_s: Vec<&str> = set.split(',').collect();
            for cube_s in cubes_s {
                let cube: Vec<&str> = cube_s.trim().split(' ').collect();
                let color = cube[1].trim();
                let count: u32 = cube[0].trim().parse().unwrap();
                println!("game_id: {game_id} -> color: {color}: {count}");

                // Check cube with model
                if let Some(m_count) = model.get(color) {
                    if count > *m_count {
                        // color count is above model's color
                        game_ok = false;
                    }
                } else {
                    // Color does not exist in the model
                    game_ok = false;
                }
            }
        }

        if !game_ok {
            println!("This game is not possible");
        } else {
            println!("This game is possible");
            possible_game_sum = possible_game_sum + game_id;
        };
    }

    println!("result: {possible_game_sum}");
}

fn _day1() {
    let content = load_file("input1.txt");

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
