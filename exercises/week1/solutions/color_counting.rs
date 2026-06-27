/// Sum up the counts of each color in the array.
///
/// The return value should be in the order (red, green, blue).
fn count(colors_array: [ColorEntry; 5]) -> (u64, u64, u64) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let mut i = 0;

    while i < 5 {
        let entry = colors_array[i];
        match entry.color {
            Color::Red => red += entry.count,
            Color::Green => green += entry.count,
            Color::Blue => blue += entry.count,
        }

        i += 1;
    }

    (red, green, blue)
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy)]
struct ColorEntry {
    color: Color,
    count: u64,
}

fn main() {
    // Simple example
    let arr = [
        ColorEntry {
            color: Color::Red,
            count: 10,
        },
        ColorEntry {
            color: Color::Red,
            count: 0,
        },
        ColorEntry {
            color: Color::Red,
            count: 0,
        },
        ColorEntry {
            color: Color::Red,
            count: 0,
        },
        ColorEntry {
            color: Color::Red,
            count: 0,
        },
    ];
    assert_eq!(count(arr), (10, 0, 0));

    // Simple example
    let arr = [
        ColorEntry {
            color: Color::Red,
            count: 1,
        },
        ColorEntry {
            color: Color::Green,
            count: 1,
        },
        ColorEntry {
            color: Color::Blue,
            count: 1,
        },
        ColorEntry {
            color: Color::Blue,
            count: 1,
        },
        ColorEntry {
            color: Color::Red,
            count: 1,
        },
    ];
    assert_eq!(count(arr), (2, 1, 2));

    // Bigger test case
    let arr = [
        ColorEntry {
            color: Color::Blue,
            count: 92837456,
        },
        ColorEntry {
            color: Color::Green,
            count: 429,
        },
        ColorEntry {
            color: Color::Blue,
            count: 830203,
        },
        ColorEntry {
            color: Color::Red,
            count: 32939,
        },
        ColorEntry {
            color: Color::Red,
            count: 9021,
        },
    ];
    assert_eq!(count(arr), (41960, 429, 93667659));

    println!("Your code seems to work well!");
}
