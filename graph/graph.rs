use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashSet;

static COUNTER: AtomicUsize = AtomicUsize::new(1000);

struct GraphNode<'a>{
    id : usize,
    data : u32,
    path : Vec<Node<'a>>
}

type Node<'a> = Rc<RefCell<GraphNode<'a>>>;

impl<'a> GraphNode<'a>{
    
    fn new( data : u32, path : Vec<Node> ) -> Node{
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        Rc::new(RefCell::new(
            GraphNode{
                id,
                data,
                path
            }
        ))
    }

    fn add_node(&mut self,node : Node<'a>){
        self.path.push(Rc::clone(&node));
    }

    fn dfs(node: &Node<'a>) {
        let mut visited = HashSet::new();
        let mut stack = vec![Rc::clone(node)];
        
        while let Some(node) = stack.pop() {
            let node_ref = node.borrow();
            if visited.insert(node_ref.id) {
                println!("{}", node_ref.data);
                for neighbor in &node_ref.path {
                    stack.push(Rc::clone(neighbor));
                }
            }
        }
    }
}

fn main(){
    let node1 = GraphNode::new(1,vec![]);
    let node2 = GraphNode::new(2,vec![Rc::clone(&node1)]);
    let node3 = GraphNode::new(3,vec![Rc::clone(&node1),Rc::clone(&node2)]);
    let node4 = GraphNode::new(4,vec![Rc::clone(&node3)]);
    node1.borrow_mut().add_node(node4);
    GraphNode::dfs(&node1);
}