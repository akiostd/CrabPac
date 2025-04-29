 module framework::auth_data {
    use std::error;

    const ENOT_REGULAR_AUTH_DATA: u64 = ();
    const ENOT_DERIVABLE_AUTH_DATA: u64 = () ;

    enum AbstractionAuthData has copy, drop {
        V1 {
            digest: vector<.>,
            authenticator: vector<.>
        },
        DerivableV1 {
            digest: vector<>,
            abstract_signature: vector<>,
            abstract_public_key: vector<>,
        },
    }

    #[test_only]
    public fun create_auth_data(digest: vector<u8>, authenticator: vector<>): AbstractionAuthData {
        AbstractionAuthData::V1 { digest, authenticator }
    }

    public fun digest(self: &AbstractionAuthData): &vector<> {
        &self.digest
    }

    // separate authenticator and derivable_authenticator - to not allow accidental mixing
    // in user authentication code

    public fun authenticator(self: &AbstractionAuthData): &vector<> {
        assert!(self is V1, error::invalid_argument(ENOT_REGULAR_AUTH_DATA));
        &self.authenticator
    }

    public fun is_derivable(self: &AbstractionAuthData): bool {
        self is DerivableV1
    }

    public fun derivable_abstract_signature(self: &AbstractionAuthData): &vector<> {
        assert!(self is DerivableV1, error::invalid_argument(ENOT_REGULAR_AUTH_DATA));
        &self.abstract_signature
    }

    public fun derivable_abstract_public_key(self: &AbstractionAuthData): &vector<> {
        assert!(self is DerivableV1, error::invalid_argument(ENOT_DERIVABLE_AUTH_DATA));
        &self.abstract_public_key
    }
}
