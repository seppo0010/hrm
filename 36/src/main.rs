use hrm::{Runner, Context, Expression, Tile};

const TMP3: usize = 20;
const TMP2: usize = 21;
const TMP: usize = 22;
const ZERO: usize = 23;
const TEN: usize = 24;
fn statements<'a>() -> Vec<Expression<'a>> {
    vec![
        // get words from inbox
        // 0-9 and 10-21 will have a zero terminated string
        Expression::CopyFrom(ZERO),
        Expression::CopyTo(TMP),
        Expression::Label("get next letter w1"),
        Expression::Inbox,
        Expression::CopyToPointer(TMP),
        Expression::JumpIfZero("copy second word"),
        Expression::Incr(TMP),
        Expression::Jump("get next letter w1"),
        Expression::Label("copy second word"),
        Expression::CopyFrom(TEN),
        Expression::CopyTo(TMP),
        Expression::Label("get next letter w2"),
        Expression::Inbox,
        Expression::CopyToPointer(TMP),
        Expression::JumpIfZero("cmp"),
        Expression::Incr(TMP),
        Expression::Jump("get next letter w2"),
        Expression::Label("cmp"),

        // do cmp
        Expression::CopyFrom(ZERO),
        Expression::CopyTo(TMP), // index of letter to compare
        Expression::Decr(TMP),
        Expression::Label("cmp next letter"),
        Expression::Incr(TMP),
        Expression::CopyFromPointer(TMP),
        Expression::CopyTo(TMP2), // first word's letter
        Expression::JumpIfZero("outbox first word"),
        Expression::CopyFrom(TEN),
        Expression::Add(TMP),
        Expression::CopyTo(TMP3), // index of letter to compare in second word
        Expression::CopyFromPointer(TMP3),
        Expression::JumpIfZero("outbox second word"),
        Expression::Sub(TMP2),
        Expression::JumpIfNegative("outbox second word"),
        Expression::JumpIfZero("cmp next letter"),
        Expression::Jump("outbox first word"),

        Expression::Label("outbox second word"),
        Expression::CopyFrom(TEN),
        Expression::CopyTo(TMP),
        Expression::Jump("copy next letter"),
        Expression::Label("outbox first word"),
        Expression::CopyFrom(ZERO),
        Expression::CopyTo(TMP),
        Expression::Label("copy next letter"),
        Expression::CopyFromPointer(TMP),
        Expression::JumpIfZero("end"),
        Expression::Outbox,
        Expression::Incr(TMP),
        Expression::Jump("copy next letter"),
        Expression::Label("end"),
    ]
}

fn context_with_inbox(inbox: Vec<Tile>) -> Context {
    let mut context = Context::new(&inbox, 25);
    context.memory[ZERO] = Some(Tile::Num(0));
    context.memory[TEN] = Some(Tile::Num(10));
    context
}

fn main() {
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Char('U'),
                Tile::Char('N'),
                Tile::Char('T'),
                Tile::Char('I'),
                Tile::Num(0),
                Tile::Char('U'),
                Tile::Char('N'),
                Tile::Char('I'),
                Tile::Char('X'),
                Tile::Num(0),
            ]),
            pos: 0,
            statements: statements(),
        }.run(),
        vec![
            Tile::Char('U'),
            Tile::Char('N'),
            Tile::Char('I'),
            Tile::Char('X'),
        ],
    );
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Char('U'),
                Tile::Char('N'),
                Tile::Char('I'),
                Tile::Char('X'),
                Tile::Num(0),
                Tile::Char('U'),
                Tile::Char('N'),
                Tile::Char('T'),
                Tile::Char('I'),
                Tile::Num(0),
            ]),
            pos: 0,
            statements: statements(),
        }.run(),
        vec![
            Tile::Char('U'),
            Tile::Char('N'),
            Tile::Char('I'),
            Tile::Char('X'),
        ],
    );
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Char('U'),
                Tile::Char('N'),
                Tile::Char('I'),
                Tile::Char('X'),
                Tile::Num(0),
                Tile::Char('U'),
                Tile::Char('N'),
                Tile::Char('I'),
                Tile::Char('X'),
                Tile::Char('E'),
                Tile::Num(0),
            ]),
            pos: 0,
            statements: statements(),
        }.run(),
        vec![
            Tile::Char('U'),
            Tile::Char('N'),
            Tile::Char('I'),
            Tile::Char('X'),
        ],
    );
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Char('U'),
                Tile::Char('N'),
                Tile::Char('I'),
                Tile::Char('X'),
                Tile::Char('E'),
                Tile::Num(0),
                Tile::Char('U'),
                Tile::Char('N'),
                Tile::Char('I'),
                Tile::Char('X'),
                Tile::Num(0),
            ]),
            pos: 0,
            statements: statements(),
        }.run(),
        vec![
            Tile::Char('U'),
            Tile::Char('N'),
            Tile::Char('I'),
            Tile::Char('X'),
        ],
    );
    assert_eq!(
        Runner {
            context: context_with_inbox(vec![
                Tile::Char('U'),
                Tile::Char('N'),
                Tile::Char('I'),
                Tile::Char('X'),
                Tile::Num(0),
                Tile::Char('U'),
                Tile::Char('N'),
                Tile::Char('I'),
                Tile::Char('X'),
                Tile::Num(0),
            ]),
            pos: 0,
            statements: statements(),
        }.run(),
        vec![
            Tile::Char('U'),
            Tile::Char('N'),
            Tile::Char('I'),
            Tile::Char('X'),
        ],
    );
}
