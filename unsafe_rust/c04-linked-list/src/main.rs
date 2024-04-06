use std::ops::Range;

struct LinkedList(*const Node);

struct Node {
    first: u64,
    rest: LinkedList,
}

impl LinkedList {
    fn range(range: Range<u64>) -> Self {
        let mut list = LinkedList(std::ptr::null());
        for value in range.rev() {
            let node = Node {
                first: value,
                rest: list,
            };
            list = LinkedList(Box::into_raw(Box::new(node)));
        }

        list
    }

    fn sum(&self) -> u64 {
        if self.0.is_null() {
            0
        } else {
            let node = unsafe { std::ptr::read(self.0) };
            node.first + LinkedList::sum(&node.rest)
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
            Some(list) if !list.0.is_null() => {
                let node = unsafe { &*list.0 };
                Some(&node.rest)
            }
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
    for item in list.into_iter() {
        size += std::mem::size_of_val(&item);
        println!("Deep size of list: {}", size);
    }
    println!("Deep size of list: {}", size);
}

#[test]
fn test_sum() {
    let list = LinkedList::range(1..10);
    assert_eq!(LinkedList::sum(&list), 45);
}
