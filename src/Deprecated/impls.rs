use crate::Staking;
use crate::context::Context;

impl <C: Context> Staking <C> {
    /// check stake amount
    /// 1. amount >= limit => become validator
    /// 2. amount < limit => delegate to validator
    /// 0 -> self delegation / 1 -> delegate to validator
    pub fn check_stake_type(&self, amount: u64) -> u8 {
        if amount >= self.context.threshold() {
            return 0_u8;
        }

        return 1_u8;
    }
}
