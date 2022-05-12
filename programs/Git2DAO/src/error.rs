use anchor_lang::error_code;

#[error_code]
pub enum Github2DAOError {
    ExcessOSDPercError,
    InvalidContributionError,
    FRTExhuastedError,
    
}