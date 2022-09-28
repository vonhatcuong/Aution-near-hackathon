
## Build
 >./build.sh
 ### OR run command
 > cargo build --all --target wasm32-unknown-unknown --release
 >
 > cp target/wasm32-unknown-unknown/release/auction.wasm ./res/auction.wasm

## Setup for Demo

### 1. Setup Master Account
> SELLER=seller_uit.testnet ( your_account.testnet )

### 2. Setup sub account
> ID1=sub_acc1.$SELLER
>
> ID2=sub_acc2.$SELLER
>
> ID3=sub_acc3.$SELLER

#### 3. Create fee:
> near create-account $ID1 --masterAccount $SELLER --initialBalance 10
>
> near create-account $ID2 --masterAccount $SELLER --initialBalance 10
>
> near create-account $ID3 --masterAccount $SELLER --initialBalance 10

#### Delete:
> near delete $ID1 $SELLER
>
> near delete $ID2 $SELLER
>
> near delete $ID3 $SELLER

### 2. Check state account:
> near state $SELLER | grep -E "(block_hash|Account|formattedAmount)"
>
> near state $ID1 | grep -E "(block_hash|Account|formattedAmount)"
>
> near state $ID2 | grep -E "(block_hash|Account|formattedAmount)"
>
> near state $ID3 | grep -E "(block_hash|Account|formattedAmount)"

### 3. Deploy contract:
> near deploy --accountId $SELLER --wasmFile ./res/your_contract.wasm

#### 4. Check storage:
> near call $SELLER get_all_auctions --accountId $SELLER
>
> near call $SELLER get_auctions_closed --accountId $SELLER
>
> near call $SELLER get_auction_id '{"auction_id": 1}' --accountId $SELLER
>
> near call $SELLER get_all_tokens --accountId $SELLER
>
> near call $SELLER get_token_by_id '{"token_id": 1}' --accountId $SELLER

#### 5. Bet
### Create Instant Contract:
> near call $SELLER new '{"owner_id": "'$SELLER'"}' --accountId $SELLER

### Mint NFT 1:

> near call $SELLER  mint_nft '{"owner_id":"'$SELLER'","token_data":{"title":"NFT","description":"1"}}' --accountId $SELLER --deposit 0.1
#### Check storage NFT 1:
>  near call $SELLER get_token_by_id '{"token_id": 1}' --accountId $SELLER

### Mint NFT 2:

> near call $SELLER  mint_nft '{"owner_id":"'$SELLER'","token_data":{"title":"NFT","description":"2"}}' --accountId $SELLER --deposit 0.1
#### Check storage NFT 2:
>  near call $SELLER get_token_by_id '{"token_id": 2}' --accountId $SELLER
### Create auction:
> near call $SELLER create_auction '{"token_id": 1, "price": 6}' --accountId $SELLER --deposit 0.1
#### Check storage Auction_id 1
> near call $SELLER get_auction_id '{"auction_id": 1}' --accountId $SELLER
### Bid:
#### User_1
> near call $SELLER bid '{"auction_id": 1}' --accountId $ID1 --deposit 7
##### Check storage
> near state $ID1 | grep -E "(block_hash|Account|formattedAmount)"

#### User_2
> near call $SELLER bid '{"auction_id": 1}' --accountId $ID2 --deposit 8
##### Check storage
> near state $ID2 | grep -E "(block_hash|Account|formattedAmount)"

#### User_3
> near call $SELLER bid '{"auction_id": 1}' --accountId $ID3 --deposit 9
##### Check storage
> near state $ID3 | grep -E "(block_hash|Account|formattedAmount)"
### Check  auction by id_1 of $SELLER
> near call $SELLER get_auction_by_id '{"auction_id": 1}' --accountId $SELLER
#### Close auction:
> near call $SELLER close_auction '{"auction_id": 1}' --accountId $SELLER
