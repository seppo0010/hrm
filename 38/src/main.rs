use hrm::{Runner, Context, Expression, Tile};

const ZERO: usize = 9;
const TEN: usize = 10;
const HUNDRED: usize = 11;
const COUNTER: usize = 8;
const NUMBER: usize = 7;

fn statements<'a>() -> Vec<Expression<'a>> {
    vec![
        Expression::Label("new inbox"),
        Expression::CopyFrom(ZERO),
        Expression::CopyTo(COUNTER),
        Expression::Inbox,
        Expression::CopyTo(NUMBER),
        Expression::Sub(TEN),
        Expression::JumpIfNegative("unit"),

        // hundreds
        Expression::Label("another hundred"),
        Expression::CopyFrom(NUMBER),
        Expression::Sub(HUNDRED),
        Expression::JumpIfNegative("tens"),
        Expression::CopyTo(NUMBER),
        Expression::Incr(COUNTER),
        Expression::Jump("another hundred"),
        Expression::Label("tens"),
        Expression::CopyFrom(COUNTER),
        Expression::JumpIfZero("another tens"),
        Expression::Outbox,

        // reset
        Expression::CopyFrom(ZERO),
        Expression::CopyTo(COUNTER),

        // tens
        Expression::Label("another tens"),
        Expression::CopyFrom(NUMBER),
        Expression::Sub(TEN),
        Expression::JumpIfNegative("units"),
        Expression::CopyTo(NUMBER),
        Expression::Incr(COUNTER),
        Expression::Jump("another tens"),
        Expression::Label("units"),
        Expression::CopyFrom(COUNTER),
        Expression::Outbox,

        // units
        Expression::Label("unit"),
        Expression::CopyFrom(NUMBER),
        Expression::Outbox,
        Expression::Jump("new inbox"),
    ]
}

fn context_with_inbox(inbox: Vec<Tile>) -> Context {
    Context::new_with_memory(&inbox, vec![
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Tile::Num(0)),
        Some(Tile::Num(10)),
        Some(Tile::Num(100)),
    ])
}

fn main() {
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Num(123),
                Tile::Num(98),
                Tile::Num(208),
                Tile::Num(3),
            ]),
            pos: 0,
            statements: statements(),
        }.run_debug(),
        vec![
            Tile::Num(1),
            Tile::Num(2),
            Tile::Num(3),
            Tile::Num(9),
            Tile::Num(8),
            Tile::Num(2),
            Tile::Num(0),
            Tile::Num(8),
            Tile::Num(3),
        ],
    )
}
