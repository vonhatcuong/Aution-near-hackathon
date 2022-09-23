// use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance};
use std::collections::HashMap;
use std::vec::Vec;
pub type TokenId = u32;
pub type AuctionId = u32;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Auction {
    pub auction_id: AuctionId,  // id of auction
    pub token_id: TokenId,      // Token id
    pub owner_id: AccountId,    // owner of token
    pub started_price: Balance, // 1 NEAR = 10^24 yoctoNEAR
    pub created_at: u64,
    pub is_active: bool,                           // Auction is active or not
    pub participants: HashMap<AccountId, Balance>, // list of participants and their bids
    pub winner: AccountId,                         // winner of auction
    pub win_price: Balance,                        // price of winner
}

impl Auction {
    pub fn calculate_max_amount(&self) -> Balance {
        let prices: Vec<&Balance> = self.participants.values().collect();

        let max_price = prices.iter().max();

        match max_price {
            Some(&max_price) => *max_price,
            None => 0,
        }
    }

    pub fn find_winner(&mut self, price: Balance) {
        let winner = self
            .participants
            .iter()
            .find_map(|(key, &val)| {
                if val == price {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .unwrap();
        self.winner = winner;
    }
}
