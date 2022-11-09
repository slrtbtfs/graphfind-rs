trait IteratorProducer<T> {
    fn iterator<'a>(&'a self) -> Box<dyn Iterator<Item = T> + 'a> where T: 'a;
    // Default implementation. Actual trait impls might have a more
    // efficient way to implement this.
    fn transformed<'a>(&'a self) -> Box<dyn Iterator<Item = T> + 'a> where T: 'a{
        Box::new(self
            .iterator()
            // Here rustc complains that i has insufficent lifetime. Why?
            .map(|i| self.transform(i)))
    }
    fn transform(&self, x: T) -> T;
}

struct RangeIterator {}

impl IteratorProducer<i32> for RangeIterator {
    fn iterator<'a>(&'a self) -> Box<dyn Iterator<Item = i32> + 'a> where i32: 'a {
        Box::new(1..10)
    }
    fn transform(&self, x: i32) -> i32 {
        x + 1
    }
}

#[test]
fn test_borrowing_iterator() {
    let ri = RangeIterator {};
    let bi: Box<dyn IteratorProducer<_>> = Box::new(ri);

    let values: Vec<_> = bi.iterator().collect();
    let larger_values: Vec<_> = bi.transformed().collect();
    dbg!(values);
    dbg!(larger_values);
}
