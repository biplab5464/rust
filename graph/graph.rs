use std::rc::Rc;
use std::cell::RefCell;

struct GraphNode<'a>{
    data : u32,
    path : Vec<Node<'a>>
}

type Node<'a> = Rc<RefCell<GraphNode<'a>>>;

impl<'a> GraphNode<'a>{
    
    fn new( data : u32, path : Vec<Node> ) -> Node{
        Rc::new(RefCell::new(
            GraphNode{
                data,
                path
            }
        ))
    }

    fn add_node(&mut self,node : Node<'a>){
        self.path.push(Rc::clone(&node));
    }

}

fn main(){
    let node1 = GraphNode::new(1,vec![]);
    let node2 = GraphNode::new(2,vec![Rc::clone(&node1)]);
    let node3 = GraphNode::new(3,vec![Rc::clone(&node1),Rc::clone(&node2)]);
    let node4 = GraphNode::new(4,vec![Rc::clone(&node3)]);
    node1.borrow_mut().add_node(node3);
}