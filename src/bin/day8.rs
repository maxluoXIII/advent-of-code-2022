use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_visible(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> bool {
    let (x, y) = pos;
    let on_border = x == 0 || x == (grid[0].len() - 1) || y == 0 || y == (grid.len() - 1);

    let vis_closure = |vis, test_height: &u32| vis && *test_height < grid[y][x];
    let vis_from_left = grid[y][0..x].iter().fold(true, vis_closure);
    let vis_from_right = grid[y][(x + 1)..].iter().fold(true, vis_closure);

    let mut vis_from_up = true;
    for test_y in 0..y {
        vis_from_up = vis_from_up && grid[test_y][x] < grid[y][x];
    }

    let mut vis_from_down = true;
    for test_y in (y + 1)..grid.len() {
        vis_from_down = vis_from_down && grid[test_y][x] < grid[y][x];
    }

    on_border || vis_from_left || vis_from_right || vis_from_up || vis_from_down
}

fn count_visible_trees(grid: &Vec<Vec<u32>>) -> usize {
    let mut sum = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _height) in row.iter().enumerate() {
            sum += is_visible(grid, (x, y)) as usize;
        }
    }

    sum
}

fn main() {
    let file = File::open("data/day8-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<u32>> = Vec::new();
    for (line_num, line) in reader.lines().enumerate() {
        grid.push(Vec::new());
        for height in line.unwrap().chars() {
            grid[line_num].push(height.to_digit(10).unwrap());
        }
    }

    println!("Visible trees: {}", count_visible_trees(&grid));
}
