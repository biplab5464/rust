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
            data : 45,
            left : None,
            right : Some(Box::new(TreeNode{
                data : 23,
                left : None,
                right : Some(Box::new(TreeNode{
                    data : 36,
                    left : None,
                    right : None
                }))
            }))
        }))
    };

    tree.root.as_mut().unwrap().left = Some(Box::new(TreeNode{
        data : 72,
        left : Some(Box::new(TreeNode{
            data : 89,
            left : None,
            right : None
        })),
        right : Some(Box::new(TreeNode{
            data : 43,
            left : None,
            right : None
        }))
    }));

    let farther_node = &mut tree.root.as_mut().unwrap().left.as_mut().unwrap();

    //println!("Tree - {:?}",farther_node.left);
    farther_node.left = Some(Box::new(TreeNode{
        data : 122,
        left : None,
        right : None
        
    }));
    println!("Tree - {:?}",tree);


    // traversal of tree
    print!("Tree -> ");
    inOrder(&tree.root);
    println!()

}

fn inOrder(tree : &Node){
    if let Some(node) = tree{
        inOrder(&node.left);
        print!("{},",node.data);
        inOrder(&node.right);
    }
}

fn preOrder(tree : &Node){
    if let Some(node) = tree{
        print!("{},",node.data);
        inOrder(&node.left);
        inOrder(&node.right);
    }
}

fn postOrder(tree : &Node){
    if let Some(node) = tree{
        inOrder(&node.left);
        inOrder(&node.right);
        print!("{},",node.data);
    }
}