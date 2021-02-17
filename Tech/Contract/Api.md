
### Contract API specification

| Function | Description | Params | Returns | State mutability | 
| ------------- | ------------- | ------------- | ------------- | ------------- |
| provider_register | This function is for providers to register in the contract | list of durations, list of prices, list of max refund percent, money destination address | None | change state |
| add_plan | This function is for providers to add their plans; each plan has duration, price, max refund percent that they are willing to lock in contract and withdraw after that the subscription period has finished. | list of durations, list of prices, list of max refund percent | None | change state |
| edit_plan | This function is for providers to edit their plan. (Old subscriptions are not affected by this change) | index of their plan, new duration, new max refund percent, new price | None | change state |
| change_disable | This function is for providers to edit their plan that changes the active or deactivate status of their plan(so people can or can't subscribe in that plan) | plan index| None | change state |
| subscribe | This payable function is for users to subscribe to their desired service and plan; they have to provide a hash of their password (the auth mechanism will be explained thoroughly in Auth Section) and provider address and plan index and some metadata that is encrypted by the public key of the provider(users can trust providers to share their data with but nobody else can know that data) | provider address, plan index, the hash of pass, An optional encrypted metadata| None | change state(payable) |
| refund | This function is for users to refund their subscribe anytime they want and instantly withdraw the rest of their money(maximum amount of refund is indicated by max refund percent that provider had set for that plan) | provider address, plan index| None |  change state |
| withdraw | This function is for providers to withdraw the amount that is now ready to withdraw(this is the money that we lock in the contract when a user subscribes to a plan according to max refund percent, and when their plan is finished, that money can be withdrawn). We used an optimized LinkedList solution, which is really cheap to execute and fast. | None | amount of money you are paid  | change state |
| check_subscription | This function is for users or anyone to check that if a user has an active subscription in a specific plan of a provider | address of the user, address of provider, plan index| return boolean | view |
| check_auth | This function is used to check if the given combination of token and passphrase can authenticate a specific user for a provider(the auth mechanism will be explained thoroughly in Auth Section) | address of the user, address of provider, token, passphrase| return boolean | view |
| retrieve_whole_data_with_password | This function is used to get every subscription record of a user with their token and passphrase, which first have to be set in setSubsCryptPass function(this token and passphrase is only worked to login in SubsCrypt website to have a whole review of your account)  | address of the user, token, passphrase| return whole records of a user | view |  
| retrieve_whole_data_with_wallet | This function is the same as the above function with a slight difference that it is used with user wallet to trigger the contract directly | None | return whole records of a user | view |
| retrieve_data_with_password | This function is used to get every subscription record of a user only related to a specific provider with their token and passphrase is set once they subscribe to their chosen plan of that provider | address of the user, address of provider, token, passphrase| return whole records of a user | view |
| retrieve_data_with_wallet | This function is the same as the above function with a slight difference that it is used with user wallet to directly trigger the contract | address of provider| return whole records of a user-related to that provider | view |
<!-- tabs:start -->
#### ** Solution **

[embedded-code-final](./lib.rs ':include :type=code embed-final')
<!-- tabs:end -->