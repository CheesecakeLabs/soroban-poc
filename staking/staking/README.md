# Staking contract

This is a contract that allows users to stake tokens inside a contract and earn reward tokens.

There is only one type of user, who needs to approve the contract to stake the token. The user earn some fee based on the time staked. This fee is payed with rewards token, so it's important the contract have the amount of this reward token to be paid.

The staking token and the reward token are represented by an ERC20 token.

The basic flow of use of the contract is illustrated bellow.

![Basic flow](images/basic-flow.png)



## Methods

![Main functions](images/main-fn.png)

Each method of the contract will be described as follows.

### `initialize`

This method is used to set the initial settings of the contract. Can be called by anyone and must be called before all other functions and only once. This method will define what is the staking token, the reward token and the reward rate to be paid.

The reward token is issued with an ERC20 contract where the current contract is the administrator.

Params:

- `staking_token_id`: Contract address of the token that will be used to stake. Must be in ERC20 standard.
- `rewards_token_id`: Contract address of the token that will be used as rewards. Must be in ERC20 standard.
- `rate`: The reward rate distributed to users per second.


### `stake`

User deposit an amount of stake token in the contract. (user send to contract)

Pré-Conditions:

User needs to approve the contract to stake the token.

Params:

- `amount`: The amount of the stake token

### `withdraw`

User can request to withdraw some stake token amount. (contract send to user)

Params:

- `amount`: The amount of stake token to be withdraw

### `claim_rwrd`

Returns to user address the total amount of rewards token earned. (contract send to user)

### `earned`

Returns the amount of the user rewards earned.

### `get_staked`

Return the amount of stake token the user have inside the contract.

### `tot_supply`

Return the total supply of stake token inside the contract. (sum of all user stake tokens)


## Rewards per token

The rewards per token is refreshed each time some user interact with the contract.

It is calculated according to the total stake token supply, the amount of reward token stored and the timestamp of the contract.

The reward rate defines how many reward tokens will be distributed per stake token.

