use std::cmp::max;

pub fn max_total(triangle: Vec<Vec<uint>>) -> uint {
    let mut max_to_node: Vec<Vec<uint>> = Vec::new();
    let mut previous_layer = Vec::new();
    for layer in triangle.iter() {
        let mut layer_max = Vec::new();

        for (i, &node) in layer.iter().enumerate() {
            let mut left = 0u;
            let mut right = 0u;
            // Make sure we don't index out of bounds
            if i != 0 {
                left = previous_layer[i - 1];
            }

            if i < previous_layer.len() {
                right = previous_layer[i];
            }

            let max_path = max(left, right) + node;
            layer_max.push(max_path);
        }

        previous_layer = layer_max.clone();
        max_to_node.push(layer_max);
    }
    *(previous_layer.iter().max().unwrap())
}

#[cfg(test)]
mod test {
    use super::max_total;

    #[test]
    fn provided_example() {
        let triangle = vec![
            vec![3u],
            vec![7u, 4],
            vec![2u, 4, 6],
            vec![8u, 5, 9, 3],
        ];
        assert_eq!(max_total(triangle), 23u);
    }

    #[test]
    fn expected_result() {
        let triangle = vec![
            vec![75u],
            vec![95u, 64],
            vec![17u, 47, 82],
            vec![18u, 35, 87, 10],
            vec![20u, 4, 82, 47, 65],
            vec![19u, 1, 23, 75, 3, 34],
            vec![88u, 2, 77, 73, 7, 63, 67],
            vec![99u, 65, 4, 28, 6, 16, 70, 92],
            vec![41u, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41u, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53u, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70u, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91u, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63u, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![4u, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
        ];
        assert_eq!(max_total(triangle), 1074u);
    }
}
