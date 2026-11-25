#![no_std]
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, BytesN, Env,
};

const DAY: u32 = 17_280; // ledgers per day (~5s each)
const BUMP_THRESHOLD: u32 = 30 * DAY;
const BUMP_TO: u32 = 120 * DAY;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,                // instance
    Attester(Address),    // persistent: bool
    Count(BytesN<32>),    // persistent: u32 (number of versions for subject)
    Att(BytesN<32>, u32), // persistent: Attestation (append-only)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    pub root: BytesN<32>, // sha256 of canonical (JCS) track record, or Merkle root
    pub schema: u32,      // version of the canonicalization/schema
    pub attester: Address,
    pub ledger_time: u64,
    pub revoked: bool,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotAttester = 1,
    NotFound = 2,
}

#[contractevent]
pub struct Attested {
    #[topic]
    pub subject: BytesN<32>,
    pub version: u32,
    pub root: BytesN<32>,
}

#[contractevent]
pub struct Revoked {
    #[topic]
    pub subject: BytesN<32>,
    pub version: u32,
}

#[contract]
pub struct Registry;

#[contractimpl]
impl Registry {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn set_attester(env: Env, attester: Address, allowed: bool) {
        Self::admin(&env).require_auth();
        let k = DataKey::Attester(attester);
        env.storage().persistent().set(&k, &allowed);
        env.storage().persistent().extend_ttl(&k, BUMP_THRESHOLD, BUMP_TO);
        env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);
    }

    /// Append a new attestation version for a subject. Returns the version (1-based).
    pub fn attest(
        env: Env,
        attester: Address,
        subject: BytesN<32>,
        root: BytesN<32>,
        schema: u32,
    ) -> Result<u32, Error> {
        attester.require_auth();
        let ak = DataKey::Attester(attester.clone());
        if !env.storage().persistent().get::<_, bool>(&ak).unwrap_or(false) {
            return Err(Error::NotAttester);
        }
        let ck = DataKey::Count(subject.clone());
        let version = env.storage().persistent().get::<_, u32>(&ck).unwrap_or(0) + 1;
        let rec = Attestation {
            root: root.clone(),
            schema,
            attester,
            ledger_time: env.ledger().timestamp(),
            revoked: false,
        };
        let rk = DataKey::Att(subject.clone(), version);
        env.storage().persistent().set(&rk, &rec);
        env.storage().persistent().set(&ck, &version);
        env.storage().persistent().extend_ttl(&rk, BUMP_THRESHOLD, BUMP_TO);
        env.storage().persistent().extend_ttl(&ck, BUMP_THRESHOLD, BUMP_TO);
        env.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_TO);
        Attested { subject, version, root }.publish(&env);
        Ok(version)
    }

    pub fn revoke(env: Env, subject: BytesN<32>, version: u32) -> Result<(), Error> {
        Self::admin(&env).require_auth();
        let rk = DataKey::Att(subject.clone(), version);
        let mut rec: Attestation = env.storage().persistent().get(&rk).ok_or(Error::NotFound)?;
        rec.revoked = true;
        env.storage().persistent().set(&rk, &rec);
        Revoked { subject, version }.publish(&env);
        Ok(())
    }

    pub fn count(env: Env, subject: BytesN<32>) -> u32 {
        env.storage().persistent().get(&DataKey::Count(subject)).unwrap_or(0)
    }

    pub fn get(env: Env, subject: BytesN<32>, version: u32) -> Option<Attestation> {
        env.storage().persistent().get(&DataKey::Att(subject, version))
    }

    pub fn latest(env: Env, subject: BytesN<32>) -> Option<Attestation> {
        let n = Self::count(env.clone(), subject.clone());
        if n == 0 { None } else { Self::get(env, subject, n) }
    }

    /// True if root equals the latest, non-revoked attestation for the subject.
    pub fn verify(env: Env, subject: BytesN<32>, root: BytesN<32>) -> bool {
        match Self::latest(env, subject) {
            Some(a) => !a.revoked && a.root == root,
            None => false,
        }
    }

    fn admin(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }
}

#[cfg(test)]
mod test;
