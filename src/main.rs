fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} <file1> <file2>", args[0]);
        std::process::exit(1);
    }
    let file1 = std::fs::read_to_string(&args[1]).unwrap();
    let file2 = std::fs::read_to_string(&args[2]).unwrap();
    for (i, (line1, line2)) in file1.lines().zip(file2.lines()).enumerate() {
        let line1 = line1.replace("0.0\t", "0\t");
        let line2 = line2.replace("0.0\t", "0\t");
        if line1.trim() != line2.trim() {
            println!("Line {}", i + 1);
            println!("{}", line1);
            println!("{}", line2);
            println!();
        }
    }
}
