use std::collections::PriorityQueue;

#[deriving(PartialEq,Eq)]
struct GridItem {
    x: uint,
    y: uint,
    value: uint,
}

impl Ord for GridItem {
    fn cmp(&self, other: &GridItem) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for GridItem {
    fn partial_cmp(&self, other: &GridItem) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_greatest_product(input_grid: Vec<Vec<uint>>) -> uint {
    let mut cell_queue = PriorityQueue::new();
    for i in input_grid.iter().enumerate() {
        let (y, row) = i;
        for j in row.iter().enumerate() {
            let (x, value) = j;
            cell_queue.push(GridItem {
                x: x,
                y: y,
                value: *value
            });
        }
    }

    let mut largest_product = 0u;
    for cell in cell_queue.iter() {
        // Discover which directions we can go from here
        let directions = vec![
            (0, 1), (1, 0), (0, -1), (-1, 0), // North, East, South, West
            (1, 1), (1, -1), (-1, 1), (-1, -1), // Angles
        ];
        for dir in directions.iter() {
            let mut product = 1u;
            for i in range(0, 4) {
                let (dir_x, dir_y) = *dir;
                let signed_cell_x: int = (cell.x as int) + (i * dir_x);
                let signed_cell_y: int = (cell.y as int) + (i * dir_y);
                let cell_x = signed_cell_x as uint;
                let cell_y = signed_cell_y as uint;
                if signed_cell_x < 0 || cell_x >= input_grid.len() ||
                        signed_cell_y < 0 || cell_y >= input_grid.len() {
                    continue;
                }

                // First vector is vector of rows, so Y comes first
                product *= input_grid[cell_y][cell_x];
            }

            if product > largest_product {
                largest_product = product;
            }
        }
    }
    largest_product
}


#[cfg(test)]
mod test {
    use super::find_greatest_product;

    #[test]
    fn expected_result() {
        let input_grid: Vec<Vec<uint>> = vec![
            vec![8, 2, 22, 97, 38, 15, 0, 40, 0, 75, 4, 5, 7, 78, 52, 12, 50, 77, 91, 8],
            vec![49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 4, 56, 62, 0],
            vec![81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 3, 49, 13, 36, 65],
            vec![52, 70, 95, 23, 4, 60, 11, 42, 69, 24, 68, 56, 1, 32, 56, 71, 37, 2, 36, 91],
            vec![22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80],
            vec![24, 47, 32, 60, 99, 3, 45, 2, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50],
            vec![32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70],
            vec![67, 26, 20, 68, 2, 62, 12, 20, 95, 63, 94, 39, 63, 8, 40, 91, 66, 49, 94, 21],
            vec![24, 55, 58, 5, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72],
            vec![21, 36, 23, 9, 75, 0, 76, 44, 20, 45, 35, 14, 0, 61, 33, 97, 34, 31, 33, 95],
            vec![78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 3, 80, 4, 62, 16, 14, 9, 53, 56, 92],
            vec![16, 39, 5, 42, 96, 35, 31, 47, 55, 58, 88, 24, 0, 17, 54, 24, 36, 29, 85, 57],
            vec![86, 56, 0, 48, 35, 71, 89, 7, 5, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58],
            vec![19, 80, 81, 68, 5, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 4, 89, 55, 40],
            vec![4, 52, 8, 83, 97, 35, 99, 16, 7, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66],
            vec![88, 36, 68, 87, 57, 62, 20, 72, 3, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69],
            vec![4, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 8, 46, 29, 32, 40, 62, 76, 36],
            vec![20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 4, 36, 16],
            vec![20, 73, 35, 29, 78, 31, 90, 1, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 5, 54],
            vec![1, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 1, 89, 19, 67, 48],
        ];
        assert_eq!(find_greatest_product(input_grid), 70600674u);
    }
}
