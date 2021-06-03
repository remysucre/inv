use egg::{rewrite as rw, *};

define_language! {
    pub enum Math {
        "+" = Add([Id; 2]),
        Num(i32),
        Symbol(Symbol),
        Other(Symbol, Vec<Id>),
    }
}

pub fn rules() -> Vec<Rewrite<Math, ()>> {
    vec![
        rw!("commute-add"; "(+ ?x ?y)"      <=> "(+ ?y ?x)"),
        rw!("assoc-add"; "(+ (+ ?x ?y) ?z)" <=> "(+ ?x (+ ?y ?z))"),
    ].concat()
}
