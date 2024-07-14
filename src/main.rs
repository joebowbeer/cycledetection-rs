fn main() {
    let linked = parse(&['A', 'B', 'C'], 'B');
    // println!("{:?}", node.into_iter().take(5).collect::<Vec<_>>());
    let floyd = Floyd {};
    println!("{:?}", floyd.find_cycle(linked.0, &linked.1));
}

type Cycle = (usize, usize);

type LinkedList<T> = (T, Box<dyn Fn(T) -> Option<T>>);

fn parse<T: Copy>(seq: &[T], _cycle_start: T) -> LinkedList<T> {
    (seq[0], Box::new(|_t| None)) // FIXME!
}

trait CycleDetector {
    fn find_cycle<T: Clone + PartialEq, FS: Fn(T) -> Option<T>>(&self, start: T, successor: FS) -> Option<Cycle>;
}

struct Floyd {}

impl CycleDetector for Floyd {
    fn find_cycle<T, FS>(&self, _start: T, _successor: FS) -> Option<Cycle> {
        None // FIXME!
    }
}
