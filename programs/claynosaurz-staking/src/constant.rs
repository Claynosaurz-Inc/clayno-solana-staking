// Experience and Level Constants
/// Experience points gained per second of staking.
pub const EXPERIENCE_FOR_SECOND: u64 = 32;
/// Maximum level a user can achieve.
pub const MAX_LEVEL: u8 = 25;
/// Points required to reach level 25.
pub const LEVEL_25_POINTS: u64 = 1_000_000_000;

// Address Constants
/// Address of the Clayno NFT collection.
#[cfg(not(feature = "mainnet"))]
pub const CLAYNO_COLLECTION_ADDRESS: &str = "CQQh6ujjzQSoBmTJPmqN3VUzSMSJXpGGxKtMkqfvJVLN";
#[cfg(feature = "mainnet")]
pub const CLAYNO_COLLECTION_ADDRESS: &str = "6mszaj17KSfVqADrQj3o4W3zoLMTykgmV37W4QadCczK";
/// Address of the Clayno NFT collection.
#[cfg(not(feature = "mainnet"))]
pub const SAGA_COLLECTION_ADDRESS: &str = "AFsQpg1wvMah361EYs6pecWWHvzcAa6TdezXeEhkGseU";
#[cfg(feature = "mainnet")]
pub const SAGA_COLLECTION_ADDRESS: &str = "1yPMtWU5aqcF72RdyRD5yipmcMRC8NGNK59NvYubLkZ";
/// Address of the admin account.
#[cfg(not(feature = "mainnet"))]
pub const ADMIN_ADDRESS: &str = "CLAYripuPYnM4s3YEMxGxXTEKUb7C87mUnPuxoiSzX9H";
#[cfg(feature = "mainnet")]
pub const ADMIN_ADDRESS: &str = "CLAYripuPYnM4s3YEMxGxXTEKUb7C87mUnPuxoiSzX9H";


// Seed Constants
/// Seed for generating the authority PDA.
pub const AUTHORITY_SEED: &str = "auth";
/// Seed for generating the staking account PDA.
pub const STAKING_ACCOUNT_SEED: &str = "staking";
/// Seed for generating the class PDA.
pub const CLASS_PDA_SEED: &str = "class";

