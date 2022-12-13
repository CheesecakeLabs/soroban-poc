admin 

Public Key	GAB3PMMKIFQKNCSTHDHWXODHDKTQWA7QDPWSW34M4PW3CI2QSJHG2DVP
Secret Key	SB6MSYRC4BXKKG4SDGN62UXJ53V6JOO2RKAQI3AS5LD62YCSK5C7ZUF2

Client 
Public Key	GDATPTFFUFTGVOIXOGQ22OTT56BDAOEJCCZTKGIGOCFDWVLRNE6X44YK
Secret Key	SDC5QPAJFTWEWXP2HN2JAD7TCKGLYY5FLXOSZXWK3DTJBZCFLH36DZDZ

1. deploy 

  soroban deploy \
    --wasm target/wasm32-unknown-unknown/release/car_rental.wasm \
    --secret-key SB6MSYRC4BXKKG4SDGN62UXJ53V6JOO2RKAQI3AS5LD62YCSK5C7ZUF2 \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Standalone Network ; February 2017'

2. id do contrato = 4a87f349312b29fafc3fb3339730de217e23b0d1f704c95909ea3eca7cc27b59
3. usando o script = Ok(PublicKeyEd25519(StrkeyPublicKeyEd25519([3, 183, 177, 138, 65, 96, 166, 138, 83, 56, 207, 107, 184, 103, 26, 167, 11, 3, 240, 27, 237, 43, 111, 140, 227, 237, 177, 35, 80, 146, 78, 109])))
4. usando o script em Go = Encoded Hex String:  03b7b18a4160a68a5338cf6bb8671aa70b03f01bed2b6f8ce3edb12350924e6d


5. 
soroban invoke \
    --id 4a87f349312b29fafc3fb3339730de217e23b0d1f704c95909ea3eca7cc27b59 \
    --secret-key SB6MSYRC4BXKKG4SDGN62UXJ53V6JOO2RKAQI3AS5LD62YCSK5C7ZUF2 \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Standalone Network ; February 2017' \
    --fn init \
    --arg '{"object":{"vec":[{"symbol":"Account"},{"object":{"accountId":{"publicKeyTypeEd25519":"03b7b18a4160a68a5338cf6bb8671aa70b03f01bed2b6f8ce3edb12350924e6d"}}}]}}'

6. invoke add carro
OBS: VALORES DEVEM SER EM BYTES JUNTOS


soroban invoke \
    --id 4a87f349312b29fafc3fb3339730de217e23b0d1f704c95909ea3eca7cc27b59 \
    --secret-key SB6MSYRC4BXKKG4SDGN62UXJ53V6JOO2RKAQI3AS5LD62YCSK5C7ZUF2 \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Standalone Network ; February 2017' \
    --fn add_car --arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' --arg 0 --arg "49485638363139"  --arg "476f6c" --arg "426c7565" --arg 90 

7. ler carro 

soroban invoke \
    --id 4a87f349312b29fafc3fb3339730de217e23b0d1f704c95909ea3eca7cc27b59 \
    --secret-key SB6MSYRC4BXKKG4SDGN62UXJ53V6JOO2RKAQI3AS5LD62YCSK5C7ZUF2 \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Standalone Network ; February 2017' \
    --fn read_car --arg "49485638363139"
RETORNO DO VALORES SERA EM DECIMAIS 





