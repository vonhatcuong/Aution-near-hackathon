use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};
use std::collections::HashMap;

near_sdk::setup_alloc!();

mod auction;
mod nft;

use auction::{Auction, AuctionId, TokenId};
use nft::{Token, TokenData};

const TRANSFER_FEE: Balance = 1_000_000_000_000_000_000_000;
const MINT_FEE: Balance = 1_00_000_000_000_000_000_000_000; // 0.1 Near
const CREATE_AUCTION_FEE: Balance = 1_00_000_000_000_000_000_000_000; // 0.1 Near

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,                             // owner of contract
    pub token_id: TokenId,                               // id of token
    pub auction_id: AuctionId,                           // id of auction
    pub auction_by_id: UnorderedMap<AuctionId, Auction>, // list of auction
    pub auction_by_owner: UnorderedMap<AccountId, Vec<AuctionId>>, // list of auction by owner
    pub token_by_id: UnorderedMap<TokenId, Token>,       // list of token
    pub auction_is_going: Vec<AuctionId>,                // list of auction is going
    pub auction_closed: Vec<AuctionId>,                  // list of auction closed
}

#[near_bindgen]
impl Contract {
    /// Initial smart contract
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            token_id: 0,
            auction_id: 0,
            auction_by_id: UnorderedMap::new(b"auction_by_id".to_vec()),
            auction_by_owner: UnorderedMap::new(b"auction_by_owner".to_vec()),
            token_by_id: UnorderedMap::new(b"token_by_id".to_vec()),
            auction_is_going: Vec::new(),
            auction_closed: Vec::new(),
        }
    }

    /// Mint a token
    #[payable]
    pub fn mint_nft(&mut self, owner_id: AccountId, token_data: TokenData) {
        let amount = env::attached_deposit();
        assert_eq!(amount, MINT_FEE, "Need 0.1N to mint nft");

        self.token_id += 1;
        let token = Token {
            owner_id: owner_id,            // owner of token
            token_id: self.token_id,       // id of token
            is_active: false,              // token is not active
            tokendata: token_data.clone(), // token data
        };

        self.token_by_id.insert(&self.token_id, &token); // insert token to list token
    }

    /// Transfer token to new account
    #[private]
    fn transfer_nft(&mut self, new_owner_id: AccountId, token_id: TokenId) {
        assert!(
            env::is_valid_account_id(new_owner_id.as_bytes()),
            "Not valid account id" // check new owner id is valid
        );

        let mut token = self.token_by_id.get(&token_id).unwrap(); // get token by id

        let sender = env::predecessor_account_id(); // get sender id
        if sender != self.owner_id && sender != token.owner_id {
            env::panic("You can't transfer NFT".as_bytes()); // check sender is owner of token
        }

        token.transfer(new_owner_id.clone()); // transfer token to new owner
        self.token_by_id.insert(&token_id, &token); // insert token to list token
    }

    /// Create an anction to bid
    #[payable]
    pub fn create_auction(&mut self, token_id: TokenId, price: Balance) {
        let amount = env::attached_deposit(); // get amount of sender
        assert_eq!(amount, CREATE_AUCTION_FEE, "Need 0.1N to create an auction"); // check amount is 0.1N
        let mut token = self.token_by_id.get(&token_id).unwrap(); // get token by id

        let account_id = env::predecessor_account_id(); // get sender id
        assert_eq!(token.owner_id, account_id, "You don't owner this token"); // check sender is owner of token
        assert_eq!(token.is_active, false, "This token is active"); // check token is active

        self.auction_id += 1; // increase auction id
        let auction = Auction {
            auction_id: self.auction_id,
            token_id: token.token_id,
            owner_id: account_id,
            started_price: price * 1_000_000_000_000_000_000_000_000,
            created_at: env::block_timestamp(),
            is_active: true,
            participants: HashMap::new(),
            winner: AccountId::new(),
            win_price: 0,
        }; // create auction

        token.is_active = true; // set token is active
        self.token_by_id.insert(&token_id, &token); // insert token to list token
        self.auction_is_going.push(self.auction_id); // push auction id to list auction is going
        self.auction_by_id.insert(&self.auction_id, &auction); // insert auction to list auction
    }

    // Close auction and reveal amount
    #[payable]
    pub fn close_auction(&mut self, auction_id: AuctionId) {
        let mut auction = self.auction_by_id.get(&auction_id).unwrap_or_else(|| {
            env::panic("This auction does not exists!".as_bytes()); // check auction is exist
        }); // get auction by id

        let account_id = env::predecessor_account_id(); // get sender id

        if account_id != self.owner_id && account_id != auction.owner_id {
            env::panic("You can't close auction".as_bytes()); // check sender is owner of auction
        }

        auction.is_active = false; // set auction is not active
        let max_price = auction.calculate_max_amount(); // get max price of auction
        auction.find_winner(max_price); // find winner of auction

        auction.win_price = max_price; // set win price of auction
        self.transfer_ft_to_seller(auction.owner_id.clone(), max_price); // transfer ft to seller

        let participants = auction
            .participants
            .clone()
            .into_iter()
            .filter(|&(_, val)| val != max_price)
            .collect(); // get participants of auction

        self.transfer_ft_back_to_participants(participants); // transfer ft back to participants

        self.transfer_nft(auction.winner.clone(), auction.token_id); // transfer token to winner
        self.auction_by_id.insert(&auction_id, &auction); // insert auction to list auction

        self.auction_is_going.retain(|&x| x != auction_id); // remove auction id from list auction is going
        self.auction_closed.push(auction_id); // push auction id to list auction closed
    }

    /// Bid to auction
    #[payable]
    pub fn bid(&mut self, auction_id: AuctionId) {
        let mut auction = self.auction_by_id.get(&auction_id).unwrap_or_else(|| {
            env::panic("This auction does not exists!".as_bytes()); // check auction is exist
        }); // get auction by id

        assert_eq!(auction.is_active, true, "This auction is not active"); // check auction is active

        let amount = env::attached_deposit(); // get amount of sender
        let account_id = env::predecessor_account_id(); // get sender id

        assert_eq!(
            amount > auction.started_price,
            true,
            "The price must be greater than current price"
        ); // check amount is greater than current price

        assert!(
            !auction.participants.contains_key(&account_id),
            "You have Already bid"
        );
        let mut token = self.token_by_id.get(&auction.token_id).unwrap(); // get token by id
        token.is_active = false; // set token is not active
        self.token_by_id.insert(&auction.token_id, &token); // insert token to list token

        auction.participants.insert(account_id, amount); // insert participant to list participant
        self.auction_by_id.insert(&auction_id, &auction); // insert auction to list auction
    }

    pub fn get_all_auctions(&self) -> Vec<AuctionId> {
        self.auction_is_going.clone() // get all auction is going
    }

    pub fn get_auctions_closed(&self) -> Vec<AuctionId> {
        self.auction_closed.clone() // get all auction closed
    }

    pub fn get_all_tokens(&self) -> Vec<TokenId> {
        self.token_by_id.iter().map(|(k, _)| k).collect() // get all token
    }

    pub fn get_auction_id(&self, auction_id: AuctionId) -> Option<Auction> {
        self.auction_by_id.get(&auction_id) // get auction by id
    }

    pub fn get_token_by_id(&self, token_id: TokenId) -> Option<Token> {
        self.token_by_id.get(&token_id) // get token by id
    }

    #[private]
    fn transfer_ft_back_to_participants(&self, _participants: HashMap<AccountId, Balance>) {
        for (account, balance) in _participants.iter() {
            let account = Promise::new(account.clone()); // get account
            account.transfer(balance - TRANSFER_FEE); // transfer ft to account
        }
    }

    #[private]
    fn transfer_ft_to_seller(&self, _owner_id: AccountId, _auction_price: Balance) {
        let account = Promise::new(_owner_id.into()); // get account
        account.transfer(_auction_price); // transfer ft to account
    }
}
