use crate::input;

use std::collections::VecDeque;

const WHITE_TRACE_PRICE: u32 = 20000;
const INNOCENCE_TRACE_PRICE: u32 = 12000;

#[derive(Clone)]
struct UpgradeContext {
    upgradeable_count: u32,
    success_rate: f64,
    recovery_rate: f64,
    upgrade_price: u32,
    white_price: u32,
    innocence_price: u32,
}

impl UpgradeContext {
    fn new(state: &input::State) -> Self {
        let upgradeable_count = state.get_or_default(input::FieldId::UpgradeableCount);

        let base_success_rate = state.get_or_default(input::FieldId::BaseSuccessRate);
        let handicraft_level = state.get_or_default(input::FieldId::Handicraft);
        let enhance_mastery_level = state.get_or_default(input::FieldId::EnhancementMastery);
        let success_rate = Self::calculate_success_rate(
            base_success_rate,
            handicraft_level,
            enhance_mastery_level,
        );

        let upgrade_salvation_level = state.get_or_default(input::FieldId::UpgradeSalvation);
        let recovery_rate = Self::calculate_recovery_rate(success_rate, upgrade_salvation_level);

        let upgrade_price = state.get_or_default(input::FieldId::UpgradePrice);

        let trace_required = state.get_or_default(input::FieldId::TraceRequired);
        let white_price = Self::calculate_white_price(trace_required);

        let trace_price = state.get_or_default(input::FieldId::TracePrice);
        let innocence_price = Self::calculate_innocence_price(trace_price);

        Self {
            upgradeable_count,
            success_rate,
            recovery_rate,
            upgrade_price,
            white_price,
            innocence_price,
        }
    }

    fn calculate_success_rate(
        base_success_rate: u32,
        handicraft_level: u32,
        enhance_mastery_level: u32,
    ) -> f64 {
        f64::from(base_success_rate)
            + handicraft_effect(handicraft_level)
            + f64::from(enhance_mastery_effect(enhance_mastery_level))
    }

    const fn get_upgradeable_count(&self) -> u32 {
        self.upgradeable_count
    }

    fn calculate_recovery_rate(success_rate: f64, upgrade_salvation_level: u32) -> f64 {
        let fail_rate = 100.0 - success_rate;
        let base_recovery_rate = f64::from(upgrade_salvation_effect(upgrade_salvation_level));

        fail_rate * base_recovery_rate / 100.0
    }

    const fn calculate_white_price(trace_price: u32) -> u32 {
        trace_price * WHITE_TRACE_PRICE
    }

    const fn calculate_innocence_price(trace_price: u32) -> u32 {
        trace_price * INNOCENCE_TRACE_PRICE
    }
}

#[derive(Clone)]
pub enum Action {
    Upgrade,
    White,
    Innocence,
}

struct Position {
    failure: usize,
    success: usize,
    upgradeable_count: usize,
}

impl Position {
    const fn new(failure: usize, success: usize, upgradeable_count: usize) -> Self {
        Self {
            failure,
            success,
            upgradeable_count,
        }
    }

    const fn initial(upgradeable_count: usize) -> Self {
        let failure = upgradeable_count;
        let success = 0;
        let upgradeable_count = upgradeable_count;

        Self::new(failure, success, upgradeable_count)
    }

    const fn get_failure(&self) -> usize {
        self.failure
    }

    const fn get_success(&self) -> usize {
        self.success
    }

    const fn get_upgradeable_count(&self) -> usize {
        self.upgradeable_count
    }

    const fn up(&self) -> Option<Self> {
        if self.get_failure() == 0 {
            return None;
        }

        Some(Position {
            failure: self.get_failure() - 1,
            success: self.get_success(),
            upgradeable_count: self.get_upgradeable_count(),
        })
    }

    const fn right(&self) -> Option<Self> {
        let next_success = self.get_success() + 1;

        if next_success >= self.get_upgradeable_count() {
            return None;
        }

        Some(Position {
            failure: self.get_failure(),
            success: next_success,
            upgradeable_count: self.get_upgradeable_count(),
        })
    }
}

type Actions = Vec<Vec<Action>>;

#[derive(Clone)]
pub struct Strategy {
    upgrade_context: UpgradeContext,
    actions: Actions,
}

impl Strategy {
    fn new(upgrade_context: UpgradeContext) -> Self {
        let count = upgrade_context.get_upgradeable_count() as usize;
        let actions = Self::initial_actions(count);

        Self {
            upgrade_context,
            actions,
        }
    }

    /*
     * if count is 4, then initial actions will be look like:
     * UUUUC
     * UUUW
     * UUW
     * UW
     * W
     */
    fn initial_actions(count: usize) -> Actions {
        (0..=count)
            .map(|failure| {
                let upgrades = count - failure;
                let mut row = vec![Action::Upgrade; upgrades];
                if failure > 0 {
                    row.push(Action::White);
                }
                row
            })
            .collect()
    }

    const fn get_upgradeable_count(&self) -> u32 {
        self.upgrade_context.get_upgradeable_count()
    }

    fn placed_innocence(&self, position: &Position) -> Self {
        let mut strategy = self.clone();
        strategy.place_innocence(position);
        strategy
    }

    fn place_innocence(&mut self, position: &Position) {
        self.actions[position.get_failure()][position.get_success()] = Action::Innocence;
    }

    fn is_better_than(&self, other: &Self) -> bool {
        self.simulate_cost() < other.simulate_cost()
    }

    fn simulate_cost(&self) -> f64 {
        todo!()
    }

    fn optimized(&self) -> Self {
        let mut best_strategy = self.clone();
        let mut positions_to_explore = self.initialize_exploration_queue();

        while let Some(position) = positions_to_explore.pop_front() {
            if let Some(improved_strategy) = self.improved(&position) {
                best_strategy = improved_strategy;
                Self::add_neighboring_positions_to_queue(&position, &mut positions_to_explore);
            }
        }

        best_strategy
    }

    fn initialize_exploration_queue(&self) -> VecDeque<Position> {
        let initial_position = Position::initial(self.get_upgradeable_count() as usize);
        VecDeque::from([initial_position])
    }

    fn improved(&self, position: &Position) -> Option<Self> {
        let candidate_strategy = self.placed_innocence(position);

        if candidate_strategy.is_better_than(self) {
            Some(candidate_strategy)
        } else {
            None
        }
    }

    fn add_neighboring_positions_to_queue(position: &Position, queue: &mut VecDeque<Position>) {
        if let Some(up) = position.up() {
            queue.push_back(up);
        }
        if let Some(right) = position.right() {
            queue.push_back(right);
        }
    }
}

pub fn handicraft_effect(handicraft_level: u32) -> f64 {
    f64::from(handicraft_level / 5 * 5) / 10.0
}

pub const fn enhance_mastery_effect(enhance_mastery_level: u32) -> u32 {
    enhance_mastery_level
}

pub const fn upgrade_salvation_effect(upgrade_salvation_level: u32) -> u32 {
    upgrade_salvation_level
}

pub fn optimized_strategy(state: &input::State) -> Strategy {
    let context = UpgradeContext::new(state);
    let strategy = Strategy::new(context);
    strategy.optimized()
}
