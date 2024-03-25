use std::rc::Rc;
use std::cell::RefCell;

struct GraphNode<'a>{
    name : String,
    data : u32,
    path : Vec<Node<'a>>
}

type Node<'a> = Rc<RefCell<GraphNode<'a>>>;

impl<'a> GraphNode<'a>{
    
    fn new(name : String, data : u32, path : Vec<Node> ) -> Node{
        Rc::new(RefCell::new(
            GraphNode{
                name,
                data,
                path
            }
        ))
    }
}

fn main(){
    let node1 = GraphNode::new("fist node".to_string(),1,vec![]);
    let node2 = GraphNode::new("second node".to_string(),2,vec![Rc::clone(&node1)]);
    let node3 = GraphNode::new("third node".to_string(),3,vec![Rc::clone(&node1),Rc::clone(&node2)]);
}