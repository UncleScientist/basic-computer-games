use rand::{
    distr::{Distribution, StandardUniform},
    rngs::ThreadRng,
    *,
};

pub struct Slots {
    payout: f32,
    rng: ThreadRng,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SlotImage {
    Bar,
    Bell,
    Orange,
    Lemon,
    Plum,
    Cherry,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WinState {
    None,
    Double,
    DoubleBar,
    TopDollar,
    Jackpot,
}

pub struct PullResult {
    pub dial1: SlotImage,
    pub dial2: SlotImage,
    pub dial3: SlotImage,
    pub result: WinState,
}

impl Default for Slots {
    fn default() -> Self {
        Self::new()
    }
}

impl Slots {
    pub fn new() -> Self {
        Self {
            payout: 0.0,
            rng: rand::rng(),
        }
    }

    pub fn get_payout(&self) -> f32 {
        self.payout
    }

    pub fn pull_arm(&mut self, bet: f32) -> PullResult {
        let dial1 = self.rng.random::<SlotImage>();
        let dial2 = self.rng.random::<SlotImage>();
        let dial3 = self.rng.random::<SlotImage>();

        let result = Self::get_win_state(dial1, dial2, dial3);

        self.payout += Self::payout(result, bet);

        PullResult {
            dial1,
            dial2,
            dial3,
            result,
        }
    }

    fn get_win_state(dial1: SlotImage, dial2: SlotImage, dial3: SlotImage) -> WinState {
        match (dial1, dial2, dial3) {
            (SlotImage::Bar, SlotImage::Bar, SlotImage::Bar) => WinState::Jackpot,
            (SlotImage::Bar, SlotImage::Bar, _)
            | (_, SlotImage::Bar, SlotImage::Bar)
            | (SlotImage::Bar, _, SlotImage::Bar) => WinState::DoubleBar,
            (x, y, z) if x == y && y == z => WinState::TopDollar,
            (x, y, z) if x == y || y == z || x == z => WinState::Double,
            _ => WinState::None,
        }
    }

    fn payout(result: WinState, bet: f32) -> f32 {
        match result {
            WinState::None => 0.0,
            WinState::Double => bet * 2.0,
            WinState::DoubleBar => bet * 5.0,
            WinState::TopDollar => bet * 10.0,
            WinState::Jackpot => bet * 100.0,
        }
    }
}

impl Distribution<SlotImage> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SlotImage {
        match rng.random_range(0..=5) {
            0 => SlotImage::Bar,
            1 => SlotImage::Bell,
            2 => SlotImage::Orange,
            3 => SlotImage::Lemon,
            4 => SlotImage::Plum,
            5 => SlotImage::Cherry,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_LIST: [((SlotImage, SlotImage, SlotImage), WinState); 6] = [
        (
            (SlotImage::Lemon, SlotImage::Cherry, SlotImage::Plum),
            WinState::None,
        ),
        (
            (SlotImage::Lemon, SlotImage::Lemon, SlotImage::Plum),
            WinState::Double,
        ),
        (
            (SlotImage::Cherry, SlotImage::Lemon, SlotImage::Cherry),
            WinState::Double,
        ),
        (
            (SlotImage::Cherry, SlotImage::Bar, SlotImage::Bar),
            WinState::DoubleBar,
        ),
        (
            (SlotImage::Bell, SlotImage::Bell, SlotImage::Bell),
            WinState::TopDollar,
        ),
        (
            (SlotImage::Bar, SlotImage::Bar, SlotImage::Bar),
            WinState::Jackpot,
        ),
    ];

    #[test]
    fn test_create_slots() {
        let slots = Slots::new();
        assert_eq!(0.0, slots.payout);
    }

    #[test]
    fn test_nowin_states() {
        for test in TEST_LIST {
            assert_eq!(test.1, Slots::get_win_state(test.0.0, test.0.1, test.0.2));
        }
    }

    #[test]
    fn test_no_payout() {
        assert_eq!(0.0, Slots::payout(WinState::None, 5.0));
    }

    #[test]
    fn test_double_payout() {
        assert_eq!(2.0, Slots::payout(WinState::Double, 1.0));
    }

    #[test]
    fn test_double_bar() {
        assert_eq!(5.0, Slots::payout(WinState::DoubleBar, 1.0));
    }

    #[test]
    fn test_top_dollar() {
        assert_eq!(10.0, Slots::payout(WinState::TopDollar, 1.0));
    }

    #[test]
    fn test_jackpot() {
        assert_eq!(100.0, Slots::payout(WinState::Jackpot, 1.0));
    }
}
