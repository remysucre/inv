use inv::{*, lang::*, inter::*};

use egg::*;

use std::collections::HashSet;
use std::iter::FromIterator;

// init: x = 1; y = 1
// loop: x = y + y; y = x + x
// inv:  x = y

pub fn main() {
    // declare variables
    let vs: HashSet<&str> = HashSet::from_iter(
        vec!["x", "y"].into_iter()
    );

    // initial state
    let ini = vec![
        ("x", "1"),
        ("y", "1"),
    ];

    // loop body
    let p = vec![
        ("x", "(+ y y)"),
        ("y", "(+ x x)"),
    ];

    // math axioms and initialization
    let mut rls = rules();
    rls.extend(init(&ini));

    let mut e = Runner::default()
        .with_expr(&"(+ x x)".parse().unwrap())
        .with_expr(&"(+ y y)".parse().unwrap())
        .run(&rls)
        .egraph;

    e.dot().to_png("init.png").unwrap();

    for n in 1..5 {

        let curr_e = Runner::default()
            .with_egraph(e.clone())
            .with_expr(&"(+ step_x step_x)".parse().unwrap())
            .with_expr(&"(+ step_y step_y)".parse().unwrap())
            .run(&step(&p))
            .egraph;

        e = intersect(&e, &rename(curr_e, &vs), ConstantFold);

        e.dot().to_png(format!("step_{}.png", n)).unwrap();
    }
}
