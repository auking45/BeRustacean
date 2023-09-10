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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut front = Box::new(ListNode::new(0));
        let mut current = &mut front;
        let mut carry = false;

        let mut num1 = &l1;
        let mut num2 = &l2;

        loop {
          let mut total = 0;

          match (num1, num2) {
            (Some(n1), Some(n2)) => {
              total = n1.val + n2.val + carry as i32;
              num1 = &n1.next;
              num2 = &n2.next;
            }
            (Some(n1), None) => {
              total = n1.val + carry as i32;
              num1 = &n1.next;
            }
            (None, Some(n2)) => {
              total = n2.val + carry as i32;
              num2 = &n2.next;
            }
            (None, None) => {
              break;
            }
          }

          if total >= 10 {
            total %= 10;
            carry = true;
          } else {
            carry = false;
          }

          current.next = Some(Box::new(ListNode::new(total)));
          current = current.next.as_mut().unwrap();
        }

        if carry {
          current.next = Some(Box::new(ListNode::new(1)));
        }

        front.next
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
