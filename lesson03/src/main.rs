#[derive(Debug)]
pub struct ListNode {
    pub val: Box<u8>,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: u8) -> Self {
        ListNode {
            next: None,
            val: Box::new(val),
        }
    }

    #[inline]
    pub fn new_boxed(val: Box<u8>) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    pub fn with_next(self, next: ListNode) -> Self {
        let next = Some(Box::new(next));
        Self { next, ..self }
    }

    #[inline]
    pub fn with_next_boxed(self, next: Box<ListNode>) -> Self {
        let next = Some(next);
        Self { next, ..self }
    }

    pub fn add_other(self, other: ListNode) -> ListNode {
        let mut first = &Some(Box::new(self));
        let mut second = &Some(Box::new(other));
        let mut over: bool = false;
        let mut reversed = None;
        let mut result = None;

        loop {
            match (first, second) {
                (Some(first_item), Some(second_item)) => {
                    let mut value = *first_item.val + *second_item.val + over as u8;
                    over = if value > 9 {
                        value -= 10;
                        true
                    } else {
                        false
                    };
                    let mut node = ListNode::new(value);
                    node.next = reversed;
                    reversed = Some(Box::new(node));
                    first = &first_item.next;
                    second = &second_item.next;
                }
                (Some(first_item), None) => {
                    let mut value = *first_item.val + over as u8;
                    over = if value > 9 {
                        value -= 10;
                        true
                    } else {
                        false
                    };
                    let mut node = ListNode::new(value);
                    node.next = reversed;
                    reversed = Some(Box::new(node));
                    first = &first_item.next;
                    second = &None;
                }
                (None, Some(second_item)) => {
                    let mut value = *second_item.val + over as u8;
                    over = if value > 9 {
                        value -= 10;
                        true
                    } else {
                        false
                    };
                    let mut node = ListNode::new(value);
                    node.next = reversed;
                    reversed = Some(Box::new(node));
                    second = &second_item.next;
                    first = &None;
                }
                (None, None) => {
                    if over {
                        let mut node = ListNode::new(1u8);
                        node.next = reversed;
                        reversed = Some(Box::new(node));
                    } else {
                        break;
                    }
                }
            }
        }

        while let Some(mut node) = reversed {
            reversed = node.next;
            node.next = result;
            result = Some(node);
        }
        *result.unwrap()
    }
}

fn main() {
    println!("It's as simple as two times two!");
}

#[cfg(test)]
mod test_add {
    use super::*;

    #[test]
    fn zero() {
        let a = ListNode::new(0u8);
        let b = ListNode::new(0u8);

        let c = a.add_other(b);

        assert_eq!(0, *c.val);
        assert!(c.next.is_none());
    }

    #[test]
    fn simple_add() {
        // 123
        let a = ListNode::new(3u8).with_next(ListNode::new(2u8).with_next(ListNode::new(1u8)));

        // 456
        let b = ListNode::new(6u8).with_next(ListNode::new(5u8).with_next(ListNode::new(4u8)));

        let c = a.add_other(b);

        let digit0 = c;
        assert_eq!(9u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(7u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(5u8, *digit2.val);
        assert!(digit2.next.is_none());
    }

    #[test]
    fn nine_in_the_middle() {
        // 1983
        let a = ListNode::new(3u8).with_next(
            ListNode::new(8u8).with_next(ListNode::new(9u8).with_next(ListNode::new(1u8))),
        );

        // 127
        let b = ListNode::new(7u8).with_next(ListNode::new(2u8).with_next(ListNode::new(1u8)));

        let c = a.add_other(b);

        let digit0 = c;
        assert_eq!(0u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(1u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(1u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(2u8, *digit3.val);
        assert!(digit3.next.is_none());
    }

    #[test]
    fn nine_in_the_middle_trans() {
        // 1983
        let a = ListNode::new(3u8).with_next(
            ListNode::new(8u8).with_next(ListNode::new(9u8).with_next(ListNode::new(1u8))),
        );

        // 127
        let b = ListNode::new(7u8).with_next(ListNode::new(2u8).with_next(ListNode::new(1u8)));

        let c = b.add_other(a);

        let digit0 = c;
        assert_eq!(0u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(1u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(1u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(2u8, *digit3.val);
        assert!(digit3.next.is_none());
    }

    #[test]
    fn nines() {
        // 99 999
        let a = ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        ));

        // 9 999
        let b = ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        );

        let c = a.add_other(b);

        let digit0 = c;
        assert_eq!(8u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(9u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(9u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(9u8, *digit3.val);
        assert!(digit3.next.is_some());

        let digit4 = digit3.next.unwrap();
        assert_eq!(0u8, *digit4.val);
        assert!(digit4.next.is_some());

        let digit5 = digit4.next.unwrap();
        assert_eq!(1u8, *digit5.val);
        assert!(digit5.next.is_none());
    }

    #[test]
    fn nines_trans() {
        // 99 999
        let a = ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        ));

        // 9 999
        let b = ListNode::new(9u8).with_next(
            ListNode::new(9u8).with_next(ListNode::new(9u8).with_next(ListNode::new(9u8))),
        );

        let c = b.add_other(a);

        let digit0 = c;
        assert_eq!(8u8, *digit0.val);
        assert!(digit0.next.is_some());

        let digit1 = digit0.next.unwrap();
        assert_eq!(9u8, *digit1.val);
        assert!(digit1.next.is_some());

        let digit2 = digit1.next.unwrap();
        assert_eq!(9u8, *digit2.val);
        assert!(digit2.next.is_some());

        let digit3 = digit2.next.unwrap();
        assert_eq!(9u8, *digit3.val);
        assert!(digit3.next.is_some());

        let digit4 = digit3.next.unwrap();
        assert_eq!(0u8, *digit4.val);
        assert!(digit4.next.is_some());

        let digit5 = digit4.next.unwrap();
        assert_eq!(1u8, *digit5.val);
        assert!(digit5.next.is_none());
    }
}
