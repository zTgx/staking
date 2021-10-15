### stake self as a validator using fn cli
```shell
# ex)
# - To stake 999,999 FRAs with a commision rate of 2% (and validator name of Validator_Pool_A)
# - Note: that is 999999 * 1000000 FRA units
# - Your Staker Memo file should like this:
cat staker_memo
{
  "name": "ExampleNode",
  "desc": "I am just a example description, please change me.",
  "website": "https://www.example.com",
  "logo": "https://www.example.com/logo"
}
fn stake -n $((999999 * 1000000)) -R 0.02 -M "$(cat staker_memo)"
```
