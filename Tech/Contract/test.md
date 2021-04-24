# Tests

Testing is an important aspect of every blockchain project, so we have covered our code with fully (100%) code coverage. However, by developing the fornt-end of our project in next phasese the testing precedure can be easier.

First of all you need to clone the repository, run:

```bash
git clone https://github.com/oxydev/SubsCrypt-ink
cd SubsCrypt-ink
```

Then, you can run the tests with this line of code:

```bash
cargo +nightly test
```

So you will see this output:

```bash
running 26 tests
test tests::default_works ... ok
test tests::change_disable_works ... ok
test tests::add_plan_fails_wrong_arguments ... ok
test tests::add_plan_works ... ok
test tests::constructor_works ... ok
test tests::check_subscription_works ... ok
test tests::check_auth_works ... ok
test tests::linked_list_works ... ok
test tests::edit_plan_works ... ok
test tests::provider_register_fails_insufficient_payment ... ok
test tests::edit_plan_fails_invalid_plan_index ... ok
test tests::provider_register_works ... ok
test tests::linked_list_default_works ... ok
test tests::add_entry_works ... ok
test tests::provider_register_fails_wrong_arguments ... ok
test tests::refund_fails_double_refund ... ok
test tests::retrieve_whole_data_with_wallet_works ... ok
test tests::retrieve_data_with_wallet_works ... ok
test tests::retrieve_whole_data_with_password_works ... ok
test tests::retrieve_data_with_password_works ... ok
test tests::subscribe_works ... ok
test tests::set_subscrypt_pass_works ... ok
test tests::subscribe_fails_insufficient_paying ... ok
test tests::withdraw_fails_provider_must_be_registered ... ok
test tests::withdraw_works ... ok
test tests::refund_works ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```


## Building

To build the WASM of your contract and metadata, you can clone and change directory to the ink project of SubsCrypt and then you have to run this line:

```bash
cargo +nightly contract build
```

This command will take some minutes and the output will be something like this:

```bash
Original wasm size: 99.1K, Optimized: 68.0K

Your contract artifacts are ready. You can find them in:
/yourDirectory/target/ink

  - SubsCrypt.contract (code + metadata)
  - SubsCrypt.wasm (the contract's code)
  - metadata.json (the contract's metadata)
```
 
You can also use the pre-built version of our code and access to the WASM and metadata files, [here](https://github.com/oxydev/SubsCrypt-ink/blob/main/deploy/SubsCrypt.wasm) and [here](https://github.com/oxydev/SubsCrypt-ink/blob/main/deploy/metadata.json).
