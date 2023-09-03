use petgraph::{algo::dijkstra, graphmap::DiGraphMap};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Coordinate")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

struct CoordinateData {
    x: usize,
    y: usize,
    elevation: u32,
    is_start: bool,
    is_end: bool,
}

impl CoordinateData {
    fn new(x: usize, y: usize, rep_char: char) -> CoordinateData {
        let elevation = if rep_char.is_ascii_lowercase() {
            u32::from(rep_char) - u32::from('a')
        } else if rep_char == 'S' {
            0
        } else if rep_char == 'E' {
            25
        } else {
            panic!()
        };
        CoordinateData {
            x,
            y,
            elevation,
            is_start: rep_char == 'S',
            is_end: rep_char == 'E',
        }
    }

    fn coordinate(&self) -> Coordinate {
        Coordinate::new(self.x, self.y)
    }
}

struct ElevationMap {
    data: HashMap<Coordinate, CoordinateData>,
    graph: DiGraphMap<Coordinate, ()>,
    width: usize,
    height: usize,
}

impl ElevationMap {
    fn calc_shortest_path(&self) -> HashMap<Coordinate, u32> {
        dijkstra(
            &self.graph,
            self.get_start_coord(),
            None,
            // Some(self.get_end_coord()),
            |_| 1,
        )
    }

    fn get_start_coord(&self) -> Coordinate {
        *self.data.iter().find(|(_, data)| data.is_start).unwrap().0
    }

    fn get_end_coord(&self) -> Coordinate {
        *self.data.iter().find(|(_, data)| data.is_end).unwrap().0
    }
}

impl From<File> for ElevationMap {
    fn from(data_file: File) -> ElevationMap {
        let mut elevation_graph = DiGraphMap::new();
        let mut elevation_data: HashMap<Coordinate, CoordinateData> = HashMap::new();

        let reader = BufReader::new(data_file);
        let mut x = 0;
        let mut y = 0;
        for line in reader.lines() {
            x = 0;
            let line = line.expect("Could not get line");
            for c in line.chars() {
                let curr_coordinate = Coordinate::new(x, y);
                elevation_graph.add_node(curr_coordinate);
                let curr_data = CoordinateData::new(x, y, c);
                let check_coords = [
                    Coordinate::new(x.wrapping_sub(1), y),
                    Coordinate::new(x, y.wrapping_sub(1)),
                ];
                for check_coord in check_coords {
                    if elevation_data
                        .get(&check_coord)
                        .is_some_and(|check_data| check_data.elevation + 1 >= curr_data.elevation)
                    {
                        elevation_graph.add_edge(check_coord, curr_coordinate, ());
                    }

                    if elevation_data
                        .get(&check_coord)
                        .is_some_and(|check_data| curr_data.elevation + 1 >= check_data.elevation)
                    {
                        elevation_graph.add_edge(curr_coordinate, check_coord, ());
                    }
                }
                elevation_data.insert(Coordinate::new(x, y), curr_data);
                x += 1;
            }

            y += 1;
        }

        ElevationMap {
            data: elevation_data,
            graph: elevation_graph,
            width: x,
            height: y,
        }
    }
}

fn main() {
    let file = File::open("data/day12-full.txt").expect("Could not find data file");
    let elev_map = ElevationMap::from(file);
    println!("Start coord: {}", elev_map.get_start_coord());
    println!("End coord: {}", elev_map.get_end_coord());
    println!("Width: {}", elev_map.width);
    println!("Height: {}", elev_map.height);

    let result_map = elev_map.calc_shortest_path();
    // for y in 0..elev_map.height {
    //     for x in 0..elev_map.width {
    //         match result_map.get(&Coordinate::new(x, y)) {
    //             Some(dist) => print!("{dist}\t"),
    //             None => print!("X"),
    //         };
    //     }
    //     println!();
    // }

    println!(
        "Distance to end: {}",
        result_map.get(&elev_map.get_end_coord()).unwrap()
    );
}
