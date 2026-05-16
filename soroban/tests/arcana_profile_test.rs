use soroban_sdk::{Env, Address, Symbol, BytesN, String};
use soroban_sdk::testutils::Address as TestAddress;

// Import the contract
use arcana_contract::ArcanaProfileContract;

#[test]
fn test_create_profile() {
    let env = Env::default();
    
    // Create test accounts
    let owner = Address::generate(&env);
    
    // Initialize contract
    ArcanaProfileContract::initialize(&env);
    
    // Create profile
    let avatar_hash = BytesN::<32>::from_array(&env, &[0u8; 32]);
    let profile_id = ArcanaProfileContract::create_profile(
        &env,
        owner.clone(),
        String::from_str(&env, "Test User"),
        avatar_hash,
    );
    
    // Verify profile was created
    assert!(profile_id > 0);
    
    // Get and verify profile
    let profile = ArcanaProfileContract::get_profile(&env, profile_id);
    assert_eq!(profile.profile_id, profile_id);
    assert_eq!(profile.owner, owner);
    assert_eq!(profile.display_name, String::from_str(&env, "Test User"));
}

#[test]
fn test_update_profile() {
    let env = Env::default();
    
    // Create test accounts
    let owner = Address::generate(&env);
    
    // Initialize contract
    ArcanaProfileContract::initialize(&env);
    
    // Create profile
    let avatar_hash = BytesN::<32>::from_array(&env, &[0u8; 32]);
    let profile_id = ArcanaProfileContract::create_profile(
        &env,
        owner.clone(),
        String::from_str(&env, "Test User"),
        avatar_hash,
    );
    
    // Update profile
    let new_avatar_hash = BytesN::<32>::from_array(&env, &[1u8; 32]);
    ArcanaProfileContract::update_profile(
        &env,
        profile_id,
        String::from_str(&env, "Updated User"),
        new_avatar_hash,
    );
    
    // Verify update
    let profile = ArcanaProfileContract::get_profile(&env, profile_id);
    assert_eq!(profile.display_name, String::from_str(&env, "Updated User"));
    assert_eq!(profile.avatar_hash, new_avatar_hash);
}
