# Stellar / Soroban technical research (as of 2026-11-24)

Scope: backing tech for "Verifiable track record for Peruvian MYPE" (Soroban attestation registry on testnet + web panel). Every number below was fetched on 2026-11-24 unless marked UNVERIFIED.

## 0. Headline findings (read this first)

1. **The organizer's local guides are one major version behind.** They say `soroban-sdk = "26"`; the registry now has **30.0.0** (published 2026-11-24). Use 30.0.0 (+ stellar-cli 30.x). Source: https://crates.io/api/v1/crates/soroban-sdk
2. **Testnet is already on protocol 30** and **mainnet moved to protocol 30 on 2026-11-23** (yesterday), per https://developers.stellar.org/docs/networks/software-versions.md . Live RPC check below.
3. **`stellar contract bindings typescript` is deprecated.** The CLI itself says to use the JS SDK generator: `npx @stellar/stellar-sdk generate ...`. Sources: https://developers.stellar.org/docs/tools/cli/stellar-cli.md , https://github.com/stellar/js-stellar-sdk (README, "CLI" section).
4. **No Stellar SEP or standard exists for attestations** (checked the full SEP index). Build our own small registry; do not wait for a standard. Source: https://github.com/stellar/stellar-protocol/blob/master/ecosystem/README.md
5. I wrote and ran a minimal registry contract against soroban-sdk 30.0.0: **5/5 tests pass** (native `cargo test`, rustc 1.100.0). Code in section 2 and on disk at `spikes/registry-prototype`. The WASM build (`stellar contract build`) was NOT run (stellar-cli not installed here, wasm target not installed): UNVERIFIED for WASM size.

## 1. Versions and endpoints

| Item | Value | Source (fetched) |
|---|---|---|
| soroban-sdk (Rust) | **30.0.0** (published 2026-11-24; requires rust-version 1.91.0) | https://crates.io/api/v1/crates/soroban-sdk ; https://raw.githubusercontent.com/stellar/rs-soroban-sdk/v30.0.0/Cargo.toml |
| SDK release note | Contracts on SDK 30 must be built with stellar-cli >= 30.0.0. P30 adds muxed contract addresses (CAP-84) and ML-DSA verify (CAP-87). | https://github.com/stellar/rs-soroban-sdk/releases/tag/v30.0.0 |
| stellar-cli | **30.0.1** (released 2026-11-03) | https://github.com/stellar/stellar-cli/releases/latest ; https://crates.io/api/v1/crates/stellar-cli |
| CLI install | `curl -fsSL https://github.com/stellar/stellar-cli/raw/main/install.sh \| sh`, or `brew install stellar-cli`, or `cargo install --locked stellar-cli` | https://developers.stellar.org/docs/tools/cli/install-cli.md |
| Rust target | **`wasm32v1-none`** (`rustup target add wasm32v1-none`, Rust >= 1.84). Output path: `target/wasm32v1-none/release/*.wasm`. | Local guide `contracts-soroban.md`, `common-pitfalls.md`; the SDK 30 workspace needs Rust 1.91. Note: the organizer's `SetUp_Dev.md` deploy example still uses `target/wasm32-unknown-unknown/...`, which contradicts its own target. Use `stellar contract build` and the `wasm32v1-none` path. |
| Testnet protocol | **30** (stellar-core 30.0.0, RPC 30.0.1) | RPC `getNetwork` / `getVersionInfo` on https://soroban-testnet.stellar.org (live call). Docs: testnet P30 since 2026-11-03. |
| Testnet RPC | `https://soroban-testnet.stellar.org` (answers JSON-RPC) | live call |
| Horizon testnet | `https://horizon-testnet.stellar.org` (answers; docs say Horizon is heading to deprecation, prefer RPC) | live call; https://developers.stellar.org/docs/tools/lab/api-explorer/horizon-endpoint.md |
| Passphrase | `Test SDF Network ; September 2015` | RPC `getNetwork` |
| Friendbot | `https://friendbot.stellar.org/?addr=G...` (returns `"successful": true` + tx hash); `stellar keys fund <name> --network testnet` also works | live call (my test funded the public doc example address only) |
| Explorer | https://stellar.expert/explorer/testnet (HTTP 200) | live call. Tx URL pattern `/explorer/testnet/tx/<hash>` and `/contract/<C...>`: UNVERIFIED (not fetched). |
| Testnet resets | "resets on a regular cadence" (no date given). Plan to re-deploy and re-seed with a script; do not rely on testnet data persisting past the hackathon. | https://developers.stellar.org/docs/networks/index.md |
| Fee stats now | Soroban inclusion fee p50 = 100 stroops, p99 = 200 (448 txs / 50 ledgers) | RPC `getFeeStats` (live) |

