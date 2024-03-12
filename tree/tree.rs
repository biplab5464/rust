#[derive(Debug)]
struct TreeNode{
    data : u32,
    left : Node,
    right : Node
}

type Node = Option<Box<TreeNode>>;

#[derive(Debug)]
struct Tree{
    root : TreeNode
}

fn main(){

    let mut tree = Tree{
        root : TreeNode{
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
        }
    };

    tree.root.left = Some(Box::new(TreeNode{
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

    let farther_node = &mut tree.root.left.as_mut().unwrap();

    //println!("Tree - {:?}",farther_node.left);
    farther_node.left = Some(Box::new(TreeNode{
        data : 122,
        left : None,
        right : None
        
    }));
    println!("Tree - {:?}",tree);

}
