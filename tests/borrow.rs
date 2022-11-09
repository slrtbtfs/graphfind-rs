trait BorrowingIterator<'a>{
    fn iterator(& self) -> Box<dyn Iterator<Item = i32> + 'a>;
    fn one_more(& self) -> Box<dyn Iterator<Item = i32> + 'a>{
        Box::new(self.iterator().map(|i| i+1))
    }
}

struct RangeIterator {}

impl<'a> BorrowingIterator<'a> for RangeIterator{
    fn iterator(& self) -> Box<dyn Iterator<Item = i32> + 'a> {
        Box::new(1..10)
    }
}

#[test]
fn test_borrowing_iterator(){
    let ri = RangeIterator{};
    let bi: Box<dyn BorrowingIterator<>> = Box::new(ri);

    let values : Vec<_> =bi.iterator().collect();
    let larger_values : Vec<_> =bi.one_more().collect();
    dbg!(values);
    dbg!(larger_values);
}