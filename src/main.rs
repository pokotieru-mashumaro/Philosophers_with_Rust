use chrono::{DateTime, Utc};
use std::env;
use std::process;

struct Philo {
    id: u32,
    left_fork_id: usize,
    left_right_id: usize,
    eat_count: u32,
    last_eat_time: DateTime<Utc>,
    configration: Configration,
}

impl Philo {
    fn eat(&self) {
        // thread::sleep_ms(1000);thread::sleep_ms(1000);
        println!("{} {} is eating", 20240827, self.id)
    }

    fn sleep(&self) {
        println!("{} {} is sleeping", 20240827, self.id)
    }

    fn think(&self) {
        println!("{} {} is thinking", 20240827, self.id)
    }

}

#[derive(Debug)] 
struct Configration {
    number_of_philosophers: u32,
    time_to_die: u32,
    time_to_eat: u32,
    time_to_sleep: u32,
    number_of_times_each_philosopher_must_eat: Option<u32>,
    is_dead: bool,
    start_eat_time: Option<DateTime<Utc>>,
}

impl Configration {
    fn new(av: &[String]) -> Result<Self, &'static str> {
        if av.len() != 5 && av.len() != 6 {
            return Err("arg is bad!");
        }
        let number_of_philosophers = av[1].parse::<u32>().map_err(|_| "Invalid number of philosophers")?;
        let time_to_die = av[2].parse::<u32>().map_err(|_| "Invalid time to die")?;
        let time_to_eat = av[3].parse::<u32>().map_err(|_| "Invalid time to eat")?;
        let time_to_sleep = av[4].parse::<u32>().map_err(|_| "Invalid time to sleep")?;
        let number_of_times_each_philosopher_must_eat = if av.len() == 6 {
            Some(av[5].parse::<u32>().map_err(|_| "Invalid number of times to eat")?)
        } else {
            None
        };
    
        Ok(Self {
            number_of_philosophers,
            time_to_die,
            time_to_eat,
            time_to_sleep,
            number_of_times_each_philosopher_must_eat,
            is_dead: false,
            start_eat_time: None,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut conf  = Configration::new(&args).unwrap_or_else(|err| {
        println!("err = {}", err);
        process::exit(0);
    });
    println!("conf = {:?}", conf);

}
