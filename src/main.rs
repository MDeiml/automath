extern crate attomath;

use attomath::{expression::Expression, statement::Statement, theorem::Theorem};

fn main() {
    let wff1 = Theorem::new(
        Statement {
            judgement: 0,
            expression: Expression::from_raw(vec![-2, 0, 1].into_boxed_slice()).unwrap(),
        },
        vec![
            Statement {
                judgement: 0,
                expression: Expression::from_raw(vec![0].into_boxed_slice()).unwrap(),
            },
            Statement {
                judgement: 0,
                expression: Expression::from_raw(vec![1].into_boxed_slice()).unwrap(),
            },
        ],
        vec![],
    );
    let wff2 = Theorem::new(
        Statement {
            judgement: 0,
            expression: Expression::from_raw(vec![-3, 0, -1].into_boxed_slice()).unwrap(),
        },
        vec![Statement {
            judgement: 0,
            expression: Expression::from_raw(vec![0].into_boxed_slice()).unwrap(),
        }],
        vec![],
    );
    let ax1 = Theorem::new(
        Statement {
            judgement: 1,
            expression: Expression::from_raw(vec![-2, 0, -2, 1, 0].into_boxed_slice()).unwrap(),
        },
        vec![
            Statement {
                judgement: 0,
                expression: Expression::from_raw(vec![0].into_boxed_slice()).unwrap(),
            },
            Statement {
                judgement: 0,
                expression: Expression::from_raw(vec![1].into_boxed_slice()).unwrap(),
            },
        ],
        vec![],
    );
    let ax2 = Theorem::new(
        Statement {
            judgement: 1,
            expression: Expression::from_raw(
                vec![-2, -2, 0, -2, 1, 2, -2, -2, 0, 1, -2, 1, 2].into_boxed_slice(),
            )
            .unwrap(),
        },
        vec![
            Statement {
                judgement: 0,
                expression: Expression::from_raw(vec![0].into_boxed_slice()).unwrap(),
            },
            Statement {
                judgement: 0,
                expression: Expression::from_raw(vec![1].into_boxed_slice()).unwrap(),
            },
            Statement {
                judgement: 0,
                expression: Expression::from_raw(vec![2].into_boxed_slice()).unwrap(),
            },
        ],
        vec![],
    );
    let ax3 = Theorem::new(
        Statement {
            judgement: 1,
            expression: Expression::from_raw(
                vec![-2, -2, -3, 0, -1, -3, 1, -1, -2, 1, 0].into_boxed_slice(),
            )
            .unwrap(),
        },
        vec![
            Statement {
                judgement: 0,
                expression: Expression::from_raw(vec![0].into_boxed_slice()).unwrap(),
            },
            Statement {
                judgement: 0,
                expression: Expression::from_raw(vec![1].into_boxed_slice()).unwrap(),
            },
        ],
        vec![],
    );
    let ax_mp = Theorem::new(
        Statement {
            judgement: 1,
            expression: Expression::from_raw(vec![1].into_boxed_slice()).unwrap(),
        },
        vec![
            Statement {
                judgement: 1,
                expression: Expression::from_raw(vec![0].into_boxed_slice()).unwrap(),
            },
            Statement {
                judgement: 1,
                expression: Expression::from_raw(vec![-2, 0, 1].into_boxed_slice()).unwrap(),
            },
        ],
        vec![],
    );
    let mut db = Vec::new();
    db.push(wff1);
    db.push(wff2);
    db.push(ax1);
    db.push(ax2);
    db.push(ax3);
    db.push(ax_mp);
}
