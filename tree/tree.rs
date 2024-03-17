use std::cmp::Ordering;

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

    println!("Tree - {:?}",tree);
    // traversal of tree
    print!("Tree -> ");
    inOrder(&tree);
    println!();
    println!("did you find the ele {}",search(&tree.root,45));

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
