use std::env;
use std::fs;
use std::fs::File;
use std::io::Write as _;
use chrono::Local;

const BIN_PATH: &str = "./src/bin";
const MG_TEMPLATEPATH: &str = "./src/templates/mg.rs";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut kernel = &args[1];
    let mut class_npb = &args[2];
    let binding = kernel.to_uppercase();
    kernel = &binding;
    let binding2 = class_npb.to_uppercase();
    class_npb = &binding2;

    if kernel == "MG" {
        write_mg_info(class_npb.as_str());
    }
}

fn write_ep_info(class_npb: &str) {
    
}

fn write_ft_info(class_npb: &str) {

}

fn write_cg_info(class_npb: &str) {

}

fn write_mg_info(class_npb: &str) {
    let mut binding = fs::read_to_string(&MG_TEMPLATEPATH).expect("File");
    let mut contents: &str = binding.as_mut_str();

    let problem_size = match class_npb {
        "S" => "32",
        "W" => "128",
        "A" => "256",
        "B" => "256",
        "C" => "512",
        "D" => "1024",
        "E" => "2048",
        _   => "32"
    };

    let nit = match class_npb {
        "S" => "4",
        "W" => "4",
        "A" => "4",
        "B" => "20",
        "C" => "20",
        "D" => "50",
        "E" => "50",
        _   => "4"
    };

    let compile_time = Local::now().to_rfc3339();

    binding = contents.replace("%% PROBLEM_SIZE %%", problem_size);
    contents = binding.as_mut_str();
    binding = contents.replace("%% NIT %%", nit);
    contents = binding.as_mut_str();
    binding = contents.replace("%% COMPILE_TIME %%", format!("\"{}\"", compile_time).as_str());
    contents = binding.as_mut_str();

    println!("teste : {}", &contents);

    let mut bin_file = File::create(format!("{}/mg-s.rs", &BIN_PATH)).unwrap();
    //write!(bin_file, "{}", &contents);
    bin_file.write_all(&contents.as_bytes());
}