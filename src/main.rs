use egg::{rewrite as rw, *};

// (tc0 x y) = 0
// (tc x y) = (+ (e x y) (* (e x z) (tc z y)))

// tc = (+ e (* e tc))

// tc0 = 0
// tc1 = e
// tc2 = e + e^2
// tc3 = e + e^2 + e^3

define_language! {
    pub enum CSr {
        // binary ops
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),

        // constants & variables
        Num(i32),
        Symbol(Symbol),

        // uninterpreted functions
        Other(Symbol, Vec<Id>),
    }
}

fn rules() -> Vec<Rewrite<CSr, ()>> {
    let mut rls = vec![
        rw!("add-0"; "(+ ?x 0)" => "?x"),
        rw!("mul-0-r"; "(* ?x 0)" => "0"),
        rw!("mul-1-r"; "(* ?x 1)" => "?x"),
    ];
    rls.extend(
        vec![
            rw!("commute-add"; "(+ ?x ?y)"      <=> "(+ ?y ?x)"),
            rw!("commute-mul"; "(* ?x ?y)"      <=> "(+ ?y ?x)"),
            rw!("assoc-add"; "(+ (+ ?x ?y) ?z)" <=> "(+ ?x (+ ?y ?z))"),
            rw!("assoc-mul"; "(* (* ?x ?y) ?z)" <=> "(* ?x (* ?y ?z))"),
        ].concat()
    );
    rls.extend(
        vec![
            rw!("init"; "(tc0 ?a ?b)" => "0"),
        ]
    );
    rls
}

fn step() -> Vec<Rewrite<CSr, ()>> {
    vec![
        rw!("tc"; "(+ (e ?x ?y) (* (e ?x ?z) (tc0 ?z ?y)))" => "(tc1 ?x ?y)")
    ]
}

pub fn main() {
    let start = "(+ (e x y) (* (e x z) (tc0 z y)))".parse().unwrap();
    let mut runner = Runner::default().with_expr(&start);

    runner.egraph.add_expr(&"(+ (e y z) (* (e y x) (tc0 x z)))".parse().unwrap());
    runner.egraph.add_expr(&"(+ (e z x) (* (e z y) (tc0 y x)))".parse().unwrap());

    runner.egraph.add_expr(&"(+ (e z y) (* (e z x) (tc0 x y)))".parse().unwrap());
    runner.egraph.add_expr(&"(+ (e x z) (* (e x y) (tc0 y z)))".parse().unwrap());
    runner.egraph.add_expr(&"(+ (e y x) (* (e y z) (tc0 z x)))".parse().unwrap());

    let runner = runner.run(&rules());
    println!("{}", runner.egraph.total_size());
    runner.egraph.dot().to_png("ast.png").unwrap();
}
