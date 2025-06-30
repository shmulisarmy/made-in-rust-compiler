use std::fmt::Display;

pub type NodeIndex = usize;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub prev: Option<NodeIndex>,
    pub next: Option<NodeIndex>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub storage: Vec<Node<T>>,
    pub head: Option<NodeIndex>,
    pub tail: Option<NodeIndex>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            head: None,
            tail: None,
        }
    }

    pub fn append(&mut self, value: T) -> NodeIndex {
        let node = Node {
            value,
            prev: self.tail,
            next: None,
        };

        self.storage.push(node);
        let node_index = self.storage.len() - 1;

        // Update the previous tail's next pointer
        if let Some(tail_index) = self.tail {
            self.storage[tail_index].next = Some(node_index);
        } else {
            // This is the first node
            self.head = Some(node_index);
        }

        self.tail = Some(node_index);
        node_index
    }

    pub fn link(&mut self, prev_index: NodeIndex, next_index: NodeIndex) {
        // Ensure indices are valid
        if prev_index >= self.storage.len() || next_index >= self.storage.len() {
            panic!(
                "Invalid node index, the node index is greater than the node list, (either the node list got shrunk or your using a node from one linkedList on a another)"
            );
        }

        // Update the previous node's next pointer
        self.storage[prev_index].next = Some(next_index);

        // Update the next node's prev pointer
        self.storage[next_index].prev = Some(prev_index);
    }

    pub fn remove(&mut self, node_index: NodeIndex) {
        if node_index >= self.storage.len() {
            panic!(
                "Invalid node index, the node index is greater than the node list, (either the node list got shrunk or your using a node from one linkedList on a another)"
            );
        }

        let node = &self.storage[node_index];
        let prev_index = node.prev;
        let next_index = node.next;

        // Update head if removing the head node
        if self.head == Some(node_index) {
            self.head = next_index;
        }

        // Update tail if removing the tail node
        if self.tail == Some(node_index) {
            self.tail = prev_index;
        }

        // Link the previous and next nodes together
        if let Some(prev) = prev_index {
            if prev < self.storage.len() {
                self.storage[prev].next = next_index;
            }
        }

        if let Some(next) = next_index {
            if next < self.storage.len() {
                self.storage[next].prev = prev_index;
            }
        }
    }

    pub fn remove_neighbours(&mut self, node_index: NodeIndex) {
        self.remove(self.storage[node_index].prev.unwrap());
        self.remove(self.storage[node_index].next.unwrap());
    }

    pub fn get_two_down(&self, node_index: NodeIndex) -> Option<NodeIndex> {
        match self.storage[node_index].next {
            Some(next) => self.storage[next].next,
            None => None,
        }
    }

    pub fn get(&self, node_index: NodeIndex) -> Option<&T> {
        self.storage.get(node_index).map(|node| &node.value)
    }

    pub fn get_mut(&mut self, node_index: NodeIndex) -> Option<&mut T> {
        self.storage.get_mut(node_index).map(|node| &mut node.value)
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            list: self,
            current: self.head,
        }
    }
    // pub fn display(&self) {
    //     let mut current = self.head;
    //     while let Some(node_index) = current {
    //         println!("{}", self.storage[node_index].value);
    //         current = self.storage[node_index].next;
    //     }
    // }
}

struct LinkedListIterator<'a, T> {
    list: &'a LinkedList<T>,
    current: Option<NodeIndex>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.current?;
        let node = &self.list.storage[current_index];
        self.current = node.next;
        Some(&node.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut list = LinkedList::new();
        list.append("first");
        assert_eq!(list.len(), 1);
        assert_eq!(list.get(0), Some(&"first"));
    }

    #[test]
    fn test_link() {
        let mut list = LinkedList::new();
        let node1 = list.append("first");
        let node2 = list.append("second");
        list.link(node1, node2);
        assert_eq!(list.get(node1), Some(&"first"));
        assert_eq!(list.get(node2), Some(&"second"));
    }

    #[test]
    fn test_remove() {
        let mut list = LinkedList::new();
        let node1 = list.append("first");
        let node2 = list.append("second");
        list.remove(node1);
        assert_eq!(list.get(node2), Some(&"second"));
    }
}
