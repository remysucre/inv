use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use egg::{rewrite as rw, *};

// This program discovers loop invariants.
// The central idea is to execute the loop *abstractly*
// over an e-graph.
// Doing so, we see how equalities evolve across
// loop iterations.
// Any *loop invariant* would be preserved at every iteration,
// so to discover these invariants, we look for equalities
// that re-emerge at every iteration.
// We achieve that by *intersecting* the e-graphs at
// adjacent iterations of the abstract execution.
// When the intersection stops changing,
// it contains all equalities that are invariant for the loop.

// let x = 1; y = 1
// loop { x = y + y; y = x + x; }
// invariant: x = y

define_language! {
    pub enum Math {
        "+" = Add([Id; 2]),
        Num(i32),
        Symbol(Symbol),
        Other(Symbol, Vec<Id>),
    }
}

fn rules() -> Vec<Rewrite<Math, ()>> {
    let mut rls = vec![];
    // Standard math identities.
    rls.extend(
        vec![
            rw!("commute-add"; "(+ ?x ?y)"      <=> "(+ ?y ?x)"),
            rw!("assoc-add"; "(+ (+ ?x ?y) ?z)" <=> "(+ ?x (+ ?y ?z))"),
        ].concat()
    );
    // The initial state of x & y.
    rls.extend(
        vec![
            rw!("init-x"; "(x 0)" <=> "1"),
            rw!("init-y"; "(y 0)" <=> "1"),
        ].concat()
    );
    rls
}

fn step() -> Vec<Rewrite<Math, ()>> {
    // Taking one iteration of the loop.
    // In general this should be (+ (y ?n) (y ?n)) <=> (x (+ 1 ?n)),
    // which would require constant folding.
    vec![
        rw!("step-x"; "(+ (y 0) (y 0))" <=> "(x 1)"),
        rw!("step-y"; "(+ (x 0) (x 0))" <=> "(y 1)"),
    ].concat()
}

// We want to find what equalities are shared across iterations,
// e.g. if x_j = y_j for both j = i and j = i+1,
// so we need to rename x_i and x_{i+1} to be the same.
// Here I hardcode again, but in general we should have
// rn_i: (x i) => xx; (y i) => yy
fn rn_0() -> Vec<Rewrite<Math, ()>> {
    vec![
        rw!("rnx"; "(x 0)" => "xx"),
        rw!("rny"; "(y 0)" => "yy"),
    ]
}

fn rn_1() -> Vec<Rewrite<Math, ()>> {
    vec![
        rw!("rnx"; "(x 1)" => "xx"),
        rw!("rny"; "(y 1)" => "yy"),
    ]
}

pub fn main() {
    // Initialize x_0 and y_0 to both be 1.
    let r0 = Runner::default()
        .with_expr(&"(x 0)".parse().unwrap())
        .with_expr(&"(y 0)".parse().unwrap())
        .run(&rules());
    let e0 = r0.egraph;

    // Take 1 step of the loop.
    let r1 = Runner::default()
        .with_egraph(e0.clone())
        .with_expr(&"(x 1)".parse().unwrap())
        .with_expr(&"(y 1)".parse().unwrap())
        .run(&step());
    let e1 = r1.egraph;

    // Rename x_0 => xx, y_0 => yy in e_0
    let e0_ = Runner::default()
        .with_egraph(e0)
        .run(&rn_0())
        .egraph;

    // Rename x_1 => xx, y_1 => yy in e_1
    let e1_ = Runner::default()
        .with_egraph(e1)
        .run(&rn_1())
        .egraph;

    // See what equalities are preserved across iterations.
    // Here I only take 1 step of the loop;
    // in general we can step until the surviving equalities stop changing,
    // in which case they are truly invariants.
    let e = intersect(&e0_, &e1_, ());

    println!("{}", e.total_size());
    e.dot().to_png("ast.png").unwrap();
    // The graph should contain xx = yy, which is an invariant.
    // The other equalities should be trimmed,
    // because they are only about x_1 and y_1 and are not invariant.
}

fn intersect<L, A>(g1: &EGraph<L, A>, g2: &EGraph<L, A>, analysis: A) -> EGraph<L, A>
where
    L: Language,
    A: Analysis<L>,
{
    let mut g = EGraph::new(analysis);
    let mut e1_e: HashMap<Id, HashSet<Id>> = HashMap::new();
    let mut e_e2: HashMap<Id, Id> = HashMap::new();
    let empty_set = HashSet::new();
    loop {
        let mut g_changed = false;
        for class in g1.classes() {
            for node in &class.nodes {
                for mut n_new in flatmap_children(node, |id| {
                    e1_e.get(&id).unwrap_or(&empty_set).iter().copied()
                }) {
                    if let Some(c2) = g2.lookup(n_new.clone().map_children(|id| e_e2[&id])) {
                        let c_new = g.lookup(&mut n_new).unwrap_or_else(|| {
                            g_changed = true;
                            g.add(n_new.clone())
                        });
                        e_e2.insert(c_new, c2);
                        e1_e.entry(class.id).or_insert(HashSet::new()).insert(c_new);
                        for c in e1_e[&class.id].iter() {
                            if g2.find(e_e2[&c]) == g2.find(c2) {
                                let unioned = g.union(c_new, *c).1;
                                g_changed = g_changed || unioned;
                                g.rebuild();
                            }
                        }
                    }
                }
            }
        }
        if !g_changed {
            break;
        }
    }
    g
}

// compute the set of new nodes op(c1',...,cn') from op(c1,...,cn)
// let op(c1,...,cn) = node;
// vec![op(c1',...,cn') where c1' in f(c1),...,cn' in f(cn)]
fn flatmap_children<L, F, I>(node: &L, f: F) -> Vec<L>
where
    L: Language,
    I: Clone + Iterator<Item = Id>,
    F: Fn(Id) -> I,
{
    if node.children().is_empty() {
        vec![node.clone()]
    } else {
        let childrens = node
            .children()
            .iter()
            .map(|id| f(*id))
            .multi_cartesian_product();
        childrens
            .map(|children| {
                let mut new_node = node.clone();
                for i in 0..children.len() {
                    new_node.children_mut()[i] = children[i];
                }
                new_node
            })
            .collect()
    }
}
