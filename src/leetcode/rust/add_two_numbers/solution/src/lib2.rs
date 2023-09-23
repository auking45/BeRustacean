// Definition for singly-linked list.
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

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut carry = 0;
      let mut head = Box::new(ListNode::new(0));
      let mut tail = &mut head;

      while l1 != None || l2 != None || carry != 0 {
        let val = match l1 {
          Some(n) => { l1 = n.next; n.val },
          None => 0
        } + match l2 {
          Some(n) => { l2 = n.next; n.val },
          None => 0
        } + carry;

        carry = val / 10;
        tail.next = Some(Box::new(ListNode::new(val % 10)));
        tail = tail.next.as_mut().unwrap();
      }
      head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(v: Vec<i32>) -> Option<Box<ListNode>> {
      let mut front = Box::new(ListNode::new(0));
      let mut current = &mut front;

      for x in v {
        current.next = Some(Box::new(ListNode::new(x)));
        current = current.next.as_mut().unwrap();
      }

      front.next
    }

    #[test]
    fn test_1() {
      assert_eq!(
        Solution::add_two_numbers(helper(vec![2, 4, 3]), helper(vec![5, 6, 4])),
        helper(vec![7, 0, 8])
      );
    }

    #[test]
    fn test_2() {
      assert_eq!(
        Solution::add_two_numbers(helper(vec![0]), helper(vec![0])),
        helper(vec![0])
      );
    }

    #[test]
    fn test_3() {
      assert_eq!(
        Solution::add_two_numbers(helper(vec![9, 9, 9, 9, 9, 9, 9]), helper(vec![9, 9, 9, 9])),
        helper(vec![8, 9, 9, 9, 0, 0, 0, 1])
      );
    }
}
