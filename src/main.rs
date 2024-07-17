fn main() {
    let list = LinkedList::parse(&['A', 'B', 'C'], 'B');
    // println!("{:?}", node.into_iter().take(5).collect::<Vec<_>>());
    // let floyd = Floyd {};
    // println!("{:?}", floyd.find_cycle(list));
    let brent = Brent {};
    println!("{:?}", brent.find_cycle(list));
}

#[allow(unused)]
struct LinkedList<T> {
    pub start: T,
    pub fs: Box<dyn Fn(T) -> Option<T>>,
}

impl<T: Copy + PartialEq> LinkedList<T> {
    fn parse(seq: &[T], _cycle_start: T) -> LinkedList<T> {
        LinkedList {
            start: seq[0],
            fs: Box::new(|_t| None), // FIXME!
        }
    }

    fn step(&self, t: Option<T>) -> Option<T> {
        t.map_or(None, |t| (self.fs)(t))
    }    
}

type Cycle = (usize, usize);

trait CycleDetector {
    fn find_cycle<T: Copy + PartialEq>(&self, list: LinkedList<T>) -> Option<Cycle>;
}

struct Floyd {}

impl CycleDetector for Floyd {
    // Robert W. Floyd's "Tortoise and Hare" algorithm.
    fn find_cycle<T: Copy + PartialEq>(&self, list: LinkedList<T>) -> Option<Cycle> {
        // Find a repetition list[i] = list[2i]
        // The hare moves twice as fast as the tortoise.
        let mut tort = Some(list.start);
        let mut hare = Some(list.start);
        loop {
            tort = list.step(tort);
            hare = list.step(list.step(hare));
            if hare == None || hare == tort {
                break;
            }
        }

        if hare == None {
            return None;
        }

        // At this point the start of the loop is equi-distant from the current
        // tortoise position and the start of the list, so the hare is moving in
        // a circle and the tortoise, moving towards the circle from the start
        // of the list, will intersect at the beginning of the circle.
        //
        // Find start of repetition. The hare and tortoise move at the same speed.
        let mut start = 0;
        tort = Some(list.start);
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

        Some((start, length))
    }
}

struct Brent {}

impl CycleDetector for Brent {
    // Richard P. Brent's algorithm, also known as the "Teleporting Tortoise".
    fn find_cycle<T: Copy + PartialEq>(&self, list: LinkedList<T>) -> Option<Cycle> {
        // Main phase: hare searches successive powers of two while the
        // tortoise teleports to the hare's position after each pass.
        let mut tort = Some(list.start);
        let mut hare = Some(list.start);
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
            if hare == None || hare == tort { break; }
        }

        if hare == None {
            return None;
        }

        /*
        * With the tortoise starting from the head of the list and the hare
        * spotted 'length' steps ahead, advance the tortoise and hare at the
        * same speed until they meet at the start of the repetition.
        */
        let mut start = 0;
        tort = Some(list.start);
        hare = Some(list.start);
        for _ in 0..length {
            hare = list.step(hare);
        }
        while tort != hare {
            tort = list.step(tort);
            hare = list.step(hare);
            start += 1;
        }

        Some((start, length))
    }
}
