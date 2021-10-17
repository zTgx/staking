//! Staking Client
//! Send Transaction to Tendermint
//! 
//! Includings:
//! stake / unstake / append / update / claim / info
//! 

#[macro_use]
pub mod macros;
use ruc::*;
pub mod pre_check;
pub mod macros;
pub mod constants;
pub mod net;


type Amount = u64;

pub struct Cli {}
impl Cli {
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
    
        Net::build(Ops::Staking).send()
    }

    /// 解质押
    pub fn unstake(
        am: Option<&str>,
        staker: Option<&str>,
        td_addr: Option<TendermintAddrRef>,
    ) -> Result<()> {
        let am = amount.parse::<u64>().c(d!("'amount' must be an integer"))?;
    
        let kp = staker
            .c(d!())
            .and_then(|sk| wallet::restore_keypair_from_mnemonic_default(sk).c(d!()))
            .or_else(|_| get_keypair().c(d!()))?;
        let td_addr_bytes = td_addr
            .c(d!())
            .and_then(|ta| td_addr_to_bytes(ta).c(d!()))
            .or_else(|_| {
                get_td_pubkey()
                    .c(d!())
                    .map(|td_pk| td_pubkey_to_td_addr_bytes(&td_pk))
            })?;
    
        Net::build(Ops::Unstaking).send()
    }

    /// 追加质押金额
    pub fn append(
        amount: &str,
        staker: Option<&str>,
        td_addr: Option<TendermintAddrRef>,
    ) -> Result<()> {
        let am = amount.parse::<u64>().c(d!("'amount' must be an integer"))?;
        pre_check::check_delegation_amount(am, true).c(d!())?;
    
        let td_addr = td_addr.map(|ta| ta.to_owned()).c(d!()).or_else(|_| {
            get_td_pubkey()
                .c(d!())
                .map(|td_pk| td_pubkey_to_td_addr(&td_pk))
        })?;
    
        let kp = staker
            .c(d!())
            .and_then(|sk| wallet::restore_keypair_from_mnemonic_default(sk).c(d!()))
            .or_else(|_| get_keypair().c(d!()))?;
    
        Net::build(Ops::Append).send()
    }
    /// 更新 staker memo等信息
    pub fn update(cr: Option<&str>, memo: Option<&str>) -> Result<()> {
        let addr = get_td_pubkey().map(|i| td_pubkey_to_td_addr(&i)).c(d!())?;
        let vd = get_validator_detail(&addr).c(d!())?;
    
        let cr = cr
            .map_or(Ok(vd.commission_rate), |s| {
                s.parse::<f64>()
                    .c(d!("commission rate must be a float number"))
                    .and_then(convert_commission_rate)
            })
            .c(d!())?;
        let memo = memo
            .map_or(Ok(vd.memo), |s| serde_json::from_str(s))
            .c(d!())?;
    
        let td_pubkey = get_td_pubkey().c(d!())?;
    
        let kp = get_keypair().c(d!())?;
        let vkp = get_td_privkey().c(d!())?;
    
        Net::build(Ops::Update).send()
    }
    /// 要奖励
    pub fn claim(am: Option<&str>, sk_str: Option<&str>) -> Result<()> {
        let am = amount.parse::<u64>().c(d!("'amount' must be an integer"))?;    
        let kp = restore_keypair_from_str_with_default(sk_str)?;
    
        Net::build(Ops::Claim).send()
    }

    /// staking 最新的信息
    pub fn info() {
        let kp = get_keypair().c(d!())?;

        let serv_addr = ruc::info!(get_serv_addr()).map(|i| {
            println!("\x1b[31;01mServer URL:\x1b[00m\n{}\n", i);
        });

        let xfr_account = ruc::info!(get_keypair()).map(|i| {
            println!(
                "\x1b[31;01mFindora Address:\x1b[00m\n{}\n",
                wallet::public_key_to_bech32(&i.get_pk())
            );
            println!(
                "\x1b[31;01mFindora Public Key:\x1b[00m\n{}\n",
                wallet::public_key_to_base64(&i.get_pk())
            );
        });

        let self_balance = ruc::info!(utils::get_balance(&kp)).map(|i| {
            println!("\x1b[31;01mNode Balance:\x1b[00m\n{} FRA units\n", i);
        });

        if basic {
            return Ok(());
        }

        let td_info = ruc::info!(get_td_pubkey()).map(|i| {
            let addr = td_pubkey_to_td_addr(&i);
            println!("\x1b[31;01mValidator Node Addr:\x1b[00m\n{}\n", addr);
            (i, addr)
        });

        let di = utils::get_delegation_info(kp.get_pk_ref());
        let bond_entries = match di.as_ref() {
            Ok(di) => Some(di.bond_entries.clone()),
            Err(_) => None,
        };

        let delegation_info = di.and_then(|di| {
            serde_json::to_string_pretty(&di).c(d!("server returned invalid data"))
        });
        let delegation_info = ruc::info!(delegation_info).map(|i| {
            println!("\x1b[31;01mYour Delegation:\x1b[00m\n{}\n", i);
        });

        if let Ok((tpk, addr)) = td_info.as_ref() {
            let self_delegation =
                bond_entries.map_or(false, |bes| bes.iter().any(|i| &i.0 == addr));
            if self_delegation {
                let res = utils::get_validator_detail(&td_pubkey_to_td_addr(tpk))
                    .c(d!("Validator not found"))
                    .and_then(|di| {
                        serde_json::to_string_pretty(&di)
                            .c(d!("server returned invalid data"))
                    })
                    .map(|i| {
                        println!("\x1b[31;01mYour Staking:\x1b[00m\n{}\n", i);
                    });
                ruc::info_omit!(res);
            }
        }

        if [
            serv_addr,
            xfr_account,
            td_info.map(|_| ()),
            self_balance,
            delegation_info,
        ]
        .iter()
        .any(|i| i.is_err())
        {
            Err(eg!("unable to obtain complete information"))
        } else {
            Ok(())
        }
    }
}