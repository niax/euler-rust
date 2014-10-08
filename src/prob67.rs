#[cfg(test)]
mod test {
    extern crate test;
    use self::test::Bencher;

    use super::super::prob18::max_total;
    use std::io::{BufferedReader,File};

    fn load_triangle(path: &str) -> Vec<Vec<uint>> {
        let file_path = Path::new(path);
        let file = File::open(&file_path);
        let mut reader = BufferedReader::new(file);
        let lines: Vec<String> = reader.lines().map(|x| { x.unwrap() }).collect();
        let mut triangle: Vec<Vec<uint>> = Vec::new();

        for line in lines.iter() {
            let mut tri_line = Vec::new();
            let mut nodes = line.as_slice().split(' ');
            for node in nodes {
                // Clean up node by removing leading 0 and newlines
                let mut clean_node = node.replace("\n", "");
                if clean_node.as_slice().starts_with("0") {
                   clean_node.remove(0);
                }
                let node_val = from_str::<uint>(clean_node.as_slice());
                tri_line.push(node_val.unwrap());
            }

            triangle.push(tri_line);
        }

        triangle
    }

    #[test]
    fn provided_example() {
        let triangle = load_triangle("src/testdata/prob67_example.txt");
        assert_eq!(max_total(triangle), 23u);
    }

    #[test]
    fn expected_result() {
        let triangle = load_triangle("src/testdata/prob67_triangle.txt");
        assert_eq!(max_total(triangle), 7273u);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let triangle = load_triangle("src/testdata/prob67_triangle.txt");
        b.iter(|| {
            assert_eq!(max_total(triangle.clone()), 7273u);
        });
    }
}
