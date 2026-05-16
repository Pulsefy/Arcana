use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol, Vec, Map, BytesN, String};
use soroban_sdk::token::TokenClient;

// Struct to represent an Arcana Profile
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[contracttype]
pub struct ArcanaProfile {
    pub profile_id: u64,
    pub owner: Address,
    pub display_name: String,
    pub avatar_hash: BytesN<32>,
    pub created_at: u64,
    pub updated_at: u64,
}

// Struct to represent Arcana Score metrics
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[contracttype]
pub struct ArcanaScore {
    pub score: u32,
    pub reputation_points: u64,
    pub trust_level: u8, // 0-5 scale
    pub last_updated: u64,
}

// Main contract implementation
#[contract]
pub struct ArcanaProfileContract;

#[contractimpl]
impl ArcanaProfileContract {
    // Initialize the contract
    pub fn initialize(env: Env) {
        // Store contract admin
        let admin = env.current_contract_address();
        env.storage().persistent().set(&Symbol::new(&env, "admin"), &admin);
    }

    // Create a new Arcana Profile
    pub fn create_profile(env: Env, owner: Address, display_name: String, avatar_hash: BytesN<32>) -> u64 {
        // Check if caller is authorized
        owner.require_auth();
        
        // Generate profile ID (simple incrementing counter)
        let counter_key = Symbol::new(&env, "profile_counter");
        let mut counter = env.storage().persistent().get::<Symbol, u64>(&counter_key).unwrap_or(0);
        counter += 1;
        env.storage().persistent().set(&counter_key, &counter);
        
        // Create profile
        let profile = ArcanaProfile {
            profile_id: counter,
            owner,
            display_name,
            avatar_hash,
            created_at: env.time().timestamp(),
            updated_at: env.time().timestamp(),
        };
        
        // Store profile
        let profile_key = Symbol::new(&env, "profile");
        env.storage().persistent().set(&profile_key, &profile);
        
        // Initialize default Arcana Score
        let score = ArcanaScore {
            score: 1000,
            reputation_points: 0,
            trust_level: 1,
            last_updated: env.time().timestamp(),
        };
        
        // Store score
        let score_key = Symbol::new(&env, "score");
        env.storage().persistent().set(&score_key, &score);
        
        counter
    }

    // Get Arcana Profile by ID
    pub fn get_profile(env: Env, profile_id: u64) -> ArcanaProfile {
        let profile_key = Symbol::new(&env, "profile");
        env.storage().persistent().get::<Symbol, ArcanaProfile>(&profile_key).unwrap()
    }

    // Update Arcana Profile
    pub fn update_profile(env: Env, profile_id: u64, display_name: String, avatar_hash: BytesN<32>) {
        // Get existing profile
        let profile_key = Symbol::new(&env, "profile");
        let mut profile = env.storage().persistent().get::<Symbol, ArcanaProfile>(&profile_key).unwrap();
        
        // Verify ownership
        profile.owner.require_auth();
        
        // Update fields
        profile.display_name = display_name;
        profile.avatar_hash = avatar_hash;
        profile.updated_at = env.time().timestamp();
        
        // Store updated profile
        env.storage().persistent().set(&profile_key, &profile);
    }

    // Get Arcana Score
    pub fn get_score(env: Env, profile_id: u64) -> ArcanaScore {
        let score_key = Symbol::new(&env, "score");
        env.storage().persistent().get::<Symbol, ArcanaScore>(&score_key).unwrap()
    }

    // Update Arcana Score based on activity
    pub fn update_score(env: Env, profile_id: u64, reputation_points: u64, trust_level: u8) {
        // Get existing score
        let score_key = Symbol::new(&env, "score");
        let mut score = env.storage().persistent().get::<Symbol, ArcanaScore>(&score_key).unwrap();
        
        // Update score logic (simplified)
        score.reputation_points += reputation_points;
        score.trust_level = trust_level;
        score.last_updated = env.time().timestamp();
        
        // Calculate score (basic formula)
        score.score = 1000 + (reputation_points as u32) / 100 + (trust_level as u32) * 100;
        
        // Store updated score
        env.storage().persistent().set(&score_key, &score);
    }
}
