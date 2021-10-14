```shell
rm -rf release
mkdir release
cd release; for i in bin lib; do mkdir $i; done
cp ./target/release/findorad ./target/release/abcid /home/u20/go/bin/tendermint ./target/release/fn ./target/release/stt ./target/release/staking_cfg_generator release/bin
cp release/bin/* ~/.cargo/bin/
cd release/bin/ && findorad pack
cp -f /tmp/findorad release/bin/
cp -f /tmp/findorad ~/.cargo/bin/
```

