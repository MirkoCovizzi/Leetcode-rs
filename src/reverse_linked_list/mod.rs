// https://leetcode.com/problems/reverse-linked-list/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut prev: Option<Box<ListNode>> = None;

    while let Some(c) = cur {
        prev = Some(Box::new(ListNode {
            val: c.val,
            next: prev
        }));
        cur = c.next;
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::{
        ListNode,
        reverse_list,
    };

    #[test]
    fn given_example() {
        let mut n_1 = ListNode::new(1);
        let mut n_2 = ListNode::new(2);
        let mut n_3 = ListNode::new(3);
        let mut n_4 = ListNode::new(4);
        let mut n_5 = ListNode::new(5);

        n_5.next = None;
        n_4.next = Some(Box::new(n_5));
        n_3.next = Some(Box::new(n_4));
        n_2.next = Some(Box::new(n_3));
        n_1.next = Some(Box::new(n_2));

        let mut m_1 = ListNode::new(5);
        let mut m_2 = ListNode::new(4);
        let mut m_3 = ListNode::new(3);
        let mut m_4 = ListNode::new(2);
        let mut m_5 = ListNode::new(1);

        m_5.next = None;
        m_4.next = Some(Box::new(m_5));
        m_3.next = Some(Box::new(m_4));
        m_2.next = Some(Box::new(m_3));
        m_1.next = Some(Box::new(m_2));

        assert_eq!(reverse_list(Some(Box::new(n_1))), Some(Box::new(m_1)));
    }
}
