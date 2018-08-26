extern crate rand;
extern crate rustneat;

#[cfg(feature = "telemetry")]
#[macro_use]
extern crate rusty_dashed;

#[cfg(feature = "telemetry")]
mod telemetry_helper;

use rustneat::Environment;
use rustneat::Organism;
use rustneat::Population;

struct FunctionApproximation;

impl Environment for FunctionApproximation {
    fn test(&self, organism: &mut Organism) -> f64 {
        let mut output = vec![0f64];
        let mut distance = 0f64;

        let mut outputs = Vec::new();

        for x in -10..11 {
            organism.activate(&vec![x as f64 / 10f64], &mut output);
            distance += ((x as f64).powf(2f64) - (output[0] * 100f64)).abs();
            outputs.push([x, (output[0] * 100f64) as i64]);
        }

        #[cfg(feature = "telemetry")]
        telemetry!("approximation1", 1.0, format!("{:?}", outputs));

        100f64 / (1f64 + distance)
    }
}

fn main() {
    let mut population = Population::create_population(150);
    let mut environment = FunctionApproximation;
    let mut champion: Option<Organism> = None;

    #[cfg(feature = "telemetry")]
    telemetry_helper::enable_telemetry("?max_fitness=20");

    #[cfg(feature = "telemetry")]
    std::thread::sleep(std::time::Duration::from_millis(2000));

    #[cfg(feature = "telemetry")]
    telemetry!("approximation1", 1.0, format!("{:?}", (-10..11).map(|x| [x, x * x]).collect::<Vec<_>>()));

    #[cfg(feature = "telemetry")]
    std::thread::sleep(std::time::Duration::from_millis(2000));

    let mut value = 0f64;
    while champion.is_none() {
        population.evolve();
        population.evaluate_in(&mut environment);
        for organism in &population.get_organisms() {
            if value < organism.fitness {
                value = organism.fitness;
                println!("{:?}", value);
            }

            if organism.fitness >= 99f64 {
                champion = Some(organism.clone());
            }
        }
    }
    println!("{:?}", champion.unwrap().genome);
}