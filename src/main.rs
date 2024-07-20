use std::{cmp::max, collections::HashMap, convert::From, hash::Hash};

fn main() {
    let list = List::from(['A', 'B', 'C', 'B']);
    let detector = Floyd::default();
    println!("{:?}", detector.find_cycle(list));
}

trait SinglyLinked<T> {
    fn start(&self) -> Option<T>;
    fn step(&self, t: Option<T>) -> Option<T>;
}

struct List<T> {
    start: Option<T>,
    map: HashMap<T, T>,
}

impl<T: Copy + Eq + Hash, const N: usize> From<[T; N]> for List<T> {
    fn from(arr: [T; N]) -> Self {
        let start = arr.first().copied();
        let capacity = max(arr.len() as isize - 1, 0) as usize;
        let mut map = HashMap::with_capacity(capacity);
        for i in 0..capacity {
            map.insert(arr[i], arr[i + 1]);
        }
        List { start, map }
    }
}

impl<T: Copy + Eq + Hash> SinglyLinked<T> for List<T> {
    fn start(&self) -> Option<T> {
        self.start
    }

    fn step(&self, t: Option<T>) -> Option<T> {
        t.and_then(|t| self.map.get(&t).copied())
    }
}

#[derive(Debug, PartialEq)]
struct Cycle(usize, usize);

trait CycleDetector<T> {
    fn find_cycle(&self, list: impl SinglyLinked<T>) -> Option<Cycle>;
}

#[derive(Default)]
struct Floyd {}

impl<T: Copy + PartialEq> CycleDetector<T> for Floyd {
    // Robert W. Floyd's "Tortoise and Hare" algorithm.
    fn find_cycle(&self, list: impl SinglyLinked<T>) -> Option<Cycle> {
        // Find a repetition list[i] = list[2i]
        // The hare moves twice as fast as the tortoise.
        let mut tort = list.start();
        let mut hare = list.start();
        loop {
            tort = list.step(tort);
            hare = list.step(list.step(hare));
            if hare.is_none() || hare == tort {
                break;
            }
        }

        hare?;

        // At this point the start of the loop is equi-distant from the current
        // tortoise position and the start of the list, so the hare is moving in
        // a circle and the tortoise, moving towards the circle from the start
        // of the list, will intersect at the beginning of the circle.
        //
        // Find start of repetition. The hare and tortoise move at the same speed.
        let mut start = 0;
        tort = list.start();
        while tort != hare {
            tort = list.step(tort);
            hare = list.step(hare);
            start += 1;
        }

        // Find length of shortest cycle starting from list[start].
        // The hare moves while the tortoise stays still.
        let mut length = 0;
        hare = tort;
        loop {
            hare = list.step(hare);
            length += 1;
            if hare == tort {
                break;
            }
        }

        Some(Cycle(start, length))
    }
}

#[derive(Default)]
struct Brent {}

impl<T: Copy + PartialEq> CycleDetector<T> for Brent {
    // Richard P. Brent's algorithm, also known as the "Teleporting Tortoise".
    fn find_cycle(&self, list: impl SinglyLinked<T>) -> Option<Cycle> {
        // Main phase: hare searches successive powers of two while the
        // tortoise teleports to the hare's position after each pass.
        let mut tort = list.start();
        let mut hare = list.start();
        let mut limit = 1;
        let mut length = 0;
        loop {
            if length == limit {
                // start a new pass
                tort = hare; // teleport
                limit *= 2;
                length = 0;
            }
            hare = list.step(hare);
            length += 1;
            if hare.is_none() || hare == tort {
                break;
            }
        }

        hare?;

        // With the tortoise starting from the head of the list and the hare
        // spotted 'length' steps ahead, advance the tortoise and hare at the
        // same speed until they meet at the start of the repetition.
        let mut start = 0;
        tort = list.start();
        hare = list.start();
        for _ in 0..length {
            hare = list.step(hare);
        }
        while tort != hare {
            tort = list.step(tort);
            hare = list.step(hare);
            start += 1;
        }

        Some(Cycle(start, length))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Brent, Cycle, CycleDetector, Floyd, List};

    fn cycle_detector_tester<D: CycleDetector<char>>(d: D) {
        assert_eq!(d.find_cycle(List::from([])), None, "empty");
        assert_eq!(d.find_cycle(List::from(['A'])), None, "A");
        assert_eq!(d.find_cycle(List::from(['A', 'B', 'C'])), None, "ABC");
        assert_eq!(
            d.find_cycle(List::from(['A', 'A'])),
            Some(Cycle(0, 1)),
            "AA"
        );
        assert_eq!(
            d.find_cycle(List::from(['A', 'B', 'B'])),
            Some(Cycle(1, 1)),
            "ABB"
        );
        assert_eq!(
            d.find_cycle(List::from(['A', 'B', 'A'])),
            Some(Cycle(0, 2)),
            "ABA"
        );
        assert_eq!(
            d.find_cycle(List::from(['A', 'B', 'C', 'B'])),
            Some(Cycle(1, 2)),
            "ABCB"
        );
        assert_eq!(
            d.find_cycle(List::from(['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'D'])),
            Some(Cycle(3, 5)),
            "ABCDEFGHD"
        );
    }

    #[test]
    fn floyd_test() {
        cycle_detector_tester(Floyd::default());
    }

    #[test]
    fn brent_test() {
        cycle_detector_tester(Brent::default());
    }
}
