use std::mem::take;

pub struct Node<T> {
    pub val: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node { val, next: None }
    }

    pub fn push_left(self, val: T) -> Self {
        Node {
            val,
            next: Some(Box::new(self)),
        }
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator(Some(self))
    }
}

pub struct LinkedListIterator<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = &mut self.0;

        match cur {
            Some(node) => {
                let ret = &node.val;
                *cur = node.next.as_deref();

                Some(ret)
            }
            None => None,
        }
    }
}

pub struct LinkedListIntoIterator<T>(Option<Node<T>>);

impl<T> Iterator for LinkedListIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        take(&mut self.0).map(|node| {
            // unbox the next node
            self.0 = node.next.map(|next_node| *next_node);

            node.val
        })
    }
}

impl<T> IntoIterator for Node<T> {
    type Item = T;

    type IntoIter = LinkedListIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIntoIterator(Some(self))
    }
}
