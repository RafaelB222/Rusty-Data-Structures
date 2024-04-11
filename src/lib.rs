struct List<T> {
    head: Link<T>,
    length: usize,
}


impl<T> List<T> {

    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, element: T) {
        let old_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element,
            next: old_head,
        }));
        self.head = new_head;
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T>{
        self.head
        .take()
        .map(|n| {
            self.length -= 1;
            self.head = n.next;
            n.element
        })
    }

    pub fn peek(&self) -> Option<&T>{
        match &self.head {
            Some(n) => Some(&n.element),
            None => None,
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Node<T> {
    element: T, 
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    let mut list: List<u32> = List::new();
    assert_eq!(None, list.peek());
    assert_eq!(None, list.pop());
    list.push(2);
    assert_eq!(Some(&2), list.peek());
    list.push(3);
    assert_eq!(2, list.length);
    assert_eq!(3, list.pop().unwrap());
    list.push(4);
    assert_eq!(&4, list.peek().unwrap());
    assert_eq!(2, list.length);

    let mut iter = list.into_iter();
    assert_eq!(Some(4), iter.next());
    assert_eq!(1, iter.0.length);
    assert_eq!(Some(2), iter.next());
    assert_eq!(None, iter.next());
    
    }
}
