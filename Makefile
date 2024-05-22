.PHONY: build build build_by_manual

build:
	dfx stop && dfx start --clean --background
	dfx canister create --all
	dfx build --all
	cp .dfx/local/canisters/backend_1/backend_1.wasm artifacts/Backend1.wasm
	cp .dfx/local/canisters/backend_2/backend_2.wasm artifacts/Backend2.wasm
	cp .dfx/local/canisters/backend_1_gzip/backend_1_gzip.wasm.gz artifacts/Backend1_gzip.wasm.gz
	cp .dfx/local/canisters/backend_2_gzip/backend_2_gzip.wasm.gz artifacts/Backend2_gzip.wasm.gz
	dfx stop

build_by_manual:
	cargo build --target wasm32-unknown-unknown --release --locked -p backend_1
	cargo build --target wasm32-unknown-unknown --release --locked -p backend_2
	ic-wasm target/wasm32-unknown-unknown/release/backend_1.wasm -o artifacts/Backend1_only_shrink.wasm shrink
	ic-wasm target/wasm32-unknown-unknown/release/backend_2.wasm -o artifacts/Backend2_only_shrink.wasm shrink

launch:
	dfx stop && dfx start --background --clean
	dfx deploy backend_2
	dfx deploy backend_1
	# dfx deploy backend_1 --argument "(principal \"$(dfx canister id backend_2)\", principal \"$(dfx canister id backend_2)\", principal \"$(dfx canister id backend_2)\")"
	dfx deploy manager --argument "(record { backend_1 = \"$(dfx canister id backend_1)\"; backend_2 = \"$(dfx canister id backend_2)\" })"

setup:
	dfx canister call manager set_principal_texts "(\"$(dfx canister id backend_1)\", \"$(dfx canister id backend_2)\")"
	dfx canister update-settings backend_1 --add-controller $(dfx canister id manager)
	dfx canister update-settings backend_2 --add-controller $(dfx canister id manager)
	# dfx canister call backend_1 add_controller "(principal \"$(dfx canister id manager)\")"
	# dfx canister call backend_2 add_controller "(principal \"$(dfx canister id manager)\")"
