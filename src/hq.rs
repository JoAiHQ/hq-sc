#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait Hq {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(burnForCredits)]
    fn burn_for_credits_endpoint(&self, team: ManagedBuffer) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt().clone();

        self.tx()
            .to(ToSelf)
            .typed(system_proxy::UserBuiltinProxy)
            .esdt_local_burn(&payment.token_identifier, payment.token_nonce, &payment.amount)
            .sync_call();

        self.credits_added_event(&caller, &team, &payment.token_identifier, &payment.amount);
    }

    #[event("creditsAdded")]
    fn credits_added_event(&self, #[indexed] caller: &ManagedAddress, #[indexed] team: &ManagedBuffer, #[indexed] token: &TokenIdentifier, #[indexed] amount: &BigUint);
}
