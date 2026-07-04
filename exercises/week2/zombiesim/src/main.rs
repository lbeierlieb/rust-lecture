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

struct WorldImpl<const WIDTH: usize, const HEIGHT: usize> {
    grid: [[GridCell; HEIGHT]; WIDTH],
}

impl<const WIDTH: usize, const HEIGHT: usize> SimulationWorld for WorldImpl<WIDTH, HEIGHT> {
    fn new(rng: &mut Rng, population_fraction: f64) -> Self {
        let mut grid = [[GridCell::Uninhabited; HEIGHT]; WIDTH];

        let mut x = 0;

        while x < WIDTH {
            let mut y = 0;
            while y < HEIGHT {
                if rng.next_f64() <= population_fraction {
                    grid[x][y] = GridCell::Uninfected;
                }
                y += 1;
            }
            x += 1;
        }

        Self { grid }
    }

    fn infect_random(&mut self, rng: &mut Rng) {
        loop {
            let x = rng.next_range(0, WIDTH) as usize;
            let y = rng.next_range(0, HEIGHT) as usize;

            if self.infect(x, y) {
                break;
            }
        }
    }

    fn simulate_infection(&mut self) {
        loop {
            let mut infection_counter = 0;

            let mut x = 0;

            while x < WIDTH {
                let mut y = 0;
                while y < HEIGHT {
                    if let GridCell::Infected = self.grid[x][y] {
                        infection_counter += self.infect_neighbors(x, y);
                    }
                    y += 1;
                }
                x += 1;
            }

            if infection_counter == 0 {
                break;
            }
        }
    }

    fn count(&self) -> (u64, u64) {
        let mut uninfected_counter = 0;
        let mut infected_counter = 0;
        let mut x = 0;

        while x < WIDTH {
            let mut y = 0;
            while y < HEIGHT {
                match self.grid[x][y] {
                    GridCell::Uninfected => uninfected_counter += 1,
                    GridCell::Infected => infected_counter += 1,
                    GridCell::Uninhabited => {}
                }
                y += 1;
            }
            x += 1;
        }

        (uninfected_counter, infected_counter)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> WorldImpl<WIDTH, HEIGHT> {
    /// Infect a cell of the grid.
    ///
    /// If the cell at index is Uninfected, this function changes the value to Infected and returns true.
    /// Otherwise (i.e., the cell is uninhabitated or already infected), the function returns false.
    fn infect(&mut self, x: usize, y: usize) -> bool {
        match self.grid[x][y] {
            GridCell::Uninfected => {
                self.grid[x][y] = GridCell::Infected;
                true
            }
            _ => false,
        }
    }

    /// Try to infect the neighbors (top, right, bottom, left) of index and return the number of successful infections.
    ///
    /// This function does not check whether at index there is actually a zombie.
    fn infect_neighbors(&mut self, x: usize, y: usize) -> u64 {
        let coord = (x, y);
        let mut infection_counter = 0;
        self.infect_neighbor(coord, (0, -1), &mut infection_counter);
        self.infect_neighbor(coord, (1, 0), &mut infection_counter);
        self.infect_neighbor(coord, (0, 1), &mut infection_counter);
        self.infect_neighbor(coord, (-1, 0), &mut infection_counter);
        infection_counter
    }

    fn infect_neighbor(
        &mut self,
        coord: (usize, usize),
        offset: (isize, isize),
        infection_counter: &mut u64,
    ) {
        if let Some(neighbor_coord) = offset_coord::<WIDTH, HEIGHT>(coord, offset) {
            if self.infect(neighbor_coord.0, neighbor_coord.1) {
                *infection_counter += 1;
            }
        }
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

/// The state of a single cell in the World's grid.
#[derive(Clone, Copy)]
enum GridCell {
    Uninhabited,
    Uninfected,
    Infected,
}

/// Return the Coordinate that is (x_off, y_off) from the current Coordinate.
///
/// If the target is outside of the World, the function returns None.
fn offset_coord<const WIDTH: usize, const HEIGHT: usize>(
    coord: (usize, usize),
    offset: (isize, isize),
) -> Option<(usize, usize)> {
    let new_x = coord.0.checked_add_signed(offset.0)?;
    let new_y = coord.1.checked_add_signed(offset.1)?;

    if new_x < WIDTH && new_y < HEIGHT {
        Some((new_x, new_y))
    } else {
        None
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
