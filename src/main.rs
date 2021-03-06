use rug::Float;
use statrs::statistics::Statistics;
use std::io::Write;
use std::fs::File;

fn get_string(the_base: i32, root_it: i32) -> String {
    let anum = Float::with_val(60000, root_it);
    let sqroot = anum.sqrt();
    let the_string = sqroot.to_string_radix(the_base, None);
    the_string
}

fn get_stats(in_vec: Vec<i32>) -> String {
    let mut float_vec: Vec<f64> = Vec::new();
    for a in in_vec {
        let float_a: f64 = a as f64;
        float_vec.push(float_a);
    }
    let the_min = Statistics::min(&mut float_vec);
    let the_max = Statistics::max(&mut float_vec);
    let the_mean = Statistics::mean(&mut float_vec);
    let the_std_dev = Statistics::std_dev(&mut float_vec);

    let ret_string = format!("Min {}, Max {}, Meam {}, Std Dev {}",
     the_min, the_max, the_mean, the_std_dev);
    ret_string
}

fn main() {
    let mut the_file = File::create("data.txt").expect("no file");
    let skip_this = vec![4,9,16,25,36,49,64,81,100,121,144,169,196,225];
    for num_num in 2..256 {

        if skip_this.contains(&num_num) { continue; }

        writeln!(&mut the_file, "\t\t**Sq Root {}**", &num_num).unwrap();

        for num_base in 2..=36 {
            let my_string = get_string(num_base, num_num);
            let mut avec = vec![0; num_base.try_into().unwrap()];
            for c in my_string.chars() {
                if c == '.' { continue; }
                let pos = match c {
                    '0' =>  0,
                    '1' =>  1,
                    '2' =>  2,
                    '3' =>  3,
                    '4' =>  4,
                    '5' =>  5,
                    '6' =>  6,
                    '7' =>  7,
                    '8' =>  8,
                    '9' =>  9,
                    'a' =>  10,
                    'b' =>  11,
                    'c' =>  12,
                    'd' =>  13,
                    'e' =>  14,
                    'f' =>  15,
                    'g' =>  16,
                    'h' =>  17,
                    'i' =>  18,
                    'j' =>  19,
                    'k' =>  20,
                    'l' =>  21,
                    'm' =>  22,
                    'n' =>  23,
                    'o' =>  24,
                    'p' =>  25,
                    'q' =>  26,
                    'r' =>  27,
                    's' =>  28,
                    't' =>  29,
                    'u' =>  30,
                    'v' =>  31,
                    'w' =>  32,
                    'x' =>  33,
                    'y' =>  34,
                    'z' =>  35,
                     _  => panic!(),
                };
                avec[pos] = avec[pos] + 1;
            }

            writeln!(&mut the_file, "{:?}", &avec).unwrap();
            let print_stats = get_stats(avec);
            writeln!(&mut the_file, "{}", print_stats).unwrap();
        }
    }

}

