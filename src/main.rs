fn main() {
    let linked = LinkedList::parse(&['A', 'B', 'C'], 'B');
    // println!("{:?}", node.into_iter().take(5).collect::<Vec<_>>());
    let floyd = Floyd {};
    println!("{:?}", floyd.find_cycle(linked.start, &linked.fs));
}

struct LinkedList<T> {
    pub start: T,
    pub fs: Box<dyn Fn(T) -> Option<T>>,
}

impl<T: Copy> LinkedList<T> {
    fn parse(seq: &[T], _cycle_start: T) -> LinkedList<T> {
        LinkedList {
            start: seq[0],
            fs: Box::new(|_t| None), // FIXME!
        }
    }
}

type Cycle = (usize, usize);

trait CycleDetector {
    fn find_cycle<T: Clone + PartialEq, FS: Fn(T) -> Option<T>>(
        &self,
        start: T,
        successor: FS,
    ) -> Option<Cycle>;
}

struct Floyd {}

impl CycleDetector for Floyd {
    fn find_cycle<T, FS>(&self, _start: T, _successor: FS) -> Option<Cycle> {
        None // FIXME!
    }
}
