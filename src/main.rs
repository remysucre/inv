use inv::{*, lang::*, inter::*};

use egg::*;

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

    let mut rls = rules();
    rls.extend(init(&i));

    // initialize x and y
    let mut r0 = Runner::default();

    for (v, _) in &i {
        r0.egraph.add_expr(&v.parse().unwrap());
    }

    let e0 = r0.run(&rls).egraph;

    // rename x => step_x, y => step_y in e0
    let prev_e = Runner::default()
        .with_egraph(e0.clone())
        .run(&rn(&i))
        .egraph;

    // loop once
    let mut r1 = Runner::default().with_egraph(e0);

    for (v, _) in &p {
        r1.egraph.add_expr(&format!("step_{}", v).parse().unwrap());
    }

    let e1 = r1.run(&step(p)).egraph;

    let e_inter = intersect(&prev_e, &e1, ());

    println!("{}", e_inter.total_size());
    e_inter.dot().to_png("ast.png").unwrap();
}

