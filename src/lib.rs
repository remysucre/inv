pub mod lang;
pub mod inter;
use lang::*;

use egg::*;

pub fn init(defs: &[(&str, &str)]) -> Vec<lang::Rewrite> {
    let mut rls = vec![];
    for (x, e) in defs {
        rls.push(
            egg::Rewrite::new(
                format!("init-{}", x), format!("init-{}", x),
                x.parse::<Pattern<Math>>().unwrap(),
                e.parse::<Pattern<Math>>().unwrap(),
            ).unwrap()
        );
        rls.push(
            egg::Rewrite::new(
                format!("init-{}-rev", x), format!("init-{}-rev", x),
                e.parse::<Pattern<Math>>().unwrap(),
                x.parse::<Pattern<Math>>().unwrap(),
            ).unwrap()
        );
    }
    rls
}

pub fn step(defs: &[(&str, &str)]) -> Vec<lang::Rewrite> {
    let mut rls = vec![];
    for (x, e) in defs {
        rls.push(
            egg::Rewrite::new(
                format!("step-{}", x), format!("step-{}", x),
                e.parse::<Pattern<Math>>().unwrap(),
                format!("step_{}", x).parse::<Pattern<Math>>().unwrap(),
            ).unwrap()
        );
        rls.push(
            egg::Rewrite::new(
                format!("step-{}-rev", x), format!("step-{}-rev", x),
                format!("step_{}", x).parse::<Pattern<Math>>().unwrap(),
                e.parse::<Pattern<Math>>().unwrap(),
            ).unwrap()
        );
    }
    rls
}

pub fn rn(xs: &[(&str, &str)]) -> Vec<lang::Rewrite> {
    let mut rls = vec![];
    for (x, _) in xs {
        rls.push(
            egg::Rewrite::new(
                format!("rn-{}", x), format!("rn-{}", x),
                x.parse::<Pattern<Math>>().unwrap(),
                Destroy {
                    e: format!("step_{}", x).parse::<Pattern<Math>>().unwrap(),
                    x: Math::Symbol(x.parse().unwrap()),
                }
            ).unwrap()
        )
    }
    rls
}

pub struct Destroy<A: Applier<Math, ConstantFold>> {
    e: A,
    x: Math,
}

impl<A: Applier<Math, ConstantFold>> Applier<Math, ConstantFold> for Destroy<A> {
    fn apply_one(&self, egraph: &mut lang::EGraph, eclass: Id, subst: &Subst) -> Vec<Id> {
        egraph[eclass].nodes.retain(|node| node != &self.x);
        self.e.apply_one(egraph, eclass, subst)
    }
}

// pub fn forget(xs: &[(&str, &str)]) -> Vec<Rewrite<Math, ()>> {
//     let mut rls = vec![];
//     for (x, _) in xs {
//         rls.push(
//             Rewrite::new(
//                 format!("rn-{}", x), format!("rn-{}", x),
//                 x.parse::<Pattern<Math>>().unwrap(),
//                 Destroy {
//                     e: "null".parse::<Pattern<Math>>().unwrap(),
//                     x: Math::Symbol(x.parse().unwrap()),
//                 }
//             ).unwrap()
//         )
//     }
//     rls
// }
