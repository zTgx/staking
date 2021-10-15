### 自我质押

```shell
cat staker_memo
{
  "name": "ExampleNode",
  "desc": "I am just a example description, please change me.",
  "website": "https://www.example.com",
  "logo": "https://www.example.com/logo"
}
fn stake -n $((999999 * 1000000)) -R 0.02 -M "$(cat staker_memo)"
```



### stake 方法实现

```rust
pub fn stake(
    amount: &str, // 质押的FRA数量
    commission_rate: &str, // 设置的validator手续费率
    memo: Option<&str>, // 设置的validator memo
    force: bool,
) -> Result<()> {
    // 解析质押 amount 数量
    let am = amount.parse::<u64>().c(d!("'amount' must be an integer"))?;
    
    // 检查是否符合质押数量
    check_delegation_amount(am, false).c(d!())?;
    
    // TODO:
    // Amount 类型就是 u64，但是使用的时候不统一，Amount 和 u64 混用！
    // amount.parse::<Amount>() 更合适。
    //
    
    // 转换费率的表达方式, 分子分母都乘 10000。
    // 0.02 =》 [u64; 2] =》[0.02 * 10000.0, 10000]
    let cr = commission_rate
        .parse::<f64>()
        .c(d!("commission rate must be a float number"))
        .and_then(|cr| convert_commission_rate(cr).c(d!()))?;
    
    // tendermint 的 publickey
    let td_pubkey = get_td_pubkey().c(d!())?;

    // 通过 MNEMONIC 注记词获取该节点的 keypair
    let kp = get_keypair().c(d!())?;
    
    // tendermint 的 privatekey
    let vkp = get_td_privkey().c(d!())?;
    
    // TODO:
    // get_td_pubkey 和 get_td_privkey 为什么中间还有加一个 get_keypair() ???
    // 

    // TODO:
    // 这样的宏工具，应该统一放到一起吧？？？
    //
    macro_rules! diff {
        ($l:expr, $r:expr) => {
            if $l > $r {
                $l - $r
            } else {
                $r - $l
            }
        };
    }

    // 获取tendermint中目前 lastest block height
    let network_height = get_block_height(get_serv_addr().unwrap());
    
    // 获取本地的block height
    // get_local_block_height 就是修改了 get_block_height 的serv addr为127.0.0.1 
    // TODO:
    // 可以换一种方式，不用再次发请求，放到本地的缓存中，直接从内存中get？？？
    // 下面的block height 比较，可以单独封一个方法了，甚至实现为可动态配置的高度区别；
    // 3 还是 hardcode， 改成const变量不更合适？？？
    
    let local_height = get_local_block_height();
    if (network_height == 0 || local_height == 0)
        || diff!(network_height, local_height) > 3
    {
        println!(
            "The difference in block height of your node and the remote network is too big: \n remote / local: {} / {}",
            network_height, local_height
        );
        if !force {
            println!("Append option --force to ignore this warning.");
            return Ok(());
        }
        println!("Continue to stake now...");
    }

    // 经过上面的 pre_check ，终于可以提交交易了。
    // TODO: 可以把上面的 pre check 代码进一步规范？？？ pre_check / stake / end_check ???
    // 下面的代码就是一套流程了，new builder -> add operation -> send_tx
    // 提交给 tendermint 进行共识
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
```



### staking 交易处理

通过上面代码把交易提交到 tendermint , 剩下的流程入口在 abci  的 check_tx / deliver_tx / end_block / commit 接口中了。
