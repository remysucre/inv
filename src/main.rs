use inv::{*, lang::*, inter::*};

use egg::*;

use std::collections::HashSet;
use std::iter::FromIterator;

// init: tc = 0
// loop: tc = e + e * tc
// inv:  tc * e = e * tc

pub fn main() {
    // initial state
    let i = vec![
        ("x", "1"),
        ("y", "1"),
    ];

    // loop body
    let p = vec![
        ("x", "(+ y y)"),
        ("y", "(+ x x)"),
    ];

    let vs: HashSet<&str> = HashSet::from_iter(
        vec!["x", "y"].into_iter()
    );

    let mut rls = rules();
    rls.extend(init(&i));

    // initialize x and y
    let mut r = Runner::default();

    for (v, _) in &i {
        r.egraph.add_expr(&v.parse().unwrap());
        r.egraph.add_expr(&"(+ x x)".parse().unwrap());
        r.egraph.add_expr(&"(+ y y)".parse().unwrap());
    }

    let mut e = r.run(&rls).egraph;

    for n in 1..5 {

        // loop once
        let mut curr_r = Runner::default().with_egraph(e.clone());

        for (v, _) in &p {
            curr_r.egraph.add_expr(&format!("step_{}", v).parse().unwrap());
            curr_r.egraph.add_expr(&"(+ step_x step_x)".parse().unwrap());
            curr_r.egraph.add_expr(&"(+ step_y step_y)".parse().unwrap());
        }

        let curr_e = rename(curr_r.run(&step(&p)).egraph, &vs);

        e = intersect(&e, &curr_e, ConstantFold);

        println!("{}", e.total_size());
        e.dot().to_png(format!("ast{}.png", n)).unwrap();
    }
}
