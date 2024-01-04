use borsh::{BorshSerialize, BorshDeserialize};


#[derive(BorshDeserialize, BorshSerialize)]
pub struct MovieAccountState {
    pub is_initialized: bool,
    pub rating: u8,
    pub title: String,
    pub description: String
}