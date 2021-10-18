use hrm::{Runner, Context, Expression, Tile};

const INBOX_LETTER: usize = 11;
const VALIDATE_POINTER: usize = 12;
const SEEN_LETTERS_LEN: usize = 13;
const ZERO: usize = 14;
fn statements<'a>() -> Vec<Expression<'a>> {
    vec![
        Expression::CopyFrom(ZERO),
        Expression::CopyTo(SEEN_LETTERS_LEN),
        Expression::Jump("inbox"),
        Expression::Label("outbox"),
        Expression::CopyFrom(INBOX_LETTER),
        Expression::Outbox,
        Expression::CopyFrom(INBOX_LETTER),
        Expression::CopyToPointer(SEEN_LETTERS_LEN),
        Expression::Incr(SEEN_LETTERS_LEN),
        Expression::Label("inbox"),
        Expression::Inbox,
        Expression::CopyTo(INBOX_LETTER),
        Expression::CopyFrom(SEEN_LETTERS_LEN),
        Expression::JumpIfZero("outbox"),
        Expression::CopyFrom(ZERO),
        Expression::CopyTo(VALIDATE_POINTER),
        Expression::Label("next letter"),
        Expression::CopyFrom(INBOX_LETTER),
        Expression::SubPointer(VALIDATE_POINTER),
        Expression::JumpIfZero("inbox"),
        Expression::Incr(VALIDATE_POINTER),
        Expression::Sub(SEEN_LETTERS_LEN),
        Expression::JumpIfZero("outbox"),
        Expression::Jump("next letter"),
    ]
}

fn context_with_inbox(inbox: Vec<Tile>) -> Context {
    let mut context = Context::new(&inbox, 15);
    context.memory[ZERO] = Some(Tile::Num(0));
    context
}

fn main() {
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Char('G'),
                Tile::Char('G'),
                Tile::Char('E'),
                Tile::Char('X'),
                Tile::Char('A'),
                Tile::Char('E'),
                Tile::Char('E'),
                Tile::Char('A'),
                Tile::Char('D'),
                Tile::Char('A'),
            ]),
            pos: 0,
            statements: statements(),
        }.run_debug(),
        vec![
            Tile::Char('G'),
            Tile::Char('E'),
            Tile::Char('X'),
            Tile::Char('A'),
            Tile::Char('D'),
        ],
    )
}
