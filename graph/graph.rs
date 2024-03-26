use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashSet;
use std::collections::VecDeque;

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
        self.path.push(node);
    }

    fn dfs(node: &Node<'a>) {
        let mut visited = HashSet::new();
        let mut stack = vec![Rc::clone(node)];
        
        println!("DFS ->");
        while let Some(node) = stack.pop() {
            let node_ref = node.borrow();
            if visited.insert(node_ref.id) {
                println!("data : {} id : {}", node_ref.data,node_ref.id);
                for neighbor in &node_ref.path {
                    stack.push(Rc::clone(neighbor));
                }
            }
        }
    }

    fn bfs(node: &Node<'a>) {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
    
        visited.insert(node.borrow().id);
        queue.push_back(Rc::clone(node));
    
        println!("BFS ->");
        while let Some(current_node) = queue.pop_front() {
            let current_node_ref = current_node.borrow();
            println!("data : {} id : {}", current_node_ref.data, current_node_ref.id);
    
            for neighbor in &current_node_ref.path {
                if visited.insert(neighbor.borrow().id) {
                    queue.push_back(Rc::clone(neighbor));
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
    node1.borrow_mut().add_node(Rc::clone(&node4));
    let node5 = GraphNode::new(5,vec![Rc::clone(&node4)]);
    node4.borrow_mut().add_node(Rc::clone(&node5));
    let node6 = GraphNode::new(6,Vec::new());
    node5.borrow_mut().add_node(Rc::clone(&node6));

    GraphNode::dfs(&node1);
    GraphNode::bfs(&node1);
}