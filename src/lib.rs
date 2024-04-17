type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

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

//making an iterator for the list that will not consume it.
pub struct Iter<'a, T> {
    curr: Option<&'a Node<T>>,
}


impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { curr: self.head.as_deref() }
    }
}


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.map(|node| {
            self.curr = node.next.as_deref();
            &node.element
        })
    }
}

//turns the list into an iterator. This will consume the list.
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

    }

    #[test]
    fn into_iter_test() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

    }

    #[test]
    fn iter_test() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

}
