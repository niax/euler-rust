use std::collections::PriorityQueue;

/// Represents a pair of numbers that are Coprime
#[deriving(Show,PartialEq,Eq)]
struct CoprimePair {
    m: uint,
    n: uint,
}

impl Ord for CoprimePair {
    fn cmp(&self, other: &CoprimePair) -> Ordering {
        let sum_self = self.m + self.n;
        let sum_other = other.m + other.n;
        // XXX: Note that the cmp is inverted
        // We want the priq to consider smaller pairs as being "bigger"
        sum_other.cmp(&sum_self)
    }
}

impl PartialOrd for CoprimePair {
    fn partial_cmp(&self, other: &CoprimePair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Iterator over pairs of numbers that are Coprime
struct CoprimePairIterator {
    fringe: PriorityQueue<CoprimePair>,
    next_values: Vec<CoprimePair>,
}

impl CoprimePairIterator {
    pub fn new() -> CoprimePairIterator {
        CoprimePairIterator {
            fringe: PriorityQueue::from_vec(vec![ CoprimePair { m: 2, n: 1 } ]),
            next_values: vec![ CoprimePair { m: 2, n: 1 } ],
        }
    }
}

impl Iterator<CoprimePair> for CoprimePairIterator {
    fn next(&mut self) -> Option<CoprimePair> {
        if self.next_values.len() > 0 {
            // If we have a value already calculated, use that!
            self.next_values.pop()
        } else {
            // Use the fringe to find the next values
            let node = self.fringe.pop().unwrap();
            let b1_node = CoprimePair {
                m: (2 * node.m - node.n),
                n: node.m
            };
            let b2_node = CoprimePair {
                m: (2 * node.m + node.n),
                n: node.m
            };
            let b3_node = CoprimePair {
                m: (node.m + 2 * node.n),
                n: node.n
            };
            for n in vec![b1_node, b2_node, b3_node].iter() {
                // Each new node is added to the fringe, as well as
                // the set of values for the iterator to return next
                self.fringe.push(*n);
                self.next_values.push(*n);
            }
            self.next_values.pop()
        }
    }
}

/// Represents a Pythagorean triple, `(a, b, c)` multiplied by constant, `k`
#[deriving(Show,PartialEq,Eq)]
struct PythagoreanTriple {
    primitive_triple: (uint, uint, uint),
    k: uint,
}

impl PythagoreanTriple {
    /// Return a tuple (a, b, c) for the triple
    pub fn triple(&self) -> (uint, uint, uint) {
        let (a, b, c) = self.primitive_triple;
        (a * self.k, b * self.k, c * self.k)
    }

    /// Return a new triple with the multiplication factor increased by 1
    pub fn incr_k(&self) -> PythagoreanTriple {
        PythagoreanTriple {
            primitive_triple: self.primitive_triple.clone(),
            k: self.k + 1
        }
    }
}

impl Ord for PythagoreanTriple {
    fn cmp(&self, other: &PythagoreanTriple) -> Ordering {
        let (self_a, self_b, self_c) = self.triple();
        let (other_a, other_b, other_c) = other.triple();
        let sum_self = self_a + self_b + self_c;
        let sum_other = other_a + other_b + other_c;
        // XXX: Again, note that the cmp is inverted for smaller
        // values at the top of a priq
        sum_other.cmp(&sum_self)
    }
}

impl PartialOrd for PythagoreanTriple {
    fn partial_cmp(&self, other: &PythagoreanTriple) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// This iterator returns Pythagorean triples in order from smallest sum.
struct PythagoreanTripleIterator {
    coprime_iter: CoprimePairIterator,
    triple_queue: PriorityQueue<PythagoreanTriple>,
}

impl PythagoreanTripleIterator {
    pub fn new() -> PythagoreanTripleIterator {
        PythagoreanTripleIterator {
            coprime_iter: CoprimePairIterator::new(),
            triple_queue: PriorityQueue::new(),
        }
    }

    // Get the next primitive triple based on the coprime pairs
    fn next_primitive(&mut self) -> PythagoreanTriple {
        let pair = self.coprime_iter.next().unwrap();
        // Assert that m > n (so swap if not)
        let (m, n) = if pair.m > pair.n {
            (pair.m, pair.n)
        } else {
            (pair.n, pair.m)
        };
        let m_square = m * m;
        let n_square = n * n;
        PythagoreanTriple {
            primitive_triple: (m_square - n_square, 2 * m * n, m_square + n_square),
            k: 1
        }
    }
}

impl Iterator<PythagoreanTriple> for PythagoreanTripleIterator {
    fn next(&mut self) -> Option<PythagoreanTriple> {
        // Always push on a new primitive
        let new_primitive = self.next_primitive();
        self.triple_queue.push(new_primitive);

        // Get the top most triple (the smallest)
        let next_triple = self.triple_queue.pop().unwrap();
        // Push back on the next triple in that series
        self.triple_queue.push(next_triple.incr_k());
        // Return the triple we found from the queue
        Some(next_triple)
    }
}



/// Get the product of the Pythagorean triple who's sum is equal to `equal_to`
fn pythagorean_triple_product(equal_to: uint) -> uint {
    let mut iter = PythagoreanTripleIterator::new();
    // Look at each triple
    for i in iter {
        let (a, b, c) = i.triple();
        // If the sum of values is equal to our target...
        if a + b + c == equal_to {
            // Return the triple's product
            return a * b * c;
        }
    }
    fail!("Fell out of an infinite loop!");
}


#[cfg(test)]
mod tests {
    extern crate test;

    use self::test::Bencher;
    use super::{CoprimePairIterator, CoprimePair, PythagoreanTripleIterator, pythagorean_triple_product};

    #[test]
    fn test_coprime_iterator() {
        let mut iter = CoprimePairIterator::new();
        assert_eq!(iter.next().unwrap(), CoprimePair { m: 2, n: 1 });
        assert_eq!(iter.next().unwrap(), CoprimePair { m: 4, n: 1 });
        assert_eq!(iter.next().unwrap(), CoprimePair { m: 5, n: 2 });
        assert_eq!(iter.next().unwrap(), CoprimePair { m: 3, n: 2 });
        assert_eq!(iter.next().unwrap(), CoprimePair { m: 7, n: 2 });
        assert_eq!(iter.next().unwrap(), CoprimePair { m: 8, n: 3 });
        assert_eq!(iter.next().unwrap(), CoprimePair { m: 4, n: 3 });
    }

    #[test]
    fn test_triple_iterator() {
        let mut iter = PythagoreanTripleIterator::new();
        assert_eq!(iter.next().unwrap().triple(), (3u, 4u, 5u));
        assert_eq!(iter.next().unwrap().triple(), (6u, 8u, 10u));
    }

    #[test]
    fn provided_example() {
        // a = 3, b = 4, c = 5 - Sum is 12, product is 60
        assert_eq!(pythagorean_triple_product(12), 60u);
    }

    #[test]
    fn expected_result() {
        assert_eq!(pythagorean_triple_product(1000), 31875000u);
    }

    #[bench]
    fn bench_triple_product(b: &mut Bencher) {
        b.iter(|| {
            pythagorean_triple_product(1000);
        })
    }
}
