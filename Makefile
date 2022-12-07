#!make
include .env

CONVERT_PK=$(shell cargo run --manifest-path aux/Cargo.toml --bin convert_public_key_ed $(PUBLIC_KEY))
IDENTIFIER_ADMIN='{"object":{"vec":[{"symbol":"Account"},{"object":{"accountId":{"publicKeyTypeEd25519":"$(PUBLIC_KEY_ED)"}}}]}}'
SOROBAN_DEPLOY=$(shell soroban deploy --wasm $(CONTRACT_WASM_TARGET) --secret-key $(SECRET_KEY) --rpc-url $(RPC_URL) --network-passphrase $(SECRET_PHRASE))
HEX_CONVERT=cargo run --manifest-path aux/Cargo.toml --bin hex_convert

friend_bot:
	curl "https://friendbot-futurenet.stellar.org/?addr=$(PUBLIC_KEY)"

friend_local:
	curl "http://localhost:8000/friendbot?addr=$(PUBLIC_KEY)"

build_contract:
	cargo build --manifest-path car_rental/Cargo.toml --target wasm32-unknown-unknown --release

convert_pk:
	echo PUBLIC_KEY_ED=$(CONVERT_PK) >> .env

deploy: convert_pk
	echo CONTRACT_ID=$(SOROBAN_DEPLOY)  >> .env

init:
	soroban invoke \
    --id $(CONTRACT_ID) \
    --secret-key $(SECRET_KEY) \
    --rpc-url $(RPC_URL) \
    --network-passphrase  $(SECRET_PHRASE) \
    --fn init \
    --arg $(IDENTIFIER_ADMIN)

add_car:
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase  $(SECRET_PHRASE) \
		--fn add_car --arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
		--arg 0 \
		--arg "$(shell $(HEX_CONVERT) $(PLATE))" \
		--arg "$(shell $(HEX_CONVERT) $(MODEL))" --arg "$(shell $(HEX_CONVERT) $(COLOR))" --arg $(HORSE)
# comma := ,
# empty:=
# space := $(empty) $(empty)
# foo := a b c
# bar := $(subst $(space),$(comma),$(foo))

# all: 
# 	@echo $(bar)