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
running 17 tests
test subscrypt::tests::add_plan_works ... ok
test subscrypt::tests::check_subscription_works ... ok
test subscrypt::tests::add_entry_works ... ok
test subscrypt::tests::change_disable_works ... ok
test subscrypt::tests::constructor_works ... ok
test subscrypt::tests::default_works ... ok
test subscrypt::tests::edit_plan_works ... ok
test subscrypt::tests::linked_List_works ... ok
test subscrypt::tests::check_auth_works ... ok
test subscrypt::tests::provider_register_works ... ok
test subscrypt::tests::refund_works ... ok
test subscrypt::tests::retrieve_data_with_password_works ... ok
test subscrypt::tests::retrieve_data_with_wallet_works ... ok
test subscrypt::tests::retrieve_whole_data_with_password_works ... ok
test subscrypt::tests::withdraw_works ... ok
test subscrypt::tests::subscribe_works ... ok
test subscrypt::tests::retrieve_whole_data_with_wallet_works ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

```

# Building

To build the wasm of your contract you can clone and change directory to the ink project of SubsCrypt and then you have to run this line:

```bash
cargo +nightly contract build
```

This command will take some minutes and the output will be sth like this:

```bash
Your contract is ready. You can find it here:
/root/test/SubsCrypt-ink/target/SubsCrypt.wasm
```


To build the metadata json of contract you can clone and change directory to the ink project of SubsCrypt and then you have to run this line:

```bash
cargo +nightly contract generate-metadata
```

You can also use the pre-built version of our code and access to the wasm and metadata files, [here](https://github.com/oxydev/SubsCrypt-ink/raw/main/SubsCrypt.wasm) and [here](https://raw.githubusercontent.com/oxydev/SubsCrypt-ink/main/metadata.json).
