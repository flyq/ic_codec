/**
 * Module     : types.mo
 * Copyright  : 2021 DFinance Team
 * License    : Apache 2.0 with LLVM Exception
 * Maintainer : DFinance Team <hello@dfinance.ai>
 * Stability  : Experimental
 */

import Principal "mo:base/Principal";
import Iter "mo:base/Iter";
import Nat8 "mo:base/Nat8";
import Char "mo:base/Char";
import Array "mo:base/Array";
import Nat32 "mo:base/Nat32";
import Nat "mo:base/Nat";
import Text "mo:base/Text";
import Base32 "./base32";
import Crc32 "./crc32";
import Prim "mo:⛔";

module {
    // todo: this should go to Blob once they add Principal.fromBlob
    func bytesToPrincipal(_bytes: [Nat8]) : Principal {
        var res: [Nat8] = [];
        res := Array.append(res, Crc32.crc32(_bytes));
        res := Array.append(res, _bytes);
        let s = Base32.encode(#RFC4648 {padding=false}, res);
        // let lowercase_s = make_ascii_lowercase(s);
        let lowercase_s = Text.map(s , Prim.charToLower);
        let len = lowercase_s.size();
        let s_slice = Iter.toArray(Text.toIter(lowercase_s));
        var ret = "";
        for (i in Iter.range(0, len-1)) {
            ret := ret # Char.toText(s_slice[i]);
            if ((i+1) % 5 == 0 and i !=len-1) {
                ret := ret # "-";
            };
        };
        return Principal.fromText(ret);
    };

    // func make_ascii_lowercase(a: Text) : Text {
    //     var bytes: [var Nat8] = Iter.toArrayMut(Iter.map<Char, Nat8>(Text.toIter(a), char_to_nat8));
    //     for (i in bytes.keys()) {
    //         bytes[i] := to_ascii_lowercase(bytes[i]);
    //     };
    //     var res = "";
    //     for (v in  Iter.map<Nat8, Char>(bytes.vals(), nat8_to_char)) {
    //         res := res # Char.toText(v);
    //     };
    //     return res;
    // };

    // func char_to_nat8(a: Char) : Nat8 {
    //     return Nat8.fromNat(Nat32.toNat(Char.toNat32(a)));
    // };

    // func nat8_to_char(a: Nat8) : Char {
    //     return Char.fromNat32(Nat32.fromNat(Nat8.toNat(a)));
    // };

    // func to_ascii_lowercase(a: Nat8) : Nat8 {
    //     if (is_ascii_uppercase(Char.fromNat32(Nat32.fromNat(Nat8.toNat(a))))) {
    //         return 32 ^ a;
    //     } else {
    //         return a;
    //     };
    // };

    // func is_ascii_uppercase(a: Char) : Bool {
    //     return (a >= 'A' and a <= 'Z');
    // };
    
    public func subToPrincipal(a: [Nat8]) : Principal {
        let len : Nat = Nat.min(Nat8.toNat(a[0]), a.size()-1);
        var bytes : [var Nat8] = Array.init<Nat8>(len, 0);
        for (i in Iter.range(1, len)) {
            bytes[i-1] := a[i];
        };
        return bytesToPrincipal(Array.freeze(bytes));
    };
};