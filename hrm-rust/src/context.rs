#[derive(Clone, Debug)]
pub enum Tile {
    Num(i32),
    Char(char),
}

impl Tile {
    pub fn is_num(&self) -> bool {
        match self {
            Tile::Num(_) => true,
            _ => false,
        }
    }
    pub fn num(&self) -> i32 {
        match self {
            Tile::Num(num) => *num,
            _ => panic!("calling num on tile with a non numeric tile"),
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, tile: &Tile) -> bool {
        match *self {
            Tile::Num(num) => match *tile {
                Tile::Num(num2) => num == num2,
                _ => false,
            },
            Tile::Char(c) => match *tile {
                Tile::Char(c2) => c == c2,
                _ => false,
            },
        }
    }
}

impl std::ops::Add for Tile {
    type Output = Tile;
    fn add(self, tile: Tile) -> Tile {
        match self {
            Tile::Num(num) => match tile {
                Tile::Num(num2) => Tile::Num(num + num2),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Sub for Tile {
    type Output = Tile;
    fn sub(self, tile: Tile) -> Tile {
        match self {
            Tile::Num(num) => match tile {
                Tile::Num(num2) => Tile::Num(num - num2),
                _ => unimplemented!(),
            },
            Tile::Char(num) => match tile {
                Tile::Char(num2) => Tile::Num(num as i32 - num2 as i32),
                _ => unimplemented!(),
            },
        }
    }
}

pub struct Context {
    pub inbox_queue: Vec<Tile>,
    pub outbox_line: Vec<Tile>,
    pub memory: Vec<Option<Tile>>,
    pub hand: Option<Tile>,
}

impl Context {
    pub fn new(t: &[Tile], memory_size: usize) -> Context {
        let mut memory = Vec::with_capacity(memory_size);
        for _ in 0..memory_size {
            memory.push(None)
        }
        Context {
            inbox_queue: t.to_vec(),
            outbox_line: vec![],
            memory,
            hand: None,
        }
    }

    pub fn new_with_memory(t: &[Tile], memory: Vec<Option<Tile>>) -> Context {
        Context {
            inbox_queue: t.to_vec(),
            outbox_line: vec![],
            memory,
            hand: None,
        }
    }

    pub fn inbox(&mut self) -> &mut Self {
        if self.inbox_queue.len() == 0 {
            self.hand = None
        } else {
            self.hand = Some(self.inbox_queue.remove(0));
        }
        self
    }

    pub fn outbox(&mut self) -> &mut Self {
        self.outbox_line.push(self.hand.take().expect("Outbox with empty hand"));
        self
    }

    pub fn copyfrom(&mut self, pos: usize) -> &mut Self {
        self.hand = self.memory[pos].clone();
        self
    }

    pub fn copyto(&mut self, pos: usize) -> &mut Self {
        self.memory[pos] = self.hand.clone();
        self
    }

    pub fn add(&mut self, pos: usize) -> &mut Self {
        self.hand = Some(self.hand.clone().unwrap() + self.memory[pos].clone().unwrap());
        self
    }

    pub fn sub(&mut self, pos: usize) -> &mut Self {
        self.hand = Some(self.hand.clone().unwrap() - self.memory[pos].clone().unwrap());
        self
    }

    pub fn incr(&mut self, pos: usize) -> &mut Self {
        self.hand = Some(self.memory[pos].clone().unwrap() + Tile::Num(1));
        self.memory[pos] = self.hand.clone();
        self
    }

    pub fn decr(&mut self, pos: usize) -> &mut Self {
        self.hand = Some(self.memory[pos].clone().unwrap() + Tile::Num(-1));
        self.memory[pos] = self.hand.clone();
        self
    }

    pub fn copyfromp(&mut self, ppos: usize) -> &mut Self {
        let pos = self.memory[ppos].as_ref().expect("empty tile pointer").num() as usize;
        self.hand = self.memory[pos].clone();
        self
    }

    pub fn copytop(&mut self, ppos: usize) -> &mut Self {
        let pos = self.memory[ppos].as_ref().expect("empty tile pointer").num() as usize;
        self.memory[pos] = self.hand.clone();
        self
    }

    pub fn addp(&mut self, ppos: usize) -> &mut Self {
        let pos = self.memory[ppos].as_ref().expect("empty tile pointer").num() as usize;
        self.hand = Some(self.hand.clone().unwrap() + self.memory[pos].clone().unwrap());
        self
    }

    pub fn subp(&mut self, ppos: usize) -> &mut Self {
        let pos = self.memory[ppos].as_ref().expect("empty tile pointer").num() as usize;
        self.hand = Some(self.hand.clone().unwrap() - self.memory[pos].clone().unwrap());
        self
    }

    pub fn incrp(&mut self, ppos: usize) -> &mut Self {
        let pos = self.memory[ppos].as_ref().expect("empty tile pointer").num() as usize;
        self.hand = Some(self.memory[pos].clone().unwrap() + Tile::Num(1));
        self.memory[pos] = self.hand.clone();
        self
    }

    pub fn decrp(&mut self, ppos: usize) -> &mut Self {
        let pos = self.memory[ppos].as_ref().expect("empty tile pointer").num() as usize;
        self.hand = Some(self.memory[pos].clone().unwrap() + Tile::Num(-1));
        self.memory[pos] = self.hand.clone();
        self
    }
}

#[test]
fn test_inbox() {
    let mut context = Context::new(&[Tile::Num(3), Tile::Num(4)], 0);
    assert_eq!(context.inbox().hand, Some(Tile::Num(3)));
    assert_eq!(context.inbox().hand, Some(Tile::Num(4)));
}

#[test]
fn test_outbox() {
    let mut context = Context::new(&[], 0);
    context.hand = Some(Tile::Num(3));
    context.outbox();
    context.hand = Some(Tile::Num(4));
    context.outbox();
    assert_eq!(context.outbox_line,
        vec![
            Tile::Num(3),
            Tile::Num(4),
        ]
    );
}

#[test]
fn test_memory() {
    let mut context = Context::new(&[Tile::Num(3), Tile::Num(4)], 6);
    context.inbox();
    context.copyto(5);
    context.inbox();
    assert_eq!(context.hand, Some(Tile::Num(4)));
    context.copyfrom(5);
    assert_eq!(context.hand, Some(Tile::Num(3)));
}

#[test]
fn test_add() {
    let mut context = Context::new(&[Tile::Num(3), Tile::Num(4)], 6);
    context.inbox();
    context.copyto(5);
    context.inbox();
    context.add(5);
    assert_eq!(context.hand, Some(Tile::Num(7)));
    context.add(5);
    assert_eq!(context.hand, Some(Tile::Num(10)));
}

#[test]
fn test_sub() {
    let mut context = Context::new(&[Tile::Num(3), Tile::Num(4)], 6);
    context.inbox();
    context.copyto(5);
    context.inbox();
    context.sub(5);
    assert_eq!(context.hand, Some(Tile::Num(1)));
    context.sub(5);
    assert_eq!(context.hand, Some(Tile::Num(-2)));
}

#[test]
fn test_incr() {
    let mut context = Context::new(&[Tile::Num(3), Tile::Num(14)], 6);
    context.inbox();
    context.copyto(5);
    context.inbox();
    assert_eq!(context.hand, Some(Tile::Num(14)));
    context.incr(5);
    assert_eq!(context.hand, Some(Tile::Num(4)));
    assert_eq!(context.memory[5], Some(Tile::Num(4)));
}

#[test]
fn test_decr() {
    let mut context = Context::new(&[Tile::Num(3), Tile::Num(14)], 6);
    context.inbox();
    context.copyto(5);
    context.inbox();
    assert_eq!(context.hand, Some(Tile::Num(14)));
    context.decr(5);
    assert_eq!(context.hand, Some(Tile::Num(2)));
    assert_eq!(context.memory[5], Some(Tile::Num(2)));
}

#[test]
fn test_copyfromp() {
    let mut context = Context::new(&[], 6);
    context.memory[3] = Some(Tile::Num(0));
    context.memory[0] = Some(Tile::Num(-1));
    context.copyfromp(3);
    assert_eq!(context.hand, Some(Tile::Num(-1)));
}

#[test]
fn test_copytop() {
    let mut context = Context::new(&[], 6);
    context.memory[3] = Some(Tile::Num(0));
    context.hand = Some(Tile::Num(-1));
    context.copytop(3);
    assert_eq!(context.memory[0], Some(Tile::Num(-1)));
}

#[test]
fn test_addp() {
    let mut context = Context::new(&[], 6);
    context.memory[3] = Some(Tile::Num(0));
    context.memory[0] = Some(Tile::Num(-3));
    context.hand = Some(Tile::Num(-1));
    context.addp(3);
    assert_eq!(context.hand, Some(Tile::Num(-4)));
}

#[test]
fn test_subp() {
    let mut context = Context::new(&[], 6);
    context.memory[3] = Some(Tile::Num(0));
    context.memory[0] = Some(Tile::Num(-3));
    context.hand = Some(Tile::Num(-1));
    context.subp(3);
    assert_eq!(context.hand, Some(Tile::Num(2)));
}
