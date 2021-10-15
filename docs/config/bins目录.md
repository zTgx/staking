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

执行 `make` 的输出，`Makefile` 部分的代码如下：

```rust
define pack
	- rm -rf $(target_dir)
	mkdir $(target_dir)
	cd $(target_dir); for i in $(subdirs); do mkdir $$i; done
	cp $(bin_files) $(target_dir)/$(bin_dir)
	cp $(target_dir)/$(bin_dir)/* ~/.cargo/bin/
	cd $(target_dir)/$(bin_dir)/ && findorad pack
	cp -f /tmp/findorad $(target_dir)/$(bin_dir)/
	cp -f /tmp/findorad ~/.cargo/bin/
endef
```

* 这个地方在 `main` 分支上有区别，代码如下：

```rust
define pack
	- rm -rf $(1)
	mkdir $(1)
	cd $(1); for i in $(subdirs); do mkdir $$i; done
	cp \
		./${CARGO_TARGET_DIR}/$(2)/$(1)/findorad \
		./${CARGO_TARGET_DIR}/$(2)/$(1)/abcid \
		./${CARGO_TARGET_DIR}/$(2)/$(1)/fn \
		./${CARGO_TARGET_DIR}/$(2)/$(1)/stt \
		./${CARGO_TARGET_DIR}/$(2)/$(1)/staking_cfg_generator \
		$(shell go env GOPATH)/bin/tendermint \
		$(1)/$(bin_dir)/
	cp $(1)/$(bin_dir)/* ~/.cargo/bin/
	cd $(1)/$(bin_dir)/ && findorad pack
	cp -f /tmp/findorad $(1)/$(bin_dir)/
	cp -f /tmp/findorad ~/.cargo/bin/
endef
```

使用 `$(2) $(1)` 来表示了路径， 调用的地方：

```shell
build_release_goleveldb: tendermint_goleveldb
	cargo build --release --bins -p abciapp -p finutils
	$(call pack,release)
```

涉及到 `$(2)` 的地方：

```shell
build_release_musl_goleveldb: tendermint_goleveldb
	cargo build --release --bins -p abciapp -p finutils --target=x86_64-unknown-linux-musl
	$(call pack,release,x86_64-unknown-linux-musl)
```

这里会有一个显示上的问题，`main` 分支直接 `make` 后，显示是这样，多了一个 `/` 符号。

```shell
rm -rf release
mkdir release
cd release; for i in bin lib; do mkdir $i; done
cp ./target//release/findorad ./target//release/abcid /home/u20/go/bin/tendermint ./target//release/fn ./target//release/stt ./target/release//staking_cfg_generator release/bin
cp release/bin/* ~/.cargo/bin/
cd release/bin/ && findorad pack
cp -f /tmp/findorad release/bin/
cp -f /tmp/findorad ~/.cargo/bin/
```


