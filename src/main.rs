use std::io::Read;

fn main() {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source).unwrap();
    let ast = aleparser::parse(source);
    let response = pythongen::generate(ast);
    
    println!("{}", response);
}