Docs-lag caveat: the docs "Software Versions" page still lists "Smart Contract Rust SDK 29.0.1" for P30, while crates.io and GitHub already have 30.0.0. Trust the registry.

## 2. Attestation registry contract design

### 2.1 Decisions (with rationale)

- **Storage.** Admin in `instance()` (one shared entry, TTL bumped on each write). Attestations, counters and attester allow-list in `persistent()`, one key per entry (`Att(subject, version)`). Never `temporary()` (expired = deleted permanently). Sources: local `contracts-soroban.md`; https://developers.stellar.org/docs/learn/fundamentals/contract-development/storage/state-archival.md
- **TTL / archival.** `extend_ttl(threshold, extend_to)` only raises TTL when it is below threshold. Anyone can extend any entry (not a security mechanism). Archived persistent entries are auto-restored if the transaction is simulated first (simulation builds the restore footprint); a hand-built tx without simulation fails. Restored entries start at the minimum TTL, so extend after restore. Source: state-archival page above. Network max TTL is ~180 days on mainnet per the local guide (verify with `stellar network settings`; testnet value UNVERIFIED). For a demo, bump on every write and run a small keep-alive script before judging.
- **Constructor.** Use `__constructor(env, admin)`: runs atomically at deploy, removes the init front-running window. Deploy with `stellar contract deploy --wasm ... -- --admin <G...>` (constructor args after `--`; exact flag syntax UNVERIFIED, check `stellar contract deploy --help`). Tests: `env.register(Registry, (admin,))` (verified in the test below).
- **Auth.** `attester.require_auth()` plus an allow-list check (admin-managed). The admin (hackathon team key) can add attesters and revoke. Server-side signer = the attester key.
- **Subject key.** `BytesN<32>` = `sha256(salt || RUC)`, computed off-chain. A RUC is a public identifier (company tax ID), but an unsalted hash of an 11-digit number is trivially reversible by enumeration. Use a project-wide secret/pepper (or per-subject salt shared with the MYPE) so the chain holds no enumerable RUC. Tradeoff: with a secret pepper only our backend can derive the key, so lenders must verify through our API or be given the salted key by the MYPE. Decide this in the product definition.
- **Versioning.** Append-only: `Count(subject)` and `Att(subject, n)`; `latest()` and `verify(subject, root)` check the newest non-revoked version. Old versions stay readable (auditable history).
- **Revocation.** Flag `revoked: bool` on the record (admin-only), plus a `Revoked` event. Records are never deleted.
- **Events.** Use the `#[contractevent]` macro and `.publish(&env)`. The old `env.events().publish(...)` is deprecated (local guides; verified compiling in SDK 30 here). Events are ephemeral on RPC (about 7 days per local guide): do not use them as the source of truth; storage is.
- **Crypto.** `env.crypto().sha256(&Bytes)` exists (used in tests below). `ed25519_verify` exists and panics on invalid signature (SDK 30 notes say the new ML-DSA verifies "work like ed25519_verify"). Not needed for the MVP, because `require_auth` already proves the attester. Add ed25519 only if you want the lender to verify an off-chain signed PDF/JSON receipt: UNVERIFIED design, optional.
- **What goes on-chain.** Only: subject key, 32-byte root, schema version, attester, ledger time, revoked flag. No names, no RUC, no amounts. Compute the root as sha256 of the RFC 8785 canonical JSON, or a Merkle root (section 5).

### 2.2 Contract sketch (compiled and tested on soroban-sdk 30.0.0)

`src/lib.rs`:

```rust
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
```

`src/test.rs`:

```rust
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
```

`Cargo.toml` key lines (full file on disk): `soroban-sdk = "30.0.0"`, dev-dependency with `features = ["testutils"]`, `crate-type = ["lib", "cdylib"]`, release profile with `opt-level = "z"`, `lto = true`, `panic = "abort"`.

Test result observed: `running 5 tests ... test result: ok. 5 passed; 0 failed`.

