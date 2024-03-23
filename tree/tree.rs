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

fn main(){

    let mut tree = Tree{
        root : Some(Box::new(TreeNode{
            data : 53,
            left : None,
            right : None
        }))
    };
    insert(&mut tree.root,32);
    insert(&mut tree.root,78);
    insert(&mut tree.root,45);
    insert(&mut tree.root,47);
    insert(&mut tree.root,63);

    //println!("Tree - {:?}",tree);
    display(&tree);
    // traversal of tree
    print!("Tree -> ");
    inOrder(&tree);
    println!();
    println!("did you find the ele {}",search(&tree.root,45));
    println!("number of node in the {}",total_node(&tree));
    println!("number of leaf node in the {}",total_leaf_node(&tree));

}

fn inOrder(tree : &Tree){
    inOrder_taversal(&tree.root);
}

fn preOrder(tree : &Tree){
    preOrder_traversal(&tree.root);
}

fn postOrder(tree : &Tree){
    postOrder_traversal(&tree.root);
}


fn inOrder_taversal(tree : &Node){
    if let Some(node) = tree{
        inOrder_taversal(&node.left);
        print!("{},",node.data);
        inOrder_taversal(&node.right);
    }
}

fn preOrder_traversal(tree : &Node){
    if let Some(node) = tree{
        print!("{},",node.data);
        preOrder_traversal(&node.left);
        preOrder_traversal(&node.right);
    }
}

fn postOrder_traversal(tree : &Node){
    if let Some(node) = tree{
        postOrder_traversal(&node.left);
        postOrder_traversal(&node.right);
        print!("{},",node.data);
    }
}

fn insert(tree : &mut Node,ele : u32){
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
            insert(&mut my_node.left,ele);
        }
    }else{
        if my_node.right.is_none(){
            my_node.right = new_node;
        }else{
            insert(&mut my_node.right,ele);
        }
    }
}

fn search(tree : &Node, ele : u32) -> bool{
    if let Some(node) = tree{
        return match ele.cmp(&node.data){
            Ordering::Less => search(&node.left,ele),
            Ordering::Equal => true,
            Ordering::Greater => search(&node.right,ele),
        }
    }
    false
}

fn display(tree : &Tree){
    display_tree(&tree.root);
    print!("\n");
}

fn display_tree(node: &Node) {
    if let Some(ref inner) = node {
        print!("( [{}] ", inner.data); 
        print!(" left :"); 
        display_tree(&inner.left);
        print!(" right :"); 
        display_tree(&inner.right);
        print!(" )"); 
    } else {
        print!(" None");
    }
}

fn total_node( tree : &Tree ) -> usize {
    let mut count_node : usize = 0;  
    count(&tree.root,&mut count_node);
    count_node
}

fn count(tree : &Node, count_node  : &mut usize){
    if let Some(node) = tree{
        count(&node.left,count_node);
        *count_node +=1;
        count(&node.right,count_node);
    }
}

fn total_leaf_node( tree : &Tree ) -> usize {
    let mut count_node : usize = 0;  
    count_leaf(&tree.root,&mut count_node);
    count_node
}

fn count_leaf(tree : &Node, count_node  : &mut usize){
    if let Some(node) = tree{
        count_leaf(&node.left,count_node);
        if node.left.is_none() && node.right.is_none(){
            *count_node +=1;
        }
        count_leaf(&node.right,count_node);
    }
}