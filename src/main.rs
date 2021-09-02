extern crate ic_base_types;
extern crate ledger_canister;

use ic_base_types::{CanisterId, PrincipalId, PrincipalIdParseError, PrincipalIdBlobParseError, CanisterIdError};
use ledger_canister::account_identifier::{AccountIdentifier, Subaccount};
use std::convert::TryInto;

const CRC_LENGTH_IN_BYTES: usize = 4;

fn main() {
    // ledger, ryjl3-tyaaa-aaaaa-aaaba-cai
    let ledger_canister_id: CanisterId = CanisterId::from_u64(2); 
    let ledger_principal_id: PrincipalId = *ledger_canister_id.get_ref();
    let ledger_account_id: AccountIdentifier = AccountIdentifier::from(ledger_principal_id);
    let ledger_subaccount: Subaccount = Subaccount::from(&ledger_principal_id);

    let ledger_canister_id_1: CanisterId = canister_from_str("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    assert_eq!(ledger_canister_id, ledger_canister_id_1);
    println!("Ledger canister id: {:?},\nprincipal id: {:?},\nprincipal id slice: {:?},\naccount id: {:?},\naccount id slice: {:?},\naccount id hex string: {:?},\nsubaccount: {:?},\nsubaccount slice: {:?},\n", 
                 ledger_canister_id, ledger_principal_id, ledger_principal_id.as_slice(), ledger_account_id, ledger_account_id.to_vec(), ledger_account_id.to_hex(), ledger_subaccount, Vec::from(ledger_subaccount));

    // user, yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae
    let user_principal_id: PrincipalId = principal_from_str("yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae").unwrap();
    let user_account_id: AccountIdentifier = AccountIdentifier::from(user_principal_id);
    let user_subaccount_1: Subaccount = Subaccount::from(&user_principal_id);
    let user_subaccount_2: Subaccount = subaccount_from_principal(&user_principal_id);
    assert_eq!(user_subaccount_1, user_subaccount_2);
    println!("user principal id: {:?},\nprincipal id slice: {:?},\naccount id: {:?},\naccount id slice: {:?},\naccount id hex string: {:?},\nsubaccount: {:?},\nsubaccount slice: {:?},\n", 
                user_principal_id, user_principal_id.as_slice(), user_account_id, user_account_id.to_vec(), user_account_id.to_hex(), user_subaccount_1, Vec::from(user_subaccount_1));

    let ledger_user = AccountIdentifier::new(ledger_canister_id.get(), Some(user_subaccount_1));
    println!("ledger_user: {:?}, ledger_user string: {:?}", 
            ledger_user, ledger_user.to_hex());

    // r7inp-6aaaa-aaaaa-aaabq-cai
    // b5c38c5c8e63d89751b8f8b3fd9deceda70c281b87957ca44319c9758a3dbef2
    let canister_id_1: CanisterId = canister_from_str("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    println!("canister_id_1: {:?}", canister_id_1.to_hex());

    // cav4d-eaaaa-aaaah-aaqrq-cai
    let canister_id_2: CanisterId = canister_from_str("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    println!("canister_id_2: {:?}", canister_id_2.to_hex());
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
    Ok(PrincipalId::new(blob.len(), data))
}

fn subaccount_from_principal(principal_id: &PrincipalId) -> Subaccount {
    let mut subaccount = [0; std::mem::size_of::<Subaccount>()];
    let principal_id = principal_id.as_slice();
    subaccount[0] = principal_id.len().try_into().unwrap();
    subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);
    Subaccount(subaccount)
}

// output
// Ledger canister id: CanisterId(ryjl3-tyaaa-aaaaa-aaaba-cai),
// principal id: ryjl3-tyaaa-aaaaa-aaaba-cai,
// principal id slice: [0, 0, 0, 0, 0, 0, 0, 2, 1, 1],
// account id: AccountIdentifier { hash: [68, 190, 81, 175, 228, 164, 66, 13, 77, 244, 190, 255, 112, 143, 60, 242, 245, 222, 94, 252, 201, 245, 134, 128, 187, 15, 54, 144] },
// account id slice: [136, 62, 239, 124, 68, 190, 81, 175, 228, 164, 66, 13, 77, 244, 190, 255, 112, 143, 60, 242, 245, 222, 94, 252, 201, 245, 134, 128, 187, 15, 54, 144],
// account id hex string: "883eef7c44be51afe4a4420d4df4beff708f3cf2f5de5efcc9f58680bb0f3690",
// subaccount: Subaccount([10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
// subaccount slice: [10, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],

// user principal id: yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae,
// principal id slice: [152, 239, 24, 172, 140, 12, 215, 142, 44, 41, 97, 216, 243, 158, 45, 26, 194, 160, 168, 254, 119, 25, 239, 130, 215, 66, 209, 166, 2],
// account id: AccountIdentifier { hash: [67, 29, 107, 111, 105, 22, 6, 139, 87, 132, 162, 65, 115, 13, 46, 52, 82, 174, 101, 0, 37, 180, 191, 122, 151, 90, 129, 240] },
// account id slice: [7, 60, 163, 53, 67, 29, 107, 111, 105, 22, 6, 139, 87, 132, 162, 65, 115, 13, 46, 52, 82, 174, 101, 0, 37, 180, 191, 122, 151, 90, 129, 240],
// account id hex string: "073ca335431d6b6f6916068b5784a241730d2e3452ae650025b4bf7a975a81f0",
// subaccount: Subaccount([29, 152, 239, 24, 172, 140, 12, 215, 142, 44, 41, 97, 216, 243, 158, 45, 26, 194, 160, 168, 254, 119, 25, 239, 130, 215, 66, 209, 166, 2, 0, 0]),
// subaccount slice: [29, 152, 239, 24, 172, 140, 12, 215, 142, 44, 41, 97, 216, 243, 158, 45, 26, 194, 160, 168, 254, 119, 25, 239, 130, 215, 66, 209, 166, 2, 0, 0],

// ledger_user: AccountIdentifier { hash: [124, 24, 9, 168, 58, 106, 122, 169, 225, 220, 192, 84, 101, 198, 91, 89, 190, 222, 232, 10, 34, 110, 79, 177, 142, 58, 59, 242] }, ledger_user string: "751915b37c1809a83a6a7aa9e1dcc05465c65b59bedee80a226e4fb18e3a3bf2"