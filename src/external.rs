use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract,
    serde::{Deserialize, Serialize},
    AccountId,
};

use near_contract_standards::non_fungible_token::{metadata::TokenMetadata, TokenId};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

// Validator interface, for cross-contract calls
#[ext_contract(hello_near)]
trait HelloNear {
    fn get_greeting(&self) -> String;
    fn set_greeting(&self, greeting: String);
}

#[ext_contract(guestbook)]
trait GuestBook {
    fn add_message(&mut self, text: String);
    fn get_messages(&self) -> Vec<PostedMessage>;
}

#[ext_contract(certificate_issuer)]
trait CertificateIssuer {
    fn nft_mint(&mut self, token_id: TokenId, token_owner_id: AccountId, metadata: TokenMetadata);
}
