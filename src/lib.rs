//! tmp file
//! 
//! 

#[macro_use]
pub mod macros;
use ruc::*;
pub mod pre_check;
pub mod macros;
pub mod constants;

pub struct StakingCommand {}
impl StakingCommand {
    /// 质押
    pub fn stake(
        amount: &str,
        commission_rate: &str,
        memo: Option<&str>,
        force: bool,
    ) -> Result<()> {
        let am = amount.parse::<u64>().c(d!("'amount' must be an integer"))?;
        pre_check::check_delegation_amount(am, false).c(d!())?;
        pre_check::check_block_height_interval();

        let kp = get_keypair().c(d!())?;
        let vkp = get_td_privkey().c(d!())?; 
        let td_pubkey = get_td_pubkey().c(d!())?;
    
        let mut builder = utils::new_tx_builder().c(d!())?;
        builder
            .add_operation_staking(&kp, am, &vkp, td_pubkey, cr, memo.map(|m| m.to_owned()))
            .c(d!())?;
        utils::gen_transfer_op(
            &kp,
            vec![(&BLACK_HOLE_PUBKEY_STAKING, am)],
            None,
            false,
            false,
        )
        .c(d!())
        .map(|principal_op| builder.add_operation(principal_op))?;
    
        utils::send_tx(&builder.take_transaction()).c(d!())
    }

    /// 解质押
    pub fn unstake() {}

    /// 追加质押金额
    pub fn append() {}

    /// 更新 staker memo等信息
    pub fn update() {}

    /// 要奖励
    pub fn claim() {}

    /// staking 最新的信息
    pub fn info() {}
}