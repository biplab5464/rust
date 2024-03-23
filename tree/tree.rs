use std::cmp::Ordering;
use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct TreeNode{
    data : u32,
    left : Node,
    right : Node
}

type Node = Option<Box<TreeNode>>;

#[derive(Debug)]
struct Tree{
    root : Node
}

impl Tree{

    fn create_root(data : u32)-> Tree{
        Tree{
            root : Some(Box::new(TreeNode{
            data,
            left : None,
            right : None
        }))}
    }

    fn in_order(&self){
        print!("Tree Inorder-> ");
        Self::in_order_taversal(&self.root);
        println!();
    }
    
    fn pre_order(&self){
        print!("Tree Preorder -> ");
        Self::pre_order_traversal(&self.root);
        println!();
    }
    
    fn post_order(&self){
        print!("Tree Postorder -> ");
        Self::post_order_traversal(&self.root);
        println!();
    }
    
    
    fn in_order_taversal(tree : &Node){
        if let Some(node) = tree{
            Self::in_order_taversal(&node.left);
            print!("{},",node.data);
            Self::in_order_taversal(&node.right);
        }
    }
    
    fn pre_order_traversal(tree : &Node){
        if let Some(node) = tree{
            print!("{},",node.data);
            Self::pre_order_traversal(&node.left);
            Self::pre_order_traversal(&node.right);
        }
    }
    
    fn post_order_traversal(tree : &Node){
        if let Some(node) = tree{
            Self:: post_order_traversal(&node.left);
            Self::post_order_traversal(&node.right);
            print!("{},",node.data);
    }
    }
        }
    }
    
    fn insert(&mut self,ele : u32){
        Self::insert_node(&mut self.root,ele);
    }

    fn insert_node(tree : &mut Node,ele : u32){
        let mut my_node = tree.as_mut().unwrap();
     
        let new_node = Some(Box::new(TreeNode{
            data : ele,
            left : None,
            right : None
        }));
    
        if my_node.data > ele {
            if my_node.left.is_none(){
                my_node.left = new_node;
            }else{
                Self::insert_node(&mut my_node.left,ele);
            }
        }else{
            if my_node.right.is_none(){
                my_node.right = new_node;
            }else{
                Self::insert_node(&mut my_node.right,ele);
            }
        }
    }
    
    fn search(&self,ele : u32) -> bool{
        Self::search_node(&self.root,ele)
    }

    fn search_node(tree : &Node, ele : u32) -> bool{
        if let Some(node) = tree{
            return match ele.cmp(&node.data){
                Ordering::Less => Self::search_node(&node.left,ele),
                Ordering::Equal => true,
                Ordering::Greater => Self::search_node(&node.right,ele),
            }
        }
        false
    }
    
    fn display(&self){
        Self::display_tree(&self.root);
        print!("\n");
    }
    
    fn display_tree(node: &Node) {
        if let Some(ref inner) = node {
            print!("( [{}] ", inner.data); 
            print!(" left :"); 
            Self::display_tree(&inner.left);
            print!(" right :"); 
            Self::display_tree(&inner.right);
            print!(" )"); 
        } else {
            print!(" None");
        }
    }
    
    fn total_node(&self) -> usize {
        let mut count_node : usize = 0;  
        Self::count(&self.root,&mut count_node);
        count_node
    }
    
    fn count(tree : &Node, count_node  : &mut usize){
        if let Some(node) = tree{
            Self::count(&node.left,count_node);
            *count_node +=1;
            Self::count(&node.right,count_node);
        }
    }
    
    fn total_leaf_node( &self ) -> usize {
        let mut count_node : usize = 0;  
        Self::count_leaf(&self.root,&mut count_node);
        count_node
    }
    
    fn count_leaf(tree : &Node, count_node  : &mut usize){
        if let Some(node) = tree{
            Self::count_leaf(&node.left,count_node);
            if node.left.is_none() && node.right.is_none(){
                *count_node +=1;
            }
            Self::count_leaf(&node.right,count_node);
        }
    }
}

fn main(){

    let mut tree = Tree::create_root(53);
    tree.insert(32);
    tree.insert(78);
    tree.insert(45);
    tree.insert(47);
    tree.insert(63);

    //println!("Tree - {:?}",tree);
    tree.display();
    // traversal of tree
    tree.in_order();
    tree.pre_order();
    tree.post_order();

    println!("did you find the ele {}",tree.search(45));
    println!("number of node in the {}",tree.total_node());
    println!("number of leaf node in the {}",tree.total_leaf_node());

}

