
../../rh0dy

// Dual Authorization (Dual Auth) is an internal security tool that
// allows small business banking customers to require certain types of
// activities and transactions to have a second user's credentials approve a submitted transaction.

// The system of storage and handling designed to prohibit individual access to certain resources by 
// requiring the presence and actions of at least two authorized persons, each capable of detecting 
// incorrect or unauthorized security procedures with respect to the task being performed.

scrypto build
scrypto test

resim publish .
export package=<p>

resim new-account
export pubkey=
export acct=

resim show $acct

export xrd=<resourcr_def>

resim call-function $package <Blueprint Hello> <Function make_hello>
export ht=<resource_def>
export hello=<component>

resim show $ht
resim show $hello

resim call-method $<component> free_token 1
resim show $<compoentn>
resim show $acct

| To create an account | ``` resim new-account ``` |
| To change the default account | ``` resim set-default-account <account_address> ``` |
| To create a token with fixed supply | ``` resim new-token-fixed <amount> ``` |
| To create a token with mutable supply | ``` resim new-token-mutable <minter_badge_address> ``` |
| To create a badge with fixed supply | ``` resim new-badge-fixed <amount> ``` |
| To create a badge with mutable supply | ``` resim new-badge-mutable <minter_badge_address> ``` |
| To mint resource | ``` resim mint <amount> <resource_def>``` |
| To transfer resource | ``` resim transfer <amount> <resource_def> <recipient_address> ``` |
| To publish a package | ``` resim publish <path_to_package_dir_or_wasm_file> ``` |
| To call a function | ``` resim call-function <package_address> <blueprint_name> <function> <args> ``` |
| To call a method | ``` resim call-method <component_address> <method> <args> ``` |
| To export the ABI of a blueprint | ``` resim export-abi <package_address> <blueprint_name> ``` |
| To show info about an address | ``` resim show <address> ``` |

**Note:** The commands use the default account as transaction sender.



#[derive(TypeId, Encode, Decode)]
struct Transaction {
    id: Decimal,
    amount: Decimal,
    from: Vault,
    to: Vault
}
