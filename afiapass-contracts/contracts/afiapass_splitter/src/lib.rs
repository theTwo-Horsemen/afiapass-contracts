#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, String};

#[contract]
pub struct AfiaPassSplitter;

#[contractimpl]
impl AfiaPassSplitter {
    
    /// Issues a permit and calculates the exact tax routing splits
    pub fn issue_permit_and_split(
        env: Env, 
        rider_id: String, 
        route_id: String, 
        amount: i128
    ) -> String {
        
        // 1. Precision Math: Calculate the exact splits
        // We multiply by the percentage first, then divide by 100 to maintain i128 integer precision
        let lga_share = (amount * 5) / 100;
        let union_share = (amount * 5) / 100;
        
        // We calculate the vendor share by subtracting the others from the total. 
        // This guarantees that 100% of the funds are accounted for, even if a fraction 
        // of a Stroop gets truncated in the division above!
        let vendor_share = amount - lga_share - union_share;

        // TODO: Look up actual wallet addresses from contract storage
        // TODO: Perform the actual NGNC token transfers using soroban_sdk::token::Client

        // 2. Emit the 'PermitIssued' Event
        // The Stellar network stores this event on the ledger. 
        // Your Java SDK will read this to generate the offline JWT!
        env.events().publish(
            (symbol_short!("Permit"), rider_id.clone(), route_id.clone()),
            (amount, lga_share, union_share, vendor_share),
        );
        
        // 3. Return a success confirmation
        String::from_str(&env, "SUCCESS_SPLIT_CALCULATED")
    }
}