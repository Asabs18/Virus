use std::fs::File;
use std::io::{self, BufWriter, Write};

//MVC traits
trait Model<T> {
    fn get_data(&self) -> &T;
    fn set_data(&mut self, data: T);
}
trait View<T> {
    fn print(&mut self) -> io::Result<()>;
}

trait Controller<T> {
    fn run(&mut self) -> bool; // Return bool indicating whether to continue
}

//STATUS

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Susceptible,
    Recovered,
    Vaccinated,
    Dead,
    Infected(u32), // Number of days infected
}

//COMMAND LINE ARGUMENTS
#[derive(Clone)]
struct CLA {
    num_simulations: u32,
    num_people: u32,
    num_days: u32,
    infection_prob: f64,
    vaccination_prob: f64,
    transmission_prob: f64,
    death_prob: f64,
    output_file: String,
}

//PERSON
#[derive(Debug, Clone, PartialEq, Eq)]
struct PersonModel {
    id: u32, // Unique ID
    status: Status,
}

impl PersonModel {
    // Create a new person with a unique ID and initial status
    fn new(id: u32) -> Self {
        Self {
            id,
            status: Status::Susceptible,
        }
    }

    fn get_status(&self) -> Status {
        self.status
    }

    fn to_string(&self) -> String {
        format!("Person ID: {}, Status: {:?}", self.id, self.status)
    }
}

impl Model<PersonModel> for PersonModel {
    fn set_data(&mut self, person: PersonModel) {
        self.status = person.status;
    }

    fn get_data(&self) -> &PersonModel {
        &self
    }
}

//POPULATION
#[derive(Clone)]
struct PopulationModel {
    people: Vec<PersonModel>,
    command_line_args: CLA,
}

impl PopulationModel {
    fn new(command_line_args: CLA) -> Self {
        Self {
            people: Vec::new(),
            command_line_args,
        }
    }

    fn populate(&mut self) -> bool {
        for i in 0..self.command_line_args.num_people {
            let person = PersonModel::new(i);
            self.people.push(person);
        }
        true
    }
}

impl Model<PopulationModel> for PopulationModel {
    fn set_data(&mut self, population: PopulationModel) {
        self.people = population.people;
    }
    fn get_data(&self) -> &PopulationModel {
        &self
    }
}

//POPULATION CONTROLLER

struct PopulationController {
    population: PopulationModel,
}

impl PopulationController {
    fn new(population: PopulationModel) -> Self {
        Self { population }
    }
}

//Should run one full day
impl Controller<PopulationModel> for PopulationController {
    fn run(&mut self) -> bool {
        /*
        overwrite current state with next day state if there are still
        days to simulate
         */
        true
    }
}

//SIMULATION CONTROLLER
struct SimulationController {
    simulations: Vec<PopulationController>, //Maybe change to a vector of population controllers
    output_view: OutputView,
    curr_sim_id: u32,
}

impl SimulationController {
    fn new(simulations: Vec<PopulationController>, output_view: OutputView) -> Self {
        Self {
            simulations,
            output_view,
            curr_sim_id: 0,
        }
    }

    fn update(&mut self) {
        // Update the simulation model
        /*
        if day = 0, output default state
        if curr_sim_id not at max num days, call its run to update to next day
        output next day state
        if curr_sim_id at max num days, curr_sim_id+=1
         */
    }
}

//Should run one full simulation
impl Controller<Vec<PopulationController>> for SimulationController {
    fn run(&mut self) -> bool {
        for i in 0..self.simulations.len() {
            for _j in 0..self.simulations[i].population.command_line_args.num_days {
                self.update();
            }
        }
        true
    }
}

//OUTPUT
struct OutputView {
    population: PopulationModel,
    output_file: File,
}

impl OutputView {
    fn new(population: PopulationModel, output_file_name: &str) -> io::Result<Self> {
        let output_file = File::create(output_file_name)?; // Create the file
        Ok(Self {
            population,
            output_file,
        })
    }
}

impl View<PopulationModel> for OutputView {
    fn print(&mut self) -> io::Result<()> {
        let mut writer = BufWriter::new(&self.output_file);

        for person in self.population.people.iter() {
            write!(writer, "ID({}): {:?}  |  ", person.id, person.status)?;
        }

        writeln!(writer)?; // Add a blank line after the day's data
        writer.flush()?; // Ensure all data is written to the file

        Ok(())
    }
}

fn main() {
    // Set up command line arguments
    let command_line_args = CLA {
        num_simulations: 3,
        num_people: 5,
        num_days: 5,
        infection_prob: 0.5,
        vaccination_prob: 0.2,
        transmission_prob: 0.5,
        death_prob: 0.1,
        output_file: "output.txt".to_string(),
    };

    println!("Virus!");
}
