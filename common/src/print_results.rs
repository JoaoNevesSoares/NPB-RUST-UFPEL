pub fn rust_print_results(
    name: &str,
    class_npb: &str,
    n1: u32,
    n2: i32,
    n3: i32,
    niter: i32,
    t: f64,
    mops: f64,
    optype: &str,
    passed_verification: bool,
    npbversion: &str,
    compiletime: &str,
    compilerversion: &str,
    libversion: &str,
    totalthreads: &str,
    rust: &str,
    rust_link: &str,
    rust_lib: &str,
    rust_inc: &str,
    rust_flags: &str,
    rust_link_flags: &str,
    rand: &str,
) {
    println!("\n\n {} Benchmark Completed", name);
    println!(" class_npb       =                        {}", class_npb);
    match name {
        "IS" => {
            if n3 == 0 {
                let mut nn: u32 = n1;
                if n2 != 0 {
                    nn *= n2 as u32;
                }
                println!(" Size            =             {:12}", nn); // as in IS
            } else {
                println!(
                    " Size            =             {:04x}{:04x}{:04x}",
                    n1, n2, n3
                );
            }
        }
        _ => {
            if (n2 == 0) && (n3 == 0) {
                if name == "EP" {
                    //macro format! is being used to format a `double` value `pow(2.0,n1)` into a stirng with a field
                    //width of 15 characters and 0 decimal places, and store the result in the `size` buffer
                    let size = format!("{:0>15}", f64::powi(2.0, n1 as i32));
                    println!(" Size            =          {}", size);
                } else {
                    println!(" Size            =             {:12}", n1);
                }
            } else {
                println!(
                    " Size            =             {:04x}{:04x}{:04x}",
                    n1, n2, n3
                )
            }
        }
    }
    println!(
        " Total threads   =             {:>12}",totalthreads);
    println!(" Iterations      =             {:>12}", niter);
    println!(" Time in seconds =             {}", t);//:12.2
    println!(" Mop/s total     =             {:12.2}", mops);
    println!(" Operation type  = {:>24}", optype);
    //if passed_verification < 0 {
    //println!(" Verification    =            NOT PERFORMED");
    //}
    if passed_verification {
        println!(" Verification    =            SUCCESSFUL");
    } else {
        println!(" Verification    =            UNSUCCESSFUL");
    }
    println!(
        " Version         =             {:>12}", npbversion);
    println!(
        " Compile date    =             {:>12}",compiletime);
    println!(
        " Compiler ver    =             {:>12}", compilerversion);
    println!(
        " OpenMP version  =             {:>12}", libversion);
    println!("\n Compile options:\n");
    println!("    RUSTC           = {}", rust);
    println!("    RUST_LINK       = {}", rust_link);
    println!("    RUST_LIB        = {}", rust_lib);
    println!("    RUST_INC        = {}", rust_inc);
    println!("    RUST_FLAGS      = {}", rust_flags);
    println!("    RUST_LINK_FLAGS = {}", rust_link_flags);
    println!("    RAND            = {}", rand);
}
