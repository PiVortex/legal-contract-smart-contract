use near_sdk::{env, log, near, require, AccountId, PanicOnDefault};

#[near(contract_state)]
#[near(serializers = [json])]
#[derive(PanicOnDefault)]
pub struct Contract {
    party_1: AccountId,
    party_2: AccountId,
    contract_terms: String,
    party_1_signed: bool,
    party_2_signed: bool,
}

#[near]
impl Contract {
    #[init]
    #[private]
    pub fn init(party_1: AccountId, party_2: AccountId, contract_terms: String) -> Self {
        Self {
            party_1,
            party_2,
            contract_terms,
            party_1_signed: false,
            party_2_signed: false,
        }
    }

    pub fn sign_contract(&mut self) {
        let signer = env::predecessor_account_id();
        if signer == self.party_1 {
            require!(
                !self.party_1_signed,
                "Party 1 has already signed the contract"
            );
            self.party_1_signed = true;
            log!("Party 1 signed the contract");
            if self.party_2_signed {
                log!("Contract is signed by both parties");
            }
        } else if signer == self.party_2 {
            require!(
                !self.party_2_signed,
                "Party 2 has already signed the contract"
            );
            self.party_2_signed = true;
            log!("Party 2 signed the contract");
            if self.party_1_signed {
                log!("Contract is signed by both parties");
            }
        } else {
            panic!("You are not authorized to sign this contract");
        }
    }

    pub fn get_contract_state(&self) -> &Contract {
        self
    }
}
