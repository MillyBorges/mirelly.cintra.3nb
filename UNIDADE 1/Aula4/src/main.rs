use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    let a = &data;
    {
        let mut borrowed = data.borrow_mut(); // Empréstimo mutável em tempo de execução
        *borrowed += 1;
        
    }
    println!("Valores: {} e {}", data.borrow(), b);
}



