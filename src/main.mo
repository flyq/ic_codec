import Convert "./convert";

actor Tools {
    public query func subaccountToPrincipal(sub: [Nat8]) : async Principal {
        return Convert.subToPrincipal(sub);
    };
};