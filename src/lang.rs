use egg::{rewrite as rw, *};

pub type Constant = i32;
pub type EGraph = egg::EGraph<Math, ConstantFold>;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;

define_language! {
    pub enum Math {
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),
        Num(i32),
        Symbol(Symbol),
        Other(Symbol, Vec<Id>),
    }
}

#[derive(Default, Clone)]
pub struct ConstantFold;
impl Analysis<Math> for ConstantFold {
    type Data = Option<Constant>;

    fn make(egraph: &EGraph, enode: &Math) -> Self::Data {
        let x = |i: &Id| egraph[*i].data;
        Some(match enode {
            Math::Num(c) => *c,
            Math::Add([a, b]) => x(a)? + x(b)?,
            Math::Mul([a, b]) => x(a)? * x(b)?,
            _ => return None,
        })
    }

    fn merge(&self, to: &mut Self::Data, from: Self::Data) -> bool {
        if let (Some(c1), Some(c2)) = (to.as_ref(), from.as_ref()) {
            assert_eq!(c1, c2);
        }
        merge_if_different(to, to.or(from))
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        let class = &mut egraph[id];
        if let Some(c) = class.data {
            let added = egraph.add(Math::Num(c));
            let (id, _did_something) = egraph.union(id, added);
            // to not prune, comment this out
            // egraph[id].nodes.retain(|n| n.is_leaf());

            assert!(
                !egraph[id].nodes.is_empty(),
                "empty eclass! {:#?}",
                egraph[id]
            );
            #[cfg(debug_assertions)]
            egraph[id].assert_unique_leaves();
        }
    }
}

pub fn rules() -> Vec<Rewrite> {
    let mut rls = vec![
        rw!("commute-add"; "(+ ?x ?y)"      <=> "(+ ?y ?x)"),
        rw!("assoc-add"; "(+ (+ ?x ?y) ?z)" <=> "(+ ?x (+ ?y ?z))"),
        rw!("commute-mul"; "(* ?x ?y)"      <=> "(* ?y ?x)"),
        rw!("assoc-mul"; "(* (* ?x ?y) ?z)" <=> "(* ?x (* ?y ?z))"),
        rw!("add-0"; "(+ ?x 0)" <=> "?x"),
        rw!("mul-1"; "(* ?x 1)" <=> "?x"),
    ].concat();
    rls.extend(vec![
        rw!("mul-0"; "(* ?x 0)" => "0"),
    ]);
    rls
}