Testing notes:
- `env.mock_all_auths()` hides a missing `require_auth`; the test `auth_is_required_for_attest` asserts `env.auths()` right after the call (it resets on the next call). Source: local `testing.md`.
- Snapshot tests: the SDK writes `test_snapshots/*.json` per test automatically when `testutils` is enabled (observed behavior in prior SDK versions; I did not inspect the folder in this run: UNVERIFIED for v30). Commit them or add to `.gitignore`, but be consistent.
- Resource limits are enforced in unit tests since SDK 25 (local `testing.md`): UNVERIFIED by me, plausible.
- Write at least: happy path, unauthorized attester, revoke, versioning, TTL, and a "wrong root fails verify" case.

## 3. Prior art and standards

- **SEPs.** The full SEP index has no attestation/credential standard. Relevant adjacent ones: SEP-46 Contract Meta, SEP-48 Contract Interface Spec, **SEP-53 Sign and Verify Messages (Final)** (useful if you want an off-chain signed receipt), SEP-10 auth. Source: https://github.com/stellar/stellar-protocol/blob/master/ecosystem/README.md
- **Projects found via GitHub search (descriptions only; I did NOT audit code or quality, treat as inspiration, not dependencies):**
  - daccred/sorobangraph.attest.so (indexer for "Soroban Attestation Contracts", last push 2025-08-14), https://github.com/daccred/sorobangraph.attest.so
  - Haroldwonder/AnchorKit, anchoring off-chain attestations to Stellar, https://github.com/Haroldwonder/AnchorKit
  - Tybravo/StellarProof, content provenance with Soroban, https://github.com/Tybravo/StellarProof
  - milepost-labs/milepost, disbursement verified by attestation, https://github.com/milepost-labs/milepost
  - OpenZeppelin/stellar-contracts (access control, pausable, upgradeable building blocks), https://github.com/OpenZeppelin/stellar-contracts
- **Recommendation.** Do not depend on a third-party attestation contract for a 10-day hackathon: our schema (subject, root, version, revoke) is about 130 lines, we control it, and the judges see original work. Optionally mention the EAS (Ethereum Attestation Service) model as the conceptual reference (EAS facts not fetched here: UNVERIFIED).

## 4. Frontend and backend

| Item | Version / fact | Source |
|---|---|---|
| @stellar/stellar-sdk | **17.4.0**, Node >= 22.12.0 | https://registry.npmjs.org/@stellar/stellar-sdk/latest |
| @stellar/freighter-api | 6.0.1 | https://registry.npmjs.org/@stellar/freighter-api/latest |
| Stellar Wallets Kit | **2.8.0** (published 2026-10-28). Published both as npm `@creit.tech/stellar-wallets-kit` and on JSR as `@creit-tech/stellar-wallets-kit` (v2 API: static `StellarWalletsKit.init({ modules: defaultModules() })`). Do not mix v1 and v2 imports. | https://registry.npmjs.org/@creit.tech/stellar-wallets-kit ; https://jsr.io/@creit-tech/stellar-wallets-kit/meta.json ; https://github.com/Creit-Tech/Stellar-Wallets-Kit |
| Python stellar-sdk | 17.0.1 | https://pypi.org/pypi/stellar-sdk/json |
| Bindings | `npx @stellar/stellar-sdk generate --contract-id C... --network testnet --output-dir ./registry-client --contract-name registry` (also `--wasm`, `--wasm-hash`). The CLI route `stellar contract bindings typescript` is marked deprecated. | https://github.com/stellar/js-stellar-sdk ; stellar-cli docs |
| Import namespace | `StellarSdk.rpc.Server` (the old `SorobanRpc` is gone) | local `common-pitfalls.md` |

Architecture recommendation (no wallet needed for MVP):
- **Write path (server, Node):** an API route holds the attester secret key in an env var, builds a tx with the generated client (`contract.Client`), simulates, signs, submits, polls for status. Reload the account sequence before each tx (avoid `tx_bad_seq`); serialize writes through one queue. Use `assembleTransaction` after simulation for the real fee.
- **Read path (browser, no wallet):** call read-only methods (`verify`, `latest`, `count`) via `simulateTransaction` against the public RPC; the generated client does this for view calls. Any funded dummy source account is acceptable for simulation (UNVERIFIED nuance: newer RPC may accept a random funded account only; use a real testnet account).
- **Wallet:** only if you want the MYPE or lender to sign something. Not required. If added, use Wallets Kit 2.8.0 (v2 API) or Freighter directly.
- **Browser CORS** to `soroban-testnet.stellar.org`: UNVERIFIED (not tested from a browser). Proxy reads through your backend if CORS blocks.

## 5. Deterministic hashing of the track record

