//CHAT-GPT Version of catch_or_not and die_or_not
use rand::Rng;

// Define the Status enum
#[derive(Copy, Clone, Debug, PartialEq)]
enum Status {
    Susceptible,
    Infected(u32), // Tracks number of days infected
    Recovered,
    Vaccinated,
    Dead,
}

//TODO: Restructure catch_or_not and die_or_not to be more testable

// Function to simulate if a person gets infected
fn catch_or_not_old(tprob: f64, num_people: usize, status_vec: &[Status]) -> bool {
    let mut rng = rand::thread_rng();
    let mut got_infected = false;

    let num_exposures = get_rand_num_exposures(num_people);

    for _ in 0..num_exposures {
        let rand_person = get_living_person(status_vec);
        if let Status::Infected(_) = rand_person {
            if rng.gen::<f64>() < tprob {
                got_infected = true;
                break;
            }
        }
    }

    got_infected
}

//NEW CATCH_OR_NOT
fn catch_or_not(tprob: f64, mut person: Status, encounters_vec: &[Status]) -> Status {
    let mut rng = rand::thread_rng();

    for encounter in encounters_vec {
        let rand_num: f64 = rng.gen();  // Generates a random number between 0.0 and 1.0

        if tprob < rand_num {
            person = Status::Infected(1);
        }
    }

    person
}


// Function to simulate if a person dies, recovers, or stays infected
fn die_or_not_old(
    dprob: f64, 
    sick_days: u32, 
    person: usize, 
    status_vec: &[Status]
) -> Status {
    let mut rng = rand::thread_rng();
    let mut output_status = status_vec[person];

    if rng.gen::<f64>() < dprob {
        output_status = Status::Dead;
    } else if let Status::Infected(days) = output_status {
        if days >= sick_days {
            output_status = Status::Recovered;
        } else {
            output_status = Status::Infected(days + 1);
        }
    }

    output_status
}

//NEW DIE_OR_NOT
fn die_or_not(dprob: f64, sick_days: u32, mut person: Status) -> Status {
    let mut rng = rand::thread_rng();

    let rand_num: f64 = rng.gen::<f64>();

    match person { //TODO: look more into match keyword
        Status::Infected(days) => {
            if rand_num < dprob {
                person = Status::Dead;
            } else {
                if days >= sick_days {
                    person = Status::Recovered;
                } else {
                    person = Status::Infected(days + 1);
                }
            }
        }
        _ => {}  // Do nothing if the person is not infected
    }

    person
}

// Helper function to get a random number of exposures (just for simulation purposes)
fn get_rand_num_exposures(num_people: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..num_people)
}

// Helper function to randomly pick a living person from the status vector
fn get_living_person(status_vec: &[Status]) -> Status {
    let mut rng = rand::thread_rng();
    loop {
        let rand_index = rng.gen_range(0..status_vec.len());
        let status = status_vec[rand_index];
        if status != Status::Dead {
            return status;
        }
    }
}


// fn main() {
//     // Example usage:
//     // let status_vec = vec![Status::Susceptible; 10000];
//     // let infected = catch_or_not(0.5, 10000, &status_vec);
//     // println!("Infected: {}", infected);
//     println!("Hello World");
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catch_or_not_infected_in_group() {
        // Test with one infected person in the group
        let status_vec = vec![
            Status::Susceptible, 
            Status::Infected(2), //TODO: change Infected to more specific name to represent count
            Status::Vaccinated
        ];
        let result = catch_or_not(0.5, status_vec.len(), &status_vec);
        assert!(result, "Expected infection when infected person is present.");
    }

    #[test]
    fn test_catch_or_not_no_infected_in_group() {
        // Test when no one in the group is infected
        let status_vec = vec![
            Status::Susceptible, 
            Status::Recovered, 
            Status::Vaccinated
        ];
        let result = catch_or_not(0.5, status_vec.len(), &status_vec);
        assert!(!result, "Did not expect infection when no one is infected.");
    }

    #[test]
    fn test_catch_or_not_tprob_zero() {
        // Test with transmission probability of 0 (no one should get infected)
        let status_vec = vec![
            Status::Susceptible, 
            Status::Infected(3), 
            Status::Vaccinated
        ];
        let result = catch_or_not(0.0, status_vec.len(), &status_vec);
        assert!(!result, "Expected no infection with transmission probability of 0.");
    }

    #[test]
    fn test_catch_or_not_tprob_one() {
        // Test with transmission probability of 1.0 (everyone should get infected if exposed)
        let status_vec = vec![
            Status::Susceptible, 
            Status::Infected(1), 
            Status::Vaccinated
        ];
        let result = catch_or_not(1.0, status_vec.len(), &status_vec);
        assert!(result, "Expected infection with transmission probability of 1.0.");
    }

    #[test]
    fn test_die_or_not_death_occurs() {
        // Test if person dies when death probability is high
        let status_vec = vec![
            Status::Infected(3), 
            Status::Recovered, 
            Status::Vaccinated
        ];
        let result = die_or_not(1.0, 5, 0, &status_vec);
        assert_eq!(result, Status::Dead, "Expected person to die with high death probability.");
    }

    #[test]
    fn test_die_or_not_person_recovers() {
        // Test if person recovers after reaching sick days limit
        let status_vec = vec![
            Status::Infected(5), 
            Status::Susceptible
        ];
        let result = die_or_not(0.0, 5, 0, &status_vec);
        assert_eq!(result, Status::Recovered, "Expected person to recover after 5 sick days.");
    }

    #[test]
    fn test_die_or_not_person_stays_infected() {
        // Test if person stays infected if they haven't been sick long enough
        let status_vec = vec![
            Status::Infected(2), 
            Status::Susceptible
        ];
        let result = die_or_not(0.0, 5, 0, &status_vec);
        assert_eq!(result, Status::Infected(3), "Expected person to stay infected with incremented days.");
    }

    #[test]
    fn test_die_or_not_dead_stays_dead() {
        // Test if dead person stays dead
        let status_vec = vec![
            Status::Dead, 
            Status::Susceptible
        ];
        let result = die_or_not(0.0, 5, 0, &status_vec);
        assert_eq!(result, Status::Dead, "Expected dead person to remain dead.");
    }
}



// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_empty() {
//         //ARRANGE
//         //let val = 5;

//         //ACT
//         //let even_vec: Vec<i32> = find_even(val);

//         //ASSERT
//         //assert_eq!();

//         //assert
//     }

// }