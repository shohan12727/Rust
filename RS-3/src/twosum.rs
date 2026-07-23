// This is leetcode problem -2 solution
    impl Solution {
        pub fn add_two_numbers(
            mut l1: Option<Box<ListNode>>,
            mut l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut dummy = Box::new(ListNode::new(0));
            let mut current = &mut dummy;
            let mut carry = 0;

            while l1.is_some() || l2.is_some() || carry !=0 {

                let x = match l1.take() {
                    Some(node) => {
                        l1 = node.next;
                        node.val
                    }
                    None => 0,
                };
                
                let y = match l2.take(){
                    Some(node) => {
                        l2 = node.next;
                        node.val
                    }
                    None => 0,
                };

                let sum = x + y + carry;

                carry = sum / 10;

                current.next = Some(Box::new(ListNode::new(sum % 10)));

                current = current.next.as_mut().unwrap();

            }
            dummy.next
        }
    }



