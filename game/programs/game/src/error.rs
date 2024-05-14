use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {

    #[msg("Invalid HTO or SOL Input Amount")]
    InvalidInputAmount,
    #[msg("Insufficient Sol Balance in the Pool")]
    InsufficientSolBalance,
    #[msg("Insufficient HTO Balance in the Pool")]
    InsufficientHtoBalance,
    #[msg("Sol Balance is Less Than Min Amount")]
    LessMinSolAmount,
    #[msg("HTO Balance is Less Than Min Amount")]
    LessMinHtoAmount,
    #[msg("HTO Balance is More Than Max Amount")]
    MoreMaxHtoAmount,
    #[msg("SOL Balance is More Than Max Amount")]
    MoreMaxSolAmount
    
}