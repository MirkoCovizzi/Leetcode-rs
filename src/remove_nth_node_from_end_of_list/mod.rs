// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    unsafe {
        let mut head = head;
        let mut front: *mut Option<Box<ListNode>> = &mut head;
        let mut tail: *mut Option<Box<ListNode>> = &mut head;
        for _ in 0..n {
            front = &mut (*front).as_mut().unwrap().next;
        }
        if (*front).is_none() {
            return head.take().unwrap().next;
        }
        loop {
            front = &mut (*front).as_mut().unwrap().next;
            if (*front).is_none() {
                break;
            }
            tail = &mut (*tail).as_mut().unwrap().next;
        }
        (*tail).as_mut().unwrap().next = (*tail).as_mut().unwrap().next.as_mut().unwrap().next.take();

        head
    }
}

fn print_list(head: &Option<Box<ListNode>>) -> String {
    let mut output = String::new();

    let mut head = head;
    loop {
        if head.as_ref().unwrap().next.is_some() {
            output.push_str(&format!("{}->", head.as_ref().unwrap().val));
        } else {
            output.push_str(&format!("{}", head.as_ref().unwrap().val));
            break;
        }
        head = &head.as_ref().unwrap().next;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_example() {
        let mut node_1 = ListNode::new(1);
        let mut node_2 = ListNode::new(2);
        let mut node_3 = ListNode::new(3);
        let mut node_4 = ListNode::new(4);
        let node_5 = ListNode::new(5);
        node_4.next = Some(Box::new(node_5));
        node_3.next = Some(Box::new(node_4));
        node_2.next = Some(Box::new(node_3));
        node_1.next = Some(Box::new(node_2));

        let mut head = Some(Box::new(node_1));

        remove_nth_from_end(head, 2);
    }
}