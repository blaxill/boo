extern crate boo;

use boo::{Forest, Tree};
use std::collections::HashSet;
use std::rc::Rc;

fn main(){
    let mut forest = Forest::new();

    let mut faf: Rc<Tree> = forest.new_empty_tree();
    let mut fat: Rc<Tree> = forest.new_empty_tree();
    let mut taf: Rc<Tree> = forest.new_empty_tree();
    let mut tat: Rc<Tree> = forest.new_empty_tree();

    forest.with(|f|{
        let _false = f.constant(false);
        let _true = f.constant(true);

        let _false_add_false = f.add(_false, _false);
        let _true_add_false = f.add(_true, _false);
        let _false_add_true = f.add(_false, _true);
        let _true_add_true = f.add(_true, _true);

        println!("false + false == {}", f.evaluate(_false_add_false, &HashSet::new()));
        println!("true  + false == {}", f.evaluate(_true_add_false, &HashSet::new()));
        println!("false + true  == {}", f.evaluate(_false_add_true, &HashSet::new()));
        println!("true  + true  == {}", f.evaluate(_true_add_true, &HashSet::new()));

        faf = f.save_as_tree(_false_add_false);
        taf = f.save_as_tree(_true_add_false);
        fat = f.save_as_tree(_false_add_true);
        tat = f.save_as_tree(_true_add_true);
    });

    println!("");
    println!("Releasing unreferenced resources and entering a new context.");
    forest.release_unreferenced_nodes();
    println!("");

    forest.with(|f|{
        let _false_add_false = f.tree_as_node(&faf);
        let _true_add_false = f.tree_as_node(&taf);
        let _false_add_true = f.tree_as_node(&fat);
        let _true_add_true = f.tree_as_node(&tat);

        println!("false + false == {}", f.evaluate(_false_add_false, &HashSet::new()));
        println!("true  + false == {}", f.evaluate(_true_add_false, &HashSet::new()));
        println!("false + true  == {}", f.evaluate(_false_add_true, &HashSet::new()));
        println!("true  + true  == {}", f.evaluate(_true_add_true, &HashSet::new()));


    });

    println!("I'm done!");
}
