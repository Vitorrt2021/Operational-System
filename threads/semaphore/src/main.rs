use std::thread;

fn main() {
    let mut semaphore: u8 = 1;

    let t1 = thread::spawn(move || {
        loop {
            down(semaphore);
            critic_region('1');
            up(semaphore);

            thread::sleep(std::time::Duration::from_millis(2000));
            println!("Finished processing the thread 1");
        }
    });

    let t2 = thread::spawn(move || {
        loop {
            down(semaphore);
            critic_region('2');
            up(semaphore);

            thread::sleep(std::time::Duration::from_millis(1000));
            println!("Finished processing the thread 2");
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

fn critic_region(thread: char){
    println!("Thread {} access critic region", thread);
}

fn down(mut semaphore: u8){
    if(semaphore != 0) {
        semaphore = semaphore - 1;
    }
}

fn up(mut semaphore: u8){
    semaphore = semaphore + 1;
}