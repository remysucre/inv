pub mod lang;
pub mod inter;
use lang::*;

use egg::*;

pub fn init(defs: &[(&str, &str)]) -> Vec<Rewrite<Math, ()>> {
    let mut rls = vec![];
    for (x, e) in defs {
        rls.push(
            Rewrite::new(
                format!("init-{}", x), format!("init-{}", x),
                x.parse::<Pattern<Math>>().unwrap(),
                e.parse::<Pattern<Math>>().unwrap(),
            ).unwrap()
        );
        rls.push(
            Rewrite::new(
                format!("init-{}-rev", x), format!("init-{}-rev", x),
                e.parse::<Pattern<Math>>().unwrap(),
                x.parse::<Pattern<Math>>().unwrap(),
            ).unwrap()
        );
    }
    rls
}

pub fn step(defs: Vec<(&str, &str)>) -> Vec<Rewrite<Math, ()>> {
    let mut rls = vec![];
    for (x, e) in defs {
        rls.push(
            Rewrite::new(
                format!("step-{}", x), format!("step-{}", x),
                e.parse::<Pattern<Math>>().unwrap(),
                format!("step_{}", x).parse::<Pattern<Math>>().unwrap(),
            ).unwrap()
        );
        rls.push(
            Rewrite::new(
                format!("step-{}-rev", x), format!("step-{}-rev", x),
                format!("step_{}", x).parse::<Pattern<Math>>().unwrap(),
                e.parse::<Pattern<Math>>().unwrap(),
            ).unwrap()
        );
    }
    rls
}

pub fn rn(xs: &[(&str, &str)]) -> Vec<Rewrite<Math, ()>> {
    let mut rls = vec![];
    for (x, _) in xs {
        rls.push(
            Rewrite::new(
                format!("rn-{}", x), format!("rn-{}", x),
                x.parse::<Pattern<Math>>().unwrap(),
                format!("step_{}", x).parse::<Pattern<Math>>().unwrap(),
            ).unwrap()
        )
    }
    rls
}
