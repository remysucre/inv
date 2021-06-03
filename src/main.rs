use inv::{*, lang::*, inter::*};

use egg::*;

// x = 1; y = 1
// loop { x = y + y; y = x + x }
// invariant: x = y

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
    let mut r = Runner::default();

    for (v, _) in &i {
        r.egraph.add_expr(&v.parse().unwrap());
    }

    let mut e = r.run(&rls).egraph;

    for n in 1..5 {
        // rename x => step_x, y => step_y in e0
        let prev_e = Runner::default()
            .with_egraph(e.clone())
            .run(&rn(&i))
            .egraph;

        // loop once
        let mut curr_r = Runner::default().with_egraph(e);

        for (v, _) in &p {
            curr_r.egraph.add_expr(&format!("step_{}", v).parse().unwrap());
        }

        let curr_e = curr_r.run(&step(&p)).egraph;

        // find common equalities
        let e_inter = intersect(&prev_e, &curr_e, ConstantFold);

        println!("{}", e_inter.total_size());
        e_inter.dot().to_png(format!("ast{}.png", n)).unwrap();

        e = e_inter;
    }
}
