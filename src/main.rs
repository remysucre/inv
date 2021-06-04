use inv::{*, lang::*, inter::*};

use egg::*;

use std::collections::HashSet;
use std::iter::FromIterator;

// init: tc = 0;
// loop: tc = tc . e
// inv:  tc . e = e . tc

pub fn main() {
    // declare variables
    let vs: HashSet<&str> = HashSet::from_iter(
        vec!["tc"].into_iter()
    );

    // initial state
    let ini = vec![
        ("tc", "0"),
    ];

    // loop body
    let p = vec![
        ("tc", "(+ e (* tc e))"),
    ];

    // math axioms and initialization
    let mut rls = rules();
    rls.extend(init(&ini));

    let mut e = Runner::default()
        .with_expr(&"(+ e (* tc e))".parse().unwrap())
        .with_expr(&"(+ e (* e tc))".parse().unwrap())
        .with_scheduler(
            BackoffScheduler::default()
                .rule_match_limit("add-0", 2)
                .rule_match_limit("add-0-rev", 2)
                .rule_match_limit("mul-x-0", 2)
                .rule_match_limit("mul-0-x", 2)
                .rule_ban_length("add-0", 100)
                .rule_ban_length("add-0-rev", 100)
                .rule_ban_length("mul-x-0", 100)
                .rule_ban_length("mul-0-x", 100)
        )
        .with_iter_limit(3)
        .run(&rls)
        .egraph;

    println!("{}", e.total_size());
    e.dot().to_png("init.png").unwrap();

    println!("yo");
    for n in 1..5 {

        let mut rls = rules();
        rls.extend(step(&p));

        let curr_e = Runner::default()
            .with_egraph(e.clone())
            .with_expr(&"(+ e (* step_tc e))".parse().unwrap())
            .with_expr(&"(+ e (* e step_tc))".parse().unwrap())
            .with_scheduler(
                BackoffScheduler::default()
                    .rule_match_limit("add-0", 2)
                    .rule_match_limit("add-0-rev", 2)
                    .rule_match_limit("mul-x-0", 2)
                    .rule_match_limit("mul-0-x", 2)
                    .rule_ban_length("add-0", 100)
                    .rule_ban_length("add-0-rev", 100)
                    .rule_ban_length("mul-x-0", 100)
                    .rule_ban_length("mul-0-x", 100)
            )
            .with_iter_limit(1)
            .run(&rls)
            .egraph;

        let rn_e = &rename(curr_e, &vs);
        println!("done");
        rn_e.dot().to_png(format!("step_{}.5.png", n - 1)).unwrap();

        e = intersect(&e, &rn_e, ());
        println!("inter");

        e.dot().to_png(format!("step_{}.png", n)).unwrap();
    }
}
