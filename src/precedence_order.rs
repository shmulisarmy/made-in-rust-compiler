use crate::linkedList::*;



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_precedence() {

        let mut ll: LinkedList<String> = LinkedList::new();
        for item in vec!["a", "=", "b", "+", "c", "*", "9"] {
            ll.append(item.to_string());
        }

        let operands = ['+', '-', '*', '/', '%', '=', '!', '&', '|'];

        let mut current = ll.head;
        while let Some(node_index) = current {
            // let node = &ll.storage[node_index];

            if operands.contains(&ll.storage[node_index].value.chars().next().unwrap()) {
                absorb_neighbors(&mut ll, node_index);
            }
            // println!("Node: {}, value: {}", node_index, node.value);
            current = ll.storage[node_index].next;
        }


        ll.display();
    }

}

static OPERANDS: &[char] = &['+', '-', '*', '/', '%', '=', '!', '&', '|'];

use std::sync::LazyLock;
static OPERATOR_PRECEDENCE: LazyLock<std::collections::HashMap<char, u8>> = LazyLock::new(|| {
    let mut map = std::collections::HashMap::new();
    map.insert('*', 3);
    map.insert('/', 3);
    map.insert('%', 3);
    map.insert('+', 2);
    map.insert('-', 2);
    map.insert('=', 1);
    map.insert('!', 1);
    map.insert('&', 1);
    map.insert('|', 1);
    map
});




fn two_down_is_greater(ll: &mut LinkedList<String>, node_index: NodeIndex)->bool {
    let double_next = ll.get_two_down(node_index);
    if double_next.is_none() {
        return false;
    }
    let two_down_is_not_operand = !OPERANDS.contains(&ll.storage[node_index].value.chars().next().unwrap());
    if two_down_is_not_operand {
        return false;
    }
    let this_onces_precedence = OPERATOR_PRECEDENCE.get(&ll.storage[node_index].value.chars().next().unwrap()).unwrap();
    let double_next_precedence = OPERATOR_PRECEDENCE.get(&ll.storage[double_next.unwrap()].value.chars().next().unwrap()).unwrap();
    this_onces_precedence < double_next_precedence 
}


fn absorb_neighbors(ll: &mut LinkedList<String>, node_index: NodeIndex) {
    while two_down_is_greater(ll, node_index) {
        absorb_neighbors(ll, ll.get_two_down(node_index).unwrap());
    }

    let prev = ll.storage[node_index].prev;
    let next = ll.storage[node_index].next;
    ll.storage[node_index].value = format!("({} {} {})", &ll.storage[prev.unwrap()].value, &ll.storage[node_index].value, &ll.storage[next.unwrap()].value);
    ll.remove(prev.unwrap());
    ll.remove(next.unwrap());
}