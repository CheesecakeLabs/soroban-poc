#!make
include .env

CONVERT_PK=$(shell cargo run --manifest-path aux/Cargo.toml --bin convert_public_key_ed $(PUBLIC_KEY))
IDENTIFIER_ADMIN='{"object":{"vec":[{"symbol":"Account"},{"object":{"accountId":{"publicKeyTypeEd25519":"$(PUBLIC_KEY_ED)"}}}]}}'

CONVERT_PK_CLIENT=$(shell cargo run --manifest-path aux/Cargo.toml --bin convert_public_key_ed $(CLIENT_PUBLIC_KEY))
IDENTIFIER_CLIENT='{"object":{"vec":[{"symbol":"Account"},{"object":{"accountId":{"publicKeyTypeEd25519":$(PUBLIC_KEY_ED_CLIENT)}}}]}}'

SOROBAN_DEPLOY=$(shell soroban deploy --wasm $(CONTRACT_WASM_TARGET) --secret-key $(SECRET_KEY) --rpc-url $(RPC_URL) --network-passphrase $(SECRET_PHRASE))
SOROBAN_READ_CAR='$(shell soroban invoke --id $(CONTRACT_ID) --secret-key $(SECRET_KEY) --rpc-url $(RPC_URL) --network-passphrase $(SECRET_PHRASE) --fn read_car --arg "$(shell $(HEX_CONVERT) $(PLATE))")'

HEX_CONVERT=cargo run --manifest-path aux/Cargo.toml --bin hex_convert
CAR_BYTES_CONVERT=cargo run --manifest-path aux/Cargo.toml --bin car_bytes_convert

CHECK_CONTRACT_ID = $(if $(value $(1)),,$(shell echo CONTRACT_ID=$(SOROBAN_DEPLOY)  >> .env))
CHECK_CONVERT_PK = $(if $(value $(1)),,$(shell 	echo PUBLIC_KEY_ED=$(CONVERT_PK) >> .env))

friendbot:
	curl "https://friendbot-futurenet.stellar.org/?addr=$(PUBLIC_KEY)"

friendbot_local:
	curl "http://localhost:8000/friendbot?addr=$(PUBLIC_KEY)"

build_contract:
	cargo build --manifest-path car_rental/Cargo.toml --target wasm32-unknown-unknown --release

convert_pk:
	$(call CHECK_CONVERT_PK,PUBLIC_KEY_ED)

convert_pk_client:
	$(eval PUBLIC_KEY_ED_CLIENT=$(CONVERT_PK_CLIENT))

deploy: convert_pk
	$(call CHECK_CONTRACT_ID,CONTRACT_ID)
	echo "deploying $(CONTRACT_ID)"

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

remove_car:
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase  $(SECRET_PHRASE) \
		--fn remove_car --arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
		--arg 0 \
		--arg "$(shell $(HEX_CONVERT) $(PLATE))" 
		

read_car:
	$(eval CAR_DATA := $(SOROBAN_READ_CAR))
	@echo $(shell $(CAR_BYTES_CONVERT) $(CAR_DATA)) 

open_req:
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(CLIENT_SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase  $(SECRET_PHRASE) \
		--fn open_req \
		--arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' --arg 0

approve_req: convert_pk_client
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase $(SECRET_PHRASE) \
		--fn appr_req \
		--arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
		--arg $(IDENTIFIER_CLIENT) \
		--arg 0

deny_req: convert_pk_client
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase $(SECRET_PHRASE) \
		--fn deny_req \
		--arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
		--arg $(IDENTIFIER_CLIENT) \
		--arg 0

read_client: convert_pk_client
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase $(SECRET_PHRASE) \
		--fn read_clnt \
		--arg $(IDENTIFIER_CLIENT) 


reserve_car:
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(CLIENT_SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase  $(SECRET_PHRASE) \
		--fn resrve_car --arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
		--arg 0 \
		--arg "$(shell $(HEX_CONVERT) $(PLATE))" 

read_rent:
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase  $(SECRET_PHRASE) \
		--fn read_rent \
		--arg "$(shell $(HEX_CONVERT) $(PLATE))" 

take_car:
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(CLIENT_SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase  $(SECRET_PHRASE) \
		--fn take_car \
		--arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
		--arg 0 \
		--arg "$(shell $(HEX_CONVERT) $(PLATE))" 

drop_car:
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(CLIENT_SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase  $(SECRET_PHRASE) \
		--fn drop_car \
		--arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
		--arg 0 \
		--arg "$(shell $(HEX_CONVERT) $(PLATE))" 

deny_drop:
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase  $(SECRET_PHRASE) \
		--fn deny_drop \
		--arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
		--arg 0 \
		--arg "$(shell $(HEX_CONVERT) $(PLATE))" 

accept_drop:
	soroban invoke \
		--id $(CONTRACT_ID) \
		--secret-key $(SECRET_KEY) \
		--rpc-url $(RPC_URL) \
		--network-passphrase  $(SECRET_PHRASE) \
		--fn accpt_drop \
		--arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
		--arg 0 \
		--arg "$(shell $(HEX_CONVERT) $(PLATE))"