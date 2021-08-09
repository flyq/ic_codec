# IC_codec

The relationships of PrincipalId, CanisterId, AccountIdentifier, Subaccount.

## result
### in rust

```sh
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
```

### in motoko
```sh
dfx canister --no-wallet call tools subaccountToPrincipal '(vec { 29;152;239;24;172;140;12;215;142;44;41;97;216;243;158;45;26;194;160;168;254;119;25;239;130;215;66;209;166;2;0;0})'
(principal "yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae")

dfx canister --no-wallet call tools subaccountToPrincipal '(vec {10;0;0;0;0;0;0;0;2;1;1;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;})'
(principal "ryjl3-tyaaa-aaaaa-aaaba-cai")
```