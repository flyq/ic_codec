extern crate ic_base_types;
extern crate ledger_canister;

use ic_base_types::{CanisterId, PrincipalId, PrincipalIdParseError, PrincipalIdBlobParseError, CanisterIdError};
use ledger_canister::account_identifier::{AccountIdentifier, Subaccount};
use std::convert::TryInto;

const CRC_LENGTH_IN_BYTES: usize = 4;

fn main() {
    // ledger, ryjl3-tyaaa-aaaaa-aaaba-cai
    let ledger_canister_id: CanisterId = CanisterId::from_u64(2); 
    let ledger_principal_id: PrincipalId = ledger_canister_id.get_ref();
    let ledger_account_id: AccountIdentifier = AccountIdentifier::from(ledger_principal_id);
    let ledger_subaccount: Subaccount = Subaccount::from(&ledger_principal_id);

    let ledger_canister_id_1: CanisterId = canister_from_str("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    assert!(ledger_canister_id, ledger_canister_id_1);
    println!("Ledger canister id: {:?},\nledger canister id slice: {:?},\nprincipal id: {:?},\nprincipal id slice: {:?},\naccount id: {:?},\naccount id slice: {:?},\naccount id hex string: {:?},\nsubaccount: {:?},\nsubaccount slice: {:?},\n", 
                 ledger_canister_id, ledger_canister_id.as_ref(), ledger_principal_id, ledger_principal_id.as_slice(), ledger_account_id, ledger_account_id.to_vec(), ledger_account_id.to_hex(), ledger_subaccount, Vec::from(ledger_subaccount));

    let user_principal_id = principal_from_str("yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae").unwrap();
    let user_account_id: AccountIdentifier = AccountIdentifier::from(user_principal_id);
    let user_subaccount_1: Subaccount = Subaccount::from(&user_principal_id);
    let user_subaccount_2: Subaccount = subaccount_from_principal(&user);
    assert!(user_subaccount_1, user_subaccount_2);
    println!("user principal id: {:?},\nprincipal id slice: {:?},\naccount id: {:?},\naccount id slice: {:?},\naccount id hex string: {:?},\nsubaccount: {:?},\nsubaccount slice: {:?},\n", 
                user_principal_id, user_principal_id.as_slice(), user_account_id, user_account_id.to_vec(), user_account_id.to_hex(); user_subaccount_1, Vec::from(user_subaccount_1));

    let ledger_user = AccountIdentifier::new(ledger_canister_id.get(), Some(user_subaccount_1));
    println!("ledger_user: {:?}, ledger_user string: {:?}", 
            ledger_user, ledger_user.to_hex());
}

fn principal_from_str(input: &str) -> Result<PrincipalId, PrincipalIdParseError> {
    // Strategy: Parse very liberally, then pretty-print and compare output.
    // This is both simpler and yields better error messages.
    let mut s = input.to_string();
    s.make_ascii_lowercase();
    s.retain(|c| c.is_ascii_alphanumeric());
    match base32::decode(base32::Alphabet::RFC4648 { padding: false }, &s) {
        Some(mut bytes) => {
            if bytes.len() < CRC_LENGTH_IN_BYTES {
                return Err(PrincipalIdParseError::TooShort);
            }
            if bytes.len() > PrincipalId::MAX_LENGTH_IN_BYTES + CRC_LENGTH_IN_BYTES {
                return Err(PrincipalIdParseError::TooLong);
            }
            println!("{:?}", bytes);
            let result =
                try_from(&bytes.split_off(CRC_LENGTH_IN_BYTES)[..]).unwrap();
            let expected = format!("{}", result);
            if input != expected {
                return Err(PrincipalIdParseError::Wrong { expected });
            }
            Ok(result)
        }
        None => Err(PrincipalIdParseError::NotBase32),
    }
}

fn canister_from_str(input: &str) -> Result<CanisterId, CanisterIdError> {
    let principal_id =
        principal_from_str(input).map_err(CanisterIdError::PrincipalIdParseError)?;
    CanisterId::new(principal_id)
}


fn try_from(blob: &[u8]) -> Result<PrincipalId, PrincipalIdBlobParseError> {
    // if blob.len() != PrincipalId::MAX_LENGTH_IN_BYTES {
    //     return Err(PrincipalIdBlobParseError::TooLong(blob.len()));
    // }
    let mut data = [0u8; PrincipalId::MAX_LENGTH_IN_BYTES];
    data[..blob.len()].copy_from_slice(&blob[..]);
    println!("{:?}\n", data);
    Ok(PrincipalId::new(blob.len(), data))
}

fn subaccount_from_principal(principal_id: &PrincipalId) -> Subaccount {
    let mut subaccount = [0; std::mem::size_of::<Subaccount>()];
    let principal_id = principal_id.as_slice();
    subaccount[0] = principal_id.len().try_into().unwrap();
    subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);
    Subaccount(subaccount)
}