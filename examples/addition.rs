extern crate boo;

use boo::Forest;
use std::collections::HashSet;


fn main(){
    let mut forest = Forest::new();

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
    });

    println!("I'm done!");
}
