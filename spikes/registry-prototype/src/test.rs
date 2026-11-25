extern crate std;
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events as _, Ledger as _},
    Bytes, Env,
};

fn setup() -> (Env, RegistryClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let id = env.register(Registry, (admin.clone(),));
    let client = RegistryClient::new(&env, &id);
    let attester = Address::generate(&env);
    client.set_attester(&attester, &true);
    (env, client, admin, attester)
}

fn subject(env: &Env, ruc: &str) -> BytesN<32> {
    // In production sha256(salt || RUC) is computed off-chain; on-chain here only for the test.
    let b = Bytes::from_slice(env, ruc.as_bytes());
    env.crypto().sha256(&b).into()
}

#[test]
fn attest_versions_and_verify() {
    let (env, c, _a, att) = setup();
    let s = subject(&env, "20123456789");
    let r1 = BytesN::from_array(&env, &[1; 32]);
    let r2 = BytesN::from_array(&env, &[2; 32]);
    assert_eq!(c.attest(&att, &s, &r1, &1), 1);
    assert_eq!(env.events().all().events().len(), 1);
    assert_eq!(c.attest(&att, &s, &r2, &1), 2);
    assert_eq!(c.count(&s), 2);
    assert!(c.verify(&s, &r2));
    assert!(!c.verify(&s, &r1));
    assert_eq!(c.get(&s, &1).unwrap().root, r1);
}

#[test]
fn unauthorized_attester_rejected() {
    let (env, c, _a, _att) = setup();
    let rogue = Address::generate(&env);
    let s = subject(&env, "20123456789");
    let r = BytesN::from_array(&env, &[1; 32]);
    assert_eq!(c.try_attest(&rogue, &s, &r, &1), Err(Ok(Error::NotAttester)));
}

#[test]
fn revoke_invalidates() {
    let (env, c, _a, att) = setup();
    let s = subject(&env, "20123456789");
    let r = BytesN::from_array(&env, &[7; 32]);
    c.attest(&att, &s, &r, &1);
    c.revoke(&s, &1);
    assert!(!c.verify(&s, &r));
    assert_eq!(c.try_revoke(&s, &9), Err(Ok(Error::NotFound)));
}

#[test]
fn auth_is_required_for_attest() {
    let (env, c, _a, att) = setup();
    let s = subject(&env, "1");
    c.attest(&att, &s, &BytesN::from_array(&env, &[3; 32]), &1);
    let auths = env.auths();
    assert_eq!(auths.len(), 1);
    assert_eq!(auths[0].0, att);
}

#[test]
fn ttl_is_extended() {
    use soroban_sdk::testutils::storage::Persistent as _;
    let (env, c, _a, att) = setup();
    let s = subject(&env, "2");
    c.attest(&att, &s, &BytesN::from_array(&env, &[4; 32]), &1);
    let ttl = env.as_contract(&c.address, || {
        env.storage().persistent().get_ttl(&DataKey::Att(s.clone(), 1))
    });
    assert!(ttl >= BUMP_TO - 1);
    env.ledger().set_timestamp(1_000_000);
    assert!(c.latest(&s).is_some());
}
