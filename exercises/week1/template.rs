static WORLD_WIDTH: usize = 100;
static WORLD_HEIGHT: usize = 100;
static WORLD_SIZE: usize = WORLD_WIDTH * WORLD_HEIGHT;

/// The World is a grid of cells with the dimensions WORLD_WIDTH * WORLD_HEIGHT.
struct World {
    // TODO
}

impl World {
    /// Create a new world.
    ///
    /// Each cell in the grid has a probability of population_fraction to be Uninfected
    /// and 1 - population_fraction probability to be Uninhabited.
    fn new(rng: &mut Rng, population_fraction: f64) -> Self {
        todo!();
    }

    /// Infect a cell of the grid.
    ///
    /// If the cell at index is Uninfected, this function changes the value to Infected and returns true.
    /// Otherwise (i.e., the cell is uninhabitated or already infected), the function returns false.
    fn infect(&mut self, index: usize) -> bool {
        todo!();
    }

    /// Change a single cell of the grid from Uninfected to Infected.
    fn infect_random(&mut self, rng: &mut Rng) {
        todo!();
    }

    /// Try to infect the neighbors (top, right, bottom, left) of index and return the number of successful infections.
    ///
    /// This function does not check whether at index there is actually a zombie.
    fn infect_neighbors(&mut self, index: usize) -> u64 {
        todo!();
    }

    /// Simulate the spread of an infection.
    ///
    /// A zombie infects his uninfected neighbors (to the top, right, bottom, left of him).
    /// The simulation stops when there is no zombie with uninfected neighbor left.
    fn simulate_infection(&mut self) {
        todo!();
    }

    /// Return (count_of_uninfected, count_of_infected).
    fn count(&self) -> (u64, u64) {
        todo!();
    }

    /// Run a full simulation.
    ///
    /// This function perfoms the following steps:
    /// - instantiate a World
    /// - infect a random inhabitant
    /// - simulate the spread of the infection
    /// - count the infected and uninfected
    /// - return the count results
    fn simulation_run(rng: &mut Rng, population_fraction: f64) -> (u64, u64) {
        todo!();
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

    // TODO
}

/// The state of a single cell in the World's grid.
#[derive(Clone, Copy)]
enum GridCell {
    Uninhabited,
    Uninfected,
    Infected,
}

/// A coordinate within a World.
///
/// Assuming (0, 0) is the top left of the 100x100 World, the following grid visualizes the relationship
/// between index of the World grid and the corresponding coordinates.
///
/// ┌─────────┬─────────┬─────────┬─ ─ ─
/// │  (0,0)  │  (1,0)  │  (2,0)  │
/// │   [0]   │   [1]   │   [2]   │  ...
/// ├─────────┼─────────┼─────────┼─ ─ ─
/// │  (0,1)  │  (1,1)  │  (2,1)  │
/// │  [100]  │  [101]  │  [102]  │  ...
/// ├─────────┼─────────┼─────────┼─ ─ ─
/// │  (0,2)  │  (1,2)  │  (2,2)  │
/// │  [200]  │  [201]  │  [202]  │  ...
/// ├─ ─ ─ ─ ─┼─ ─ ─ ─ ─┼─ ─ ─ ─ ─┼─ ─ ─
/// │         │         │         │
/// │   ...   │   ...   │   ...   │  ...
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    /// Instantiate a coordinate from a world index.
    ///
    /// Use this function only when you are certain that you the index is valid.
    /// The function will panic when the index is invalid.
    fn from_index(index: usize) -> Self {
        if index >= WORLD_SIZE {
            panic!();
        }

        let x = index % WORLD_WIDTH;
        let y = index / WORLD_WIDTH;

        Coordinate { x, y }
    }

    /// Return the Coordinate that is (x_off, y_off) from the current Coordinate.
    ///
    /// If the target is outside of the World, the function returns None.
    fn offset(&self, x_off: isize, y_off: isize) -> Option<Coordinate> {
        let new_x = self.x.checked_add_signed(x_off);
        let new_y = self.y.checked_add_signed(y_off);

        match (new_x, new_y) {
            (Some(x), Some(y)) if x < WORLD_WIDTH && y < WORLD_HEIGHT => Some(Coordinate { x, y }),
            _ => None,
        }
    }

    /// Get the corresponding World grid index for this Coordinate.
    fn to_index(&self) -> usize {
        self.x + self.y * WORLD_WIDTH
    }
}

/// Random-number generator based on xoshiro.
pub struct Rng {
    state: [u64; 4],
}

impl Rng {
    /// Instantiate a new random-number generation with a given seed.
    fn new(seed: u64) -> Self {
        let mut state = [0u64; 4];
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

    /// Random u64.
    fn next_u64(&mut self) -> u64 {
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
        let bits = self.next_u64() >> 11;
        bits as f64 * (1.0_f64 / (1u64 << 53) as f64)
    }

    /// Random integer in [lo, hi).
    fn next_range(&mut self, lo: u64, hi: u64) -> u64 {
        assert!(hi > lo);
        let range = hi - lo;
        let threshold = u64::MAX - (u64::MAX % range);
        loop {
            let r = self.next_u64();
            if r < threshold {
                return lo + (r % range);
            }
        }
    }
}
