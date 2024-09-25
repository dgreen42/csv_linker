use std::{
    env::{args, set_var},
    fs::read,
    io::{stdout, BufRead, Write},
};

fn main() {
    set_var("RUST_BACKTRACE", "1");
    let file = args().nth(1).expect("Enter valid path");
    if file == "--help" {
        println!("

csv_linker v1.0.0

csv_linker [file]

Reads a csv and outputs the first column as standard output. For now this is used in the genetics pipeline in order to read the csv output of the first program in the pipe. This can them be used to output the fields from the .csv into the next step in the pipe. 

Use cases:

    - kbrecondo into csv_linker into mokuba
    - a data set with gene ID's as the first column into csv_linker into mokuba

");
    }
    let reader = read(file).unwrap();
    let mut count = 1;
    for line in reader.lines() {
        let line = line.unwrap();
        if count == 1 {
            let mut header = String::from("#");
            header.push_str(line.split(",").next().unwrap());
            stdout()
                .write_all(format!("{}\n", header).as_bytes())
                .unwrap();
        } else {
            let first_column = line.split(",").next().unwrap();
            stdout()
                .write_all(format!("{}\n", first_column).as_bytes())
                .unwrap();
        }
        count += 1;
    }
}
