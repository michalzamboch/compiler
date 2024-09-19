use lexer::scanner::Scanner;

mod lexer;
mod types;

fn main() {
    let input = r"
        int x = 10;
        float y = 3.14f;
        boolean z = true;
        class MyClass {
            void myMethod() {
                print('Hello, world!');
            }
        }
    ";

    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    for token in tokens.iter() {
        println!("{}", token);
    }
}
