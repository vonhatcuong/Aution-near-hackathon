
## Build
 >./build.sh
 ### OR run command
 > cargo build --all --target wasm32-unknown-unknown --release
 >
 > cp target/wasm32-unknown-unknown/release/auction.wasm ./res/auction.wasm

## Setup for Demo

### 1. Setup Master Account
> ID_MASTER=user_003.testnet ( your_account.testnet )

### 2. Setup sub account
> ID1=sub_acc1.$ID_MASTER
>
> ID2=sub_acc2.$ID_MASTER
>
> ID3=sub_acc3.$ID_MASTER

#### 3. Create fee:
> near create-account $ID1 --masterAccount $ID_MASTER --initialBalance 10
>
> near create-account $ID2 --masterAccount $ID_MASTER --initialBalance 10
>
> near create-account $ID3 --masterAccount $ID_MASTER --initialBalance 10

#### Delete:
> near delete $ID1 $ID_MASTER
>
> near delete $ID2 $ID_MASTER
>
> near delete $ID3 $ID_MASTER

### 2. Check state account:
> near state $ID_MASTER | grep -E "(block_hash|Account|formattedAmount)"
>
> near state $ID1 | grep -E "(block_hash|Account|formattedAmount)"
>
> near state $ID2 | grep -E "(block_hash|Account|formattedAmount)"
>
> near state $ID3 | grep -E "(block_hash|Account|formattedAmount)"

### 3. Deploy contract:
> near deploy --accountId $ID_MASTER --wasmFile ./res/your_contract.wasm

#### 4. Check storage:
> near call $ID_MASTER get_all_auctions --accountId $ID_MASTER
>
> near call $ID_MASTER get_auctions_closed --accountId $ID_MASTER
>
> near call $ID_MASTER get_auction_id '{"auction_id": 1}' --accountId $ID_MASTER
>
> near call $ID_MASTER get_all_tokens --accountId $ID_MASTER
>
> near call $ID_MASTER get_token_by_id '{"token_id": 1}' --accountId $ID_MASTER

#### 5. Bet
### Create Instant Contract:
> near call $ID_MASTER new '{"owner_id": "'$ID_MASTER'"}' --accountId $ID_MASTER

### Mint NFT 1:

> near call $ID_MASTER  mint_nft '{"owner_id":"'$ID_MASTER'","token_data":{"title":"NFT","description":"1"}}' --accountId $ID_MASTER --deposit 0.1
#### Check storage NFT 1:
> > near call $ID_MASTER get_auction_by_id '{"auction_id": 1}' --accountId $ID_MASTER

### Mint NFT 2:

> near call $ID_MASTER  mint_nft '{"owner_id":"'$ID_MASTER'","token_data":{"title":"NFT","description":"2"}}' --accountId $ID_MASTER --deposit 0.1
#### Check storage NFT 2:
> > near call $ID_MASTER get_auction_by_id '{"auction_id": 2}' --accountId $ID_MASTER
### Create auction:
> near call $ID_MASTER create_auction '{"token_id": 1, "price": 6}' --accountId $ID_MASTER --deposit 0.1
#### Check storage Auction_id 1
> near call $ID_MASTER get_auction_by_id '{"auction_id": 1}' --accountId $ID_MASTER
### Bid:
#### User_1
> near call $ID_MASTER bid '{"auction_id": 1}' --accountId $ID1 --deposit 7
##### Check storage
> near state $ID1 | grep -E "(block_hash|Account|formattedAmount)"

#### User_2
> near call $ID_MASTER bid '{"auction_id": 1}' --accountId $ID2 --deposit 8
##### Check storage
> near state $ID2 | grep -E "(block_hash|Account|formattedAmount)"

#### User_3
> near call $ID_MASTER bid '{"auction_id": 1}' --accountId $ID3 --deposit 9
##### Check storage
> near state $ID3 | grep -E "(block_hash|Account|formattedAmount)"
### Check  auction by id_1 of $ID_MASTER
> near call $ID_MASTER get_auction_by_id '{"auction_id": 1}' --accountId $ID_MASTER
#### Close auction:
> near call $ID_MASTER close_auction '{"auction_id": 1}' --accountId $ID_MASTER
