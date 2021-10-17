

enum Ops {
    Staking,
    Unstaking,
    Claim,
    Append,
    Update,
}
pub struct Net {
}

impl Net {
    pub fn build(op: Ops) -> Net {
        // let mut builder = utils::new_tx_builder().c(d!())?;
        // builder
        //     .add_operation_staking(&kp, am, &vkp, td_pubkey, cr, memo.map(|m| m.to_owned()))
        //     .c(d!())?;
        // utils::gen_transfer_op(
        //     &kp,
        //     vec![(&BLACK_HOLE_PUBKEY_STAKING, am)],
        //     None,
        //     false,
        //     false,
        // )
        // .c(d!())
        // .map(|principal_op| builder.add_operation(principal_op))?;
    
        // utils::send_tx(&builder.take_transaction()).c(d!())

        Self {

        }
    }

    pub fn send(&self) -> Result<()> {

    }
}

