mod types;

pub use crate::types::*;

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

use derivative::Derivative;

pub trait Character {}

pub trait Hero: Character {}

pub trait Minion: Character {}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Player {
    pub id: usize,
    #[derivative(Debug = "ignore")]
    pub match_: Weak<Match>,
    #[derivative(Debug(format_with = "Player::fmt_opponent"))]
    pub opponent_: RefCell<Weak<Player>>,
    pub max_mana: i32,
    pub mana: i32,
    #[derivative(Debug = "ignore")]
    pub characters: Vec<RefCell<Rc<dyn Character>>>,
    #[derivative(Debug = "ignore")]
    pub hero: Option<RefCell<Rc<dyn Hero>>>,
    #[derivative(Debug = "ignore")]
    pub minions: Vec<RefCell<Rc<dyn Minion>>>,
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
}

pub struct Game {
    pub card_library: Vec<Card>,
    pub matches: RefCell<Vec<Rc<Match>>>,
}

pub struct Match {
    pub game: RefCell<Weak<Game>>,
    pub players: RefCell<Vec<Rc<Player>>>,
    pub characters: Vec<RefCell<Rc<dyn Character>>>,
    pub turn_num: Cell<usize>,
}

impl Game {
    pub fn new(card_library: Vec<Card>) -> Rc<Game> {
        Rc::new(Game {
            card_library,
            matches: RefCell::new(vec![]),
        })
    }

    pub fn create_match(self: &Rc<Self>) -> Rc<Match> {
        let match_ = Rc::new(Match {
            game: RefCell::new(Rc::downgrade(self)),
            players: RefCell::new(vec![]),
            characters: vec![],
            turn_num: Cell::new(0),
        });
        self.matches.borrow_mut().push(match_.clone());
        let p0 = match_.add_player();
        let p1 = match_.add_player();
        *p0.opponent_.borrow_mut() = Rc::downgrade(&p1);
        *p1.opponent_.borrow_mut() = Rc::downgrade(&p0);
        match_
    }
}

impl Match {
    pub fn add_player(self: &Rc<Self>) -> Rc<Player> {
        let player = Rc::new(Player {
            id: self.players.borrow().len(),
            match_: Rc::downgrade(self),
            opponent_: RefCell::new(Weak::new()),
            max_mana: 0,
            mana: 0,
            characters: vec![],
            hero: None,
            minions: vec![],
            deck: vec![],
            hand: vec![],
        });
        self.players.borrow_mut().push(player.clone());
        player
    }

    pub fn current_player(self: &Rc<Self>) -> Rc<Player> {
        self.players.borrow()[self.turn_num.get() % 2].clone()
    }

    pub fn next_turn(self: &Rc<Self>) {
        self.turn_num.set(self.turn_num.get() + 1);
    }
}

impl Player {
    pub fn opponent(self: &Rc<Self>) -> Rc<Player> {
        self.opponent_.borrow().upgrade().unwrap()
    }

    pub fn end_turn(self: &Rc<Self>) {
        let match_ = self.match_.upgrade().unwrap();
        match_.next_turn();
    }

    fn fmt_opponent(
        obj: &RefCell<Weak<Player>>,
        fmt: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        let obj = obj.borrow().upgrade().unwrap();
        write!(fmt, "Player {{ id: {} }}", obj.id)?;
        Ok(())
    }
}
