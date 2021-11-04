use rand::{
    distributions::{Distribution, Standard},
    prelude::SliceRandom,
    thread_rng, Rng,
};
use std::{
    cmp::Ordering,
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Door {
    Left,
    Middle,
    Right,
}

impl Display for Door {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Distribution<Door> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Door {
        use Door::*;
        *[Left, Middle, Right].choose(rng).unwrap()
    }
}

impl Door {
    fn get_other<R: Rng + ?Sized>(rng: &mut R, first: Self, second: Self) -> Self {
        use Door::*;
        match (first, second) {
            (Middle, Right) | (Right, Middle) => Left,
            (Left, Right) | (Right, Left) => Middle,
            (Left, Middle) | (Middle, Left) => Right,
            (Left, Left) => *[Middle, Right].choose(rng).unwrap(),
            (Middle, Middle) => *[Left, Right].choose(rng).unwrap(),
            (Right, Right) => *[Left, Middle].choose(rng).unwrap(),
        }
    }
}

fn main() {
    const COUNT: u8 = 100;
    let mut rng = thread_rng();
    let mut switching_won_count = 0;
    for _ in 0..COUNT {
        let prize = rng.gen();
        let first_pick = rng.gen();
        let host_pick = Door::get_other(&mut rng, prize, first_pick);
        let switch_pick = Door::get_other(&mut rng, first_pick, host_pick);

        let which_won;
        if switch_pick == prize {
            switching_won_count += 1;
            which_won = "Switching";
        } else {
            which_won = "Not Switching";
        }

        println!(
            "prize: {},\tpicked: {},\thost picked: {},\tswitched to: {},\twhich won: {}",
            prize, first_pick, host_pick, switch_pick, which_won
        );
    }

    let not_switching_won_count = COUNT - switching_won_count;
    let overall_winner = match switching_won_count.cmp(&not_switching_won_count) {
        Ordering::Less => "Not Switching wins",
        Ordering::Equal => "It's a tie",
        Ordering::Greater => "Switching wins",
    };

    println!(
        "Switching to Not Switching = {} to {}\n{}",
        switching_won_count, not_switching_won_count, overall_winner
    );
}
