use std::ops::Range;

#[derive(Clone)]
enum LinkedList {
    Nil,
    Cons(u64, Box<LinkedList>),
}

use LinkedList::*;

impl LinkedList {
    fn range(range: Range<u64>) -> Self {
        let mut list = Nil;
        for value in range.rev() {
            list = Cons(value, Box::new(list));
        }

        list
    }

    fn sum(&self) -> u64 {
        match self {
            Nil => 0,
            Cons(first, rest) => first + LinkedList::sum(&rest),
        }
    }
}

struct LinkedListIter<'a> {
    next: Option<&'a LinkedList>,
}

impl<'a> Iterator for LinkedListIter<'a> {
    type Item = &'a LinkedList;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        self.next = match current {
            Some(Cons(_, rest)) => Some(rest),
            _ => None,
        };
        current
    }
}

impl<'a> IntoIterator for &'a LinkedList {
    type Item = &'a LinkedList;
    type IntoIter = LinkedListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter { next: Some(self) }
    }
}

fn main() {
    let list = LinkedList::range(1..3);
    let mut size = std::mem::size_of_val(&list);
    println!("Current size of list: {}", size);
    for item in list.into_iter() {
        size += std::mem::size_of_val(&item);
        println!("Current size of list: {}", size);
    }
    println!("Deep size of list: {}", size);
}

#[test]
fn test_sum() {
    let list = LinkedList::range(1..10);
    assert_eq!(list.sum(), 45);
}
