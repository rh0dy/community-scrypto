use scrypto::prelude::*;

blueprint! {
    struct DualAuthorization {
        amount: Decimal,
        from: Address,
        to: Address,       
        owner_badge: ResourceDef, 
        authorizer_badges: Vault
        //,authorizers: [ResourceDef; 2]
    }

    impl DualAuthorization {
        pub fn new(amount: Decimal, from: Address, to: Address) -> (Component, Bucket) {

            //scrypto_assert!(amount.is_negative(), "You must pass in a positive amount");

            // pass in authorizers?
            // get rid of owner badge?

            info!("new: amount - {}", amount);
            info!("new: from - {}", from);
            info!("new: to - {}", to);
        
            let owner_badge = ResourceBuilder::new()
                .metadata("name", "owner_badge")
                .new_badge_fixed(1);

            let authorizer_badge = ResourceBuilder::new()
                .metadata("name", "authorizer_badge")
                .new_badge_fixed(2); // TODO mutable to burn?

            info!("new: {}", owner_badge.resource_def().supply());
            info!("new: {}", owner_badge.resource_address());

            let component = Self {
                amount: amount,
                from: from,
                to: to,
                owner_badge: owner_badge.resource_def(),
                authorizer_badges: Vault::with_bucket(authorizer_badge)
                //, authorizers: authorizers
            }
            .instantiate();

            info!("new: {}", component.address());

            (component, owner_badge)
        }

        //pub fn get_authorizers(&self) {
        //    info!("set_authorizers authorization: ");
        //}

        //#[auth(owner_badge)]
        //pub fn set_authorizers(&self) {
        //    info!("set_authorizers authorization: ");
        //}

        //#[auth(authorizer_badge)]
        //pub fn authorize(&mut self) {
        //    info!("authorize: ");
        //}

        //pub fn status(&self) {
        //    info!("status: ");
        //}

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        //pub fn free_token(&mut self) -> Bucket {
        //    info!("My balance is: {} HelloToken. Now giving away a token!", self.sample_vault.amount());
        //    // If the semi-colon is omitted on the last line, the last value seen is automatically returned
        //    // In this case, a bucket containing 1 HelloToken is returned
        //    self.sample_vault.take(1)
        //}
    }
}
