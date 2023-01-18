#!make
include .env

CONVERT_PK=$(shell cargo run --manifest-path aux/Cargo.toml --bin convert_public_key_ed $(PUBLIC_KEY))
IDENTIFIER_ADMIN='{"object":{"vec":[{"symbol":"Account"},{"object":{"account_id":{"public_key_type_ed25519":"$(PUBLIC_KEY_ED)"}}}]}}'

SOROBAN_DEPLOY=$(shell soroban deploy --wasm $(CONTRACT_WASM_TARGET) --secret-key $(SECRET_KEY) --rpc-url $(RPC_URL) --network-passphrase $(SECRET_PHRASE))

CHECK_CONVERT_PK = $(if $(value $(1)),,$(shell 	echo PUBLIC_KEY_ED=$(CONVERT_PK) >> .env))

convert_pk:
	$(call CHECK_CONVERT_PK,PUBLIC_KEY_ED)

init:
	soroban invoke \
    --wasm lottery/target/wasm32-unknown-unknown/release/lottery.wasm \
    --id 1 \
    --fn initialize \
    --arg $(IDENTIFIER_ADMIN) \
	--arg 8216da0fc28346cdd27a61819f93cc7c1488584717ba6e94a8e45a5daa084688 \
	--arg 10 \
	--arg 100

start:
	soroban invoke \
	--wasm lottery/target/wasm32-unknown-unknown/release/lottery.wasm \
	--id 1 \
	--account $(PUBLIC_KEY) \
	--fn start

end:
	soroban invoke \
	--wasm lottery/target/wasm32-unknown-unknown/release/lottery.wasm \
	--id 1 \
	--account $(PUBLIC_KEY) \
	--fn end

buy_ticket:
	soroban invoke \
	--wasm lottery/target/wasm32-unknown-unknown/release/lottery.wasm \
	--id 1 \
	--account $(USER_PUBLIC_KEY) \
	--fn buy_ticket
