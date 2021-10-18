use hrm::{Runner, Context, Expression, Tile};

fn statements<'a>() -> Vec<Expression<'a>> {
    vec![
        Expression::Label("start"),
        Expression::Inbox,
        Expression::CopyTo(19),
        Expression::Label("send"),
        Expression::CopyFromPointer(19),
        Expression::JumpIfZero("start"),
        Expression::Outbox,
        Expression::Incr(19),
        Expression::Jump("send"),
    ]
}

fn context_with_inbox(inbox: Vec<Tile>) -> Context {
    Context::new_with_memory(&inbox, vec![
        Some(Tile::Char('G')),
        Some(Tile::Char('E')),
        Some(Tile::Char('T')),
        Some(Tile::Num(0)),
        Some(Tile::Char('T')),
        Some(Tile::Char('H')),
        Some(Tile::Num(0)),
        Some(Tile::Char('A')),
        Some(Tile::Char('W')),
        Some(Tile::Char('A')),
        Some(Tile::Char('K')),
        Some(Tile::Char('E')),
        Some(Tile::Num(0)),
        Some(Tile::Char('I')),
        Some(Tile::Char('S')),
        Some(Tile::Num(0)),
        Some(Tile::Char('X')),
        Some(Tile::Char('X')),
        Some(Tile::Char('X')),
        Some(Tile::Num(0)),
        None,
    ])
}

fn main() {
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Num(0),
                Tile::Num(1),
            ]),
            pos: 0,
            statements: statements(),
        }.run(),
        vec![
            Tile::Char('G'),
            Tile::Char('E'),
            Tile::Char('T'),
            Tile::Char('E'),
            Tile::Char('T'),
        ],
    )
}
