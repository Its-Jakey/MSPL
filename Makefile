RUSTC := rustc

mspl: src/main.rs
	$(RUSTC) -o mspl src/main.rs