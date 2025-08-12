use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let lines_iterator = f.lines().map(|l| l.unwrap());

    let parsed_dimensions_iterator = lines_iterator.map(|l| {
        l.split('x')
            .map(|str| str.parse::<isize>().unwrap())
            .collect::<Vec<_>>()
    });

    let presents: Vec<_> = parsed_dimensions_iterator
        .map(|ds| Present::new(ds[0], ds[1], ds[2]))
        .collect();

    let total_present_surface: isize = presents.iter().map(|p| p.surface).sum();
    let total_ribbon_length: isize = presents.iter().map(|p| p.ribbon_length).sum();

    println!(
        "total_present_surface: {total_present_surface} | total_ribbon_length: {total_ribbon_length}"
    );

    Ok(())
}

struct Present {
    surface: isize,
    ribbon_length: isize,
}

impl Present {
    fn new(l: isize, w: isize, h: isize) -> Self {
        let surface_1 = l * w;
        let surface_2 = w * h;
        let surface_3 = h * l;

        let surface_total = 2 * surface_1
            + 2 * surface_2
            + 2 * surface_3
            + [surface_1, surface_2, surface_3].iter().min().unwrap();

        let mut dimensions = [l, w, h];
        dimensions.sort_unstable();

        let ribbon_length =
            2 * dimensions[0] + 2 * dimensions[1] + dimensions.iter().product::<isize>();

        Self {
            surface: surface_total,
            ribbon_length,
        }
    }
}
