use std::thread;

fn main() {
    let (mut n1, mut n2) = (false, false);
    let mut vez: char = '1'; 

    let t1 = thread::spawn(move || {
        loop {
            n1 = true;
            vez = '2';
            while n2 && vez == '1' {}
            critic_region('1');
            n1 = false;
            thread::sleep(std::time::Duration::from_millis(1000));
            println!("Finished processing the {}", vez);
        }
    });

    let t2 = thread::spawn(move || {
        loop {
            n2 = true;
            vez = '1';
            while n1 && vez == '2' {}
            critic_region('2');
            n2 = false;
            thread::sleep(std::time::Duration::from_millis(1000));
            println!("Finished processing the {}", vez);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
}

fn critic_region(vez: char){
    println!("Thread {} access critic region", vez);
}