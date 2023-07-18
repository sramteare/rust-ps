#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LNode {
    pub val: i32,
    pub next: Option<Box<LNode>>
}
impl LNode {
    fn new (val: i32) -> Self{
        LNode {
            next: None,
            val
        }
    }
}
pub fn merge_two_lists(mut list1: Option<Box<LNode>>, mut list2: Option<Box<LNode>>) -> Option<Box<LNode>>{
    
    let mut anchor = LNode::new(0);
    let mut cur = &mut anchor;
    while let (Some(l1), Some(l2)) = (&list1, &list2) {
        let tmp = if l1.val < l2.val {
            &mut list1
        } else { &mut list2 }; // get least

        cur.next = tmp.take(); // take and set cur.next
        cur = cur.next.as_mut().unwrap(); // move cur to cur.next
        *tmp = cur.next.take(); // move cur.next to l1 or l2
    }
    cur.next = list1.or(list2);
    anchor.next
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_merge_sorted_ll () {
        use super::*;
        let mut list1 = LNode::new(2);
        let mut list2 = LNode::new(1);
        list1.next = Some(Box::new(LNode::new(4)));
        let n = list1.next.as_mut().unwrap();
        n.next = Some(Box::new(LNode::new(6)));

        list2.next = Some(Box::new(LNode::new(3)));
        let n = list2.next.as_mut().unwrap();
        n.next = Some(Box::new(LNode::new(4)));
        
        let mut merged =  LNode::new(1);
        merged.next =  Some(Box::new(LNode::new(2)));
        let n = merged.next.as_mut().unwrap();
        n.next = Some(Box::new(LNode::new(3)));
        
        let n = n.next.as_mut().unwrap();
        n.next = Some(Box::new(LNode::new(4)));
        let n = n.next.as_mut().unwrap();
        n.next = Some(Box::new(LNode::new(4)));
        let n = n.next.as_mut().unwrap();
        n.next = Some(Box::new(LNode::new(6)));

        assert_eq!(merge_two_lists(Some(Box::new(list1.clone())), Some(Box::new(list2.clone()))), Some(Box::new(merged)));
    }
    #[test]
    fn should_merge_sorted_ll2 () {
        use super::*;
        let  list1 = LNode::new(2);
        let mut list2 = LNode::new(1);
        // list1.next = Some(Box::new(LNode::new(4)));
        // let n = list1.next.as_mut().unwrap();
        // n.next = Some(Box::new(LNode::new(6)));

        list2.next = Some(Box::new(LNode::new(3)));
        let n = list2.next.as_mut().unwrap();
        n.next = Some(Box::new(LNode::new(4)));
        
        let mut merged =  LNode::new(1);
        merged.next =  Some(Box::new(LNode::new(2)));
        let n = merged.next.as_mut().unwrap();
        n.next = Some(Box::new(LNode::new(3)));
        
        let n = n.next.as_mut().unwrap();
        n.next = Some(Box::new(LNode::new(4)));

        assert_eq!(merge_two_lists(Some(Box::new(list1.clone())), Some(Box::new(list2.clone()))), Some(Box::new(merged)));
    }
}