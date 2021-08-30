build:
	cargo build 
run:
	cargo run --features bevy/dynamic
#check:
#	cargo rustc -- -Awarnings

check:
		bash ./check.sh
		
