extern crate boo;

use boo::Forest;
use std::collections::HashSet;


fn main(){
    let mut forest = Forest::new();

    forest.with(|f|{
        let _false = f.constant(false);
        let _true = f.constant(true);

        println!("false == {}", f.evaluate(_false, &HashSet::new()));
        println!("true == {}", f.evaluate(_true, &HashSet::new()));
    });

    println!("I'm done!");
}
