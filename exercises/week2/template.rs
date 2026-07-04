// TODO: Implement the generic type `WorldImpl<const WIDTH: usize, const HEIGHT: usize>` and
// implement the `SimulationWorld` trait for `WorldImpl` of all widths and heights.
//
// Note: `SimulationWorld`'s supertrait `Sized` is necessary, so that `new` can return an
// owned Self value. You do not need manually implement this trait, the compiler automatically
// implements this marker trait for every type whose size in memory it knows at compile time.

trait SimulationWorld: Sized {
    /// Create a new world.
    ///
    /// Each cell in the grid has a probability of population_fraction to be Uninfected
    /// and 1 - population_fraction probability to be Uninhabited.
    fn new(rng: &mut Rng, population_fraction: f64) -> Self;

    /// Change a single cell of the grid from Uninfected to Infected.
    fn infect_random(&mut self, rng: &mut Rng);

    /// Simulate the spread of an infection.
    ///
    /// A zombie infects his uninfected neighbors (to the top, right, bottom, left of him).
    /// The simulation stops when there is no zombie with uninfected neighbor left.
    fn simulate_infection(&mut self);

    /// Return (count_of_uninfected, count_of_infected).
    fn count(&self) -> (u64, u64);

    /// Run a full simulation.
    fn simulation_run(rng: &mut Rng, population_fraction: f64) -> (u64, u64) {
        let mut world = Self::new(rng, population_fraction);
        world.infect_random(rng);
        world.simulate_infection();
        world.count()
    }
}

// Desired example output:
//
// Population 0.10: 100 repetitions showed an average of 0.1% infected
// Population 0.15: 100 repetitions showed an average of 0.1% infected
// Population 0.20: 100 repetitions showed an average of 0.1% infected
// Population 0.25: 100 repetitions showed an average of 0.2% infected
// Population 0.30: 100 repetitions showed an average of 0.1% infected
// Population 0.35: 100 repetitions showed an average of 0.2% infected
// Population 0.40: 100 repetitions showed an average of 0.3% infected
// Population 0.45: 100 repetitions showed an average of 0.6% infected
// Population 0.50: 100 repetitions showed an average of 0.9% infected
// Population 0.55: 100 repetitions showed an average of 3.5% infected
// Population 0.60: 100 repetitions showed an average of 36.3% infected
// Population 0.65: 100 repetitions showed an average of 83.0% infected
// Population 0.70: 100 repetitions showed an average of 95.7% infected
// Population 0.75: 100 repetitions showed an average of 98.2% infected
// Population 0.80: 100 repetitions showed an average of 99.7% infected
// Population 0.85: 100 repetitions showed an average of 99.9% infected
// Population 0.90: 100 repetitions showed an average of 100.0% infected
// Population 0.95: 100 repetitions showed an average of 100.0% infected
fn main() {
    let mut rng = Rng::new(4739234);

    let population_min = 0.1;
    let population_max = 1.0;
    let population_step = 0.05;

    let repetitions = 100;

    let mut population = population_min;
    while population <= population_max {
        let mut rep_counter = 0;
        let mut infection_percentage_sum = 0.;

        while rep_counter < repetitions {
            let (uninfected, infected) =
                WorldImpl::<100, 100>::simulation_run(&mut rng, population);
            let total = uninfected + infected;
            let infection_percentage = infected as f64 / total as f64;
            infection_percentage_sum += infection_percentage;

            rep_counter += 1;
        }

        let infection_percentage_average = infection_percentage_sum / rep_counter as f64;
        println!(
            "Population {population:.2}: {repetitions} repetitions showed an average of {:.1}% infected",
            infection_percentage_average * 100.
        );

        population += population_step;
    }
}

/// Random-number generator based on xoshiro.
pub struct Rng {
    state: [usize; 4],
}

impl Rng {
    /// Instantiate a new random-number generation with a given seed.
    fn new(seed: usize) -> Self {
        let mut state = [0usize; 4];
        let mut s = seed;
        for word in &mut state {
            s = s.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = s;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            *word = z ^ (z >> 31);
        }
        Self { state }
    }

    /// Random usize.
    fn next_usize(&mut self) -> usize {
        let result = (self.state[0].wrapping_add(self.state[3]))
            .rotate_left(23)
            .wrapping_add(self.state[0]);

        let t = self.state[1] << 17;
        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        result
    }

    /// Random f64 in [0, 1).
    fn next_f64(&mut self) -> f64 {
        // Use the upper 53 bits for the mantissa
        let bits = self.next_usize() >> 11;
        bits as f64 * (1.0_f64 / (1usize << 53) as f64)
    }

    /// Random integer in [lo, hi).
    fn next_range(&mut self, lo: usize, hi: usize) -> usize {
        assert!(hi > lo);
        let range = hi - lo;
        let threshold = usize::MAX - (usize::MAX % range);
        loop {
            let r = self.next_usize();
            if r < threshold {
                return lo + (r % range);
            }
        }
    }
}
