enum List {
    Pair(i32, Box<List>),
    Nil,
}

use List::{Pair, Nil};

fn main() {
    let list = Pair(1, 
        Box::new(Pair(2, 
            Box::new(Pair(3, 
                Box::new(Pair(4, 
                    Box::new(Pair(5, 
                        Box::new(Pair(6, 
                            Box::new(Nil))))))))))));
}

