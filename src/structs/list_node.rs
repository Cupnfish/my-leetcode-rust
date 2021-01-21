#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn with_node(val: i32, node: Option<Box<ListNode>>) -> Self {
        ListNode { next: node, val }
    }
}

#[macro_export]
macro_rules! list[
    ()                       => (Option::<Box<$crate::structs::list_node::ListNode>>::None);
    ($x:expr)                => (Some(Box::new($crate::structs::list_node::ListNode::new($x))));
    ($x:expr, $($xs:expr),+) => (Some(Box::new($crate::structs::list_node::ListNode::with_node($x,list!($($xs),+)))));
];
