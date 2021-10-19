use hrm::{Runner, Context, Expression, Tile};

fn statements<'a>() -> Vec<Expression<'a>> {
    vec![
        Expression::Label("inbox"),
        Expression::Inbox,
        Expression::CopyTo(2),
        Expression::Label("next"),
        Expression::CopyFromPointer(2),
        Expression::Outbox,
        Expression::Incr(2),
        Expression::CopyFromPointer(2),
        Expression::JumpIfNegative("inbox"),
        Expression::CopyTo(2),
        Expression::Jump("next"),
    ]
}

fn context_with_inbox(inbox: Vec<Tile>) -> Context {
    Context::new_with_memory(&inbox, vec![
        Some(Tile::Char('E')),
        Some(Tile::Num(13)),
        None,
        Some(Tile::Char('C')),
        Some(Tile::Num(23)),
        None,
        None,
        None,
        None,
        None,
        Some(Tile::Char('P')),
        Some(Tile::Num(20)),
        None,
        Some(Tile::Char('S')),
        Some(Tile::Num(3)),
        None,
        None,
        None,
        None,
        None,
        Some(Tile::Char('E')),
        Some(Tile::Num(-1)),
        None,
        Some(Tile::Char('A')),
        Some(Tile::Num(10)),
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
                Tile::Num(23),
                Tile::Num(0),
            ]),
            pos: 0,
            statements: statements(),
        }.run_debug(),
        vec![
            Tile::Char('A'),
            Tile::Char('P'),
            Tile::Char('E'),
            Tile::Char('E'),
            Tile::Char('S'),
            Tile::Char('C'),
            Tile::Char('A'),
            Tile::Char('P'),
            Tile::Char('E'),
        ],
    )
}
