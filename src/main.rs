use std::io::Read;

fn main() {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source).unwrap();
    let mut ast = aleparser::parse(source);

    ast = constantfolding::transform(ast);
    
    let response = pythongen::generate(ast);
    
    println!("{}", response);
}