- **Canonicalization:** RFC 8785 JCS. TS: `canonicalize` 5.1.0 (https://registry.npmjs.org/canonicalize/latest) or `json-canonicalize` 3.0.1 (https://registry.npmjs.org/json-canonicalize/latest). Python: `rfc8785` 0.1.4, pure Python (https://pypi.org/pypi/rfc8785/json). Rule of thumb: pick ONE implementation per side and cross-check with a shared test vector in CI (hash of the same JSON computed in TS and Python must match).
- **Pitfalls (known properties of JCS, not fetched):** JCS serializes numbers per ECMAScript rules, so amounts above 2^53 or floats are fragile. Store money as **integer minor units (céntimos) or decimal strings**, dates as ISO-8601 UTC strings, and never include floats. Include `schema_version`, `period`, `source` (OCDS release ids), and `generated_at` inside the hashed document, so the hash binds the data to its version.
- **Hash:** sha256 over the UTF-8 bytes of the canonical string → 32 bytes → the contract's `root: BytesN<32>`.
- **Merkle option:** leaves = sha256 of each canonical contract record (sorted by contract id); on-chain root = Merkle root; the panel can show an inclusion proof ("this contract is part of the attested history"). Libraries: `@openzeppelin/merkle-tree` 1.0.8 or `merkletreejs` 0.6.0 (npm); `pymerkle` 7.0.0 (PyPI). Caveat: use a library with domain-separated leaf/node hashing (OpenZeppelin's double-hashes leaves) to prevent second-preimage attacks. Recommendation: **ship the flat JCS hash for the MVP; add the Merkle root as a stretch goal**, since it adds a second proof format, more tests and UI work for little judging value.

## 6. Costs and gotchas

Costs:
- Testnet XLM is free from Friendbot. Inclusion fee currently ~100 stroops (1 stroop = 0.0000001 XLM); resource fee comes from simulation and has refundable (rent, events) and non-refundable parts. Source: https://developers.stellar.org/docs/learn/fundamentals/fees-resource-limits-metering.md ; live `getFeeStats`.
- Per-attestation cost is dominated by rent for two small persistent entries (record + counter) and is negligible on testnet. Exact stroops per call: UNVERIFIED, measure with `stellar contract invoke ... --send=no` / simulation and put the number in the pitch.
- Current limits: Stellar Lab Network Limits page or `stellar network settings` (https://developers.stellar.org/docs/networks/resource-limits-fees.md).

Pitfalls (merge of local `common-pitfalls.md` with what I observed):
1. Wrong SDK/CLI pairing: SDK 30 needs stellar-cli >= 30. Do not copy "soroban-sdk = 26" from the organizer's guide.
2. Target: use `wasm32v1-none` and `stellar contract build`; ignore the `wasm32-unknown-unknown` path in SetUp_Dev.md.
3. Constructor cannot be added after deploy; design it before the first deploy. Upgrades do not rerun it.
4. `mock_all_auths` hides missing auth: assert `env.auths()`.
5. `env.auths()`/`env.events().all()` reset on the next call; assert immediately.
6. Instance storage is one entry shared by everything in it; keep it tiny (admin only).
7. TTL is not security (anyone can extend); archived entries need simulation to restore.
8. `tx_bad_seq` when reusing an account object; reload sequence per tx and serialize server writes.
9. Always simulate then `assembleTransaction`; submit promptly; poll `getTransaction` until SUCCESS/FAILED.
10. `scValToNative` returns `BigInt` for i128/u64: convert explicitly; JSON.stringify of BigInt throws.
11. Testnet resets: keep a one-command redeploy + reseed script and put the contract id in config.
12. No public SDF mainnet RPC (use a provider); irrelevant for the testnet deadline.
13. Privacy: even salted hashes are linkable (same subject key across versions). Say so in the pitch; do not claim anonymity, claim "no personal data on-chain".
14. Wallets Kit v1 vs v2 API mix-up (v2 is static, `defaultModules()`).

## 7. Open items for the team (cannot be decided from research)

- Subject-key derivation: secret pepper (backend-only) vs per-MYPE salt shared with the lender.
- Who is the attester in the demo: a single project key (simple) vs OECE-like role allow-list (more convincing story).
- Flat hash only, or Merkle proofs too.

## 8. Unverified list

stellar.expert URL patterns; deploy-with-constructor CLI flag syntax; testnet max TTL value; SDK 30 snapshot-file behavior; browser CORS on the public RPC; WASM size of the sketch; per-call fee in stroops; EAS facts; the quality of third-party attestation repos; the ed25519 receipt design.
