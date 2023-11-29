use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Cria um Mutex compartilhado entre threads
    let number = Arc::new(Mutex::new(0));

    let mut threads = vec![];

    (0..10).for_each(|i| {
        // Without cloning the Arc, each thread would have its own copy of the Mutex, and they wouldn't be able to share ownership of the underlying data
        let number = Arc::clone(&number);
    
        threads.push(thread::spawn(move || {
            *number.lock().unwrap() += i;
            println!("Number {}  in iteration {}", *number.lock().unwrap(), i);
        }));
    });

    for handle in threads {
        handle.join().unwrap();
    }
    
    println!("Number {}", number.lock().unwrap());
}

fn counter(){
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        // Cria uma cópia do Mutex para cada thread
        let counter = Arc::clone(&counter);

        // Cria uma nova thread
        let handle = thread::spawn(move || {
            // Bloqueia o Mutex para acessar ou modificar o valor interno
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // Aguarda que todas as threads terminem
    for handle in handles {
        handle.join().unwrap();
    }

    // Imprime o resultado final
    println!("Resultado final: {}", *counter.lock().unwrap());
}