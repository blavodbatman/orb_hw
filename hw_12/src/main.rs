use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

struct Node {
    vl: u32,
    ne: Option<Link>,
}

struct SinglyLinkedList {
    first: Option<Link>,
    last: Option<Link>,
    size: usize,
}

struct SinglyLinkedListIterator {
    head: Option<Link>,
}

impl SinglyLinkedList {
    fn new() -> Self {
        SinglyLinkedList {
            first: None,
            last: None,
            size: 0,
        }
    }

    fn add_first(&mut self, val: u32) -> bool {
        self.size += 1;
        let node = Rc::new(RefCell::new(Node {
            vl: val,
            ne: self.first.clone(),
        }));
        self.first = Some(node.clone());
        if self.last.is_none() {
            self.last = Some(node);
        }
        true
    }

    fn add_middle(&mut self, val: u32, idx: usize) -> bool {
        if idx > self.size {
            return false;
        }
        if idx == 0 {
            return self.add_first(val);
        }
        if idx == self.size {
            return self.add_last(val);
        }
        self.size += 1;
        let previous = self.iter().nth(idx - 1);
        let node = Rc::new(RefCell::new(Node {
            vl: val,
            ne: previous.as_ref().unwrap().borrow().ne.clone(),
        }));
        previous.unwrap().borrow_mut().ne = Some(node.clone());
        true
    }

    fn add_last(&mut self, val: u32) -> bool {
        self.size += 1;
        let node = Rc::new(RefCell::new(Node { vl: val, ne: None }));
        if self.last.is_some() {
            self.last.as_ref().unwrap().borrow_mut().ne = Some(node.clone());
        } else {
            self.first = Some(node.clone());
        }
        self.last = Some(node);
        true
    }

    fn clear(&mut self) {
        self.first = None;
        self.last = None;
        self.size = 0;
    }

    fn iter(&self) -> impl Iterator<Item = Link> {
        SinglyLinkedListIterator {
            head: self.first.clone(),
        }
    }
}

impl Iterator for SinglyLinkedListIterator {
    type Item = Link;
    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_none() {
            None
        } else {
            let current = self.head.as_ref().unwrap().clone();
            let next = self.head.as_ref().unwrap().borrow().ne.clone();
            self.head = next;
            Some(current)
        }
    }
}

fn main() {
    let mut list = SinglyLinkedList::new();
    assert!(list.add_last(10));
    assert!(list.add_last(20));
    assert!(list.add_last(30));
    assert!(list.add_first(0));
    assert_eq!(list.size, 4);
    list.iter().for_each(|v| println!("{}", v.borrow().vl));
    println!("-----------");

    list.clear();
    assert_eq!(list.size, 0);
    assert!(list.add_first(30));
    assert!(list.add_first(20));
    assert!(list.add_first(10));
    assert!(list.add_last(40));
    assert_eq!(list.size, 4);
    list.iter().for_each(|v| println!("{}", v.borrow().vl));
    println!("-----------");

    assert!(!list.add_middle(111, 5));
    assert!(list.add_middle(0, 0));
    assert!(list.add_middle(111, 1));
    assert!(list.add_middle(222, 6));
    assert_eq!(list.size, 7);
    list.iter().for_each(|v| println!("{}", v.borrow().vl));
}
