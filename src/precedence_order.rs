use crate::expression::Expression;
use crate::libs::linkedList::*;

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
}

static OPERAND_CHARS: &[char] = &['+', '-', '*', '/', '%', '=', '!', '&', '|', '(', ')'];

use std::sync::LazyLock;
static OPERATOR_PRECEDENCE: LazyLock<std::collections::HashMap<String, u8>> = LazyLock::new(|| {
    let mut map = std::collections::HashMap::new();
    map.insert("*".to_string(), 3);
    map.insert("/".to_string(), 3);
    map.insert("%".to_string(), 3);
    map.insert("+".to_string(), 2);
    map.insert("-".to_string(), 2);
    map.insert("=".to_string(), 1);
    map.insert("!".to_string(), 1);
    map.insert("&".to_string(), 1);
    map.insert("-=".to_string(), 1);
    map.insert("+=".to_string(), 1);
    map.insert("*=".to_string(), 1);
    map.insert("/=".to_string(), 1);
    map.insert("%=".to_string(), 1);
    map
});

use std::fmt::Display;

use crate::expression::ExpressionPiece;
use crate::expression::FunctionCall;
use crate::expression::OperatorToString;

fn two_down_is_greater(ll: &mut LinkedList<ExpressionPiece>, node_index: NodeIndex) -> bool {
    let double_next = ll.get_two_down(node_index);
    if double_next.is_none() {
        return false;
    }
    if let ExpressionPiece::Operator(two_down_op) = &ll.storage[double_next.unwrap()].value {
        if let ExpressionPiece::Operator(this_op) = &ll.storage[node_index].value {
            let double_next_precedence = OPERATOR_PRECEDENCE.get(two_down_op).unwrap();
            let this_onces_precedence = OPERATOR_PRECEDENCE.get(this_op).unwrap();
            return this_onces_precedence < double_next_precedence;
        }
    }
    false
}

pub fn absorb_neighbors(ll: &mut LinkedList<ExpressionPiece>, node_index: NodeIndex) {
    while two_down_is_greater(ll, node_index) {
        absorb_neighbors(ll, ll.get_two_down(node_index).unwrap());
    }

    let prev = ll.storage[node_index].prev;
    let next = ll.storage[node_index].next;
    ll.storage[node_index].value = ExpressionPiece::FunctionCall(FunctionCall {
        name: OperatorToString(&ll.storage[node_index].value),
        params: vec![
            Expression(
                ll.storage
                    [prev.expect("there must be some kind of value/expression before an operator")]
                .value
                .clone(),
            ),
            Expression(
                ll.storage
                    [next.expect("there must be some kind of value/expression after an operator")]
                .value
                .clone(),
            ),
        ],
    });
    ll.remove(prev.unwrap());
    ll.remove(next.unwrap());
}
