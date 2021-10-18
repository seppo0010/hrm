use hrm::{Runner, Context, Expression, Tile};


const ZERO: usize = 14;
const COUNTER: usize = 15;
const LETTER_POINTER: usize = 16;
const LETTER: usize = 17;

fn statements<'a>() -> Vec<Expression<'a>> {
    vec![
        Expression::Jump("start"),
        Expression::Label("newinbox"),
        Expression::CopyFrom(COUNTER),
        Expression::Outbox,
        Expression::Label("start"),
        Expression::CopyFrom(ZERO),
        Expression::CopyTo(COUNTER),
        Expression::CopyTo(LETTER_POINTER),
        Expression::Inbox,
        Expression::CopyTo(LETTER),
        Expression::Decr(LETTER_POINTER),
        Expression::Label("newletter"),
        Expression::Incr(LETTER_POINTER),
        Expression::CopyFromPointer(LETTER_POINTER),
        Expression::JumpIfZero("newinbox"),
        Expression::Sub(LETTER),
        Expression::JumpIfZero("incr"),
        Expression::Jump("newletter"),
        Expression::Label("incr"),
        Expression::Incr(COUNTER),
        Expression::Jump("newletter"),
    ]
}

fn context_with_inbox(inbox: Vec<Tile>) -> Context {
    Context::new_with_memory(&inbox, vec![
        Some(Tile::Char('G')),
        Some(Tile::Char('E')),
        Some(Tile::Char('T')),
        Some(Tile::Char('H')),
        Some(Tile::Char('A')),
        Some(Tile::Char('W')),
        Some(Tile::Char('A')),
        Some(Tile::Char('K')),
        Some(Tile::Char('E')),
        Some(Tile::Char('S')),
        Some(Tile::Char('X')),
        Some(Tile::Char('X')),
        Some(Tile::Char('X')),
        Some(Tile::Char('R')),
        Some(Tile::Num(0)),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ])
}

fn main() {
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Char('G'),
                Tile::Char('X'),
                Tile::Char('A'),
                Tile::Char('D'),
            ]),
            pos: 0,
            statements: statements(),
        }.run(),
        vec![
            Tile::Num(1),
            Tile::Num(3),
            Tile::Num(2),
            Tile::Num(0),
        ],
    )
}
