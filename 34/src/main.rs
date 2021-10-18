use hrm::{Runner, Context, Expression, Tile};

fn statements<'a>() -> Vec<Expression<'a>> {
    vec![
        Expression::Jump("start"),
        Expression::Label("outbox"),
        Expression::CopyFrom(7),
        Expression::Outbox,
        Expression::Label("start"),
        Expression::CopyFrom(5),
        Expression::CopyTo(6), // check next letter pointer
        Expression::Inbox,
        Expression::CopyTo(7),
        Expression::Label("nextletter"),
        Expression::CopyFromPointer(6),
        Expression::JumpIfZero("outbox"),
        Expression::Sub(7),
        Expression::JumpIfZero("start"),
        Expression::Incr(6),
        Expression::Jump("nextletter"),
    ]
}

fn context_with_inbox(inbox: Vec<Tile>) -> Context {
    Context::new_with_memory(&inbox, vec![
        Some(Tile::Char('A')),
        Some(Tile::Char('E')),
        Some(Tile::Char('I')),
        Some(Tile::Char('O')),
        Some(Tile::Char('U')),
        Some(Tile::Num(0)),
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
                Tile::Char('E'),
                Tile::Char('X'),
                Tile::Char('A'),
                Tile::Char('D'),
            ]),
            pos: 0,
            statements: statements(),
        }.run_debug(),
        vec![
            Tile::Char('G'),
            Tile::Char('X'),
            Tile::Char('D'),
        ],
    )
}
