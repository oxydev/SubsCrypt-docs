
### Contract API specification
<!-- tabs:start -->

#### ** Request **

**Provider Registration**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| provider_register | change state | This function is for providers to register in the contract | list of durations, list of prices, list of max refund permille, money destination address, username, hash of password of provider, plans characteristics array|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Add new Plan**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| add_plan | change state |This function is for providers to add their plans; each plan has duration, price, max refund percent that they are willing to lock in contract and withdraw after that the subscription period has finished. | list of durations, list of prices, list of max refund permille, plan_characteristics |

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Edit prev plans**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| edit_plan | change state |This function is for providers to edit their plan. (Old subscriptions are not affected by this change) | index of their plan, new duration, new max refund permille, new price , disable it or not|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Adding new characteristics to existing plans**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| add_characteristic_for_plan | change state |This function is for providers to adding new characteristics to existing plans| index of their plan, characteristics|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Change status of Plan**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| change_disable | change state  | This function is for providers to edit their plan that changes the active or deactivate status of their plan(so people can or can't subscribe in that plan) | plan index|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Subscribe**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| subscribe | change state  | This payable function is for users to subscribe to their desired service and plan; they have to provide a hash of their password (the auth mechanism will be explained thoroughly in Auth Section) and provider address and plan index and some characteristics values that is encrypted by the public key of the provider(users can trust providers to share their data with but nobody else can know that data) | provider address, plan index, the hash of pass, username, An optional characteristics values|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Renew subscription**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| renew | change state  | This payable function is for users to renew their subscription; they have to already have a valid subscription to that specifc plan and some characteristics values that is encrypted by the public key of the provider(users can trust providers to share their data with but nobody else can know that data). Also The start of that renewed subscription will be at the end of current subscription. The current subscription can not be refunded.| provider address, plan index, An optional characteristics values|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Set Pass of user in SubsCrypt Dashboard**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| set_user_subscrypt_pass | change state | This function is for users to change their password anytime they want and instantly| hash of password | 

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Set Pass of user in Each provider**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| subs_crypt_pass_hash_for_each_provider | change state | This function is for users to change their password in each provider anytime they want and instantly| provider address, hash of new password | 

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Set Pass of Provider in SubsCrypt Dashboard**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| set_provider_subscrypt_pass | change state | This function is for providers to change their password anytime they want and instantly| hash of password | 

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Refund subscription**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| refund | change state | This function is for users to refund their subscribe anytime they want and instantly withdraw the rest of their money(maximum amount of refund is indicated by max refund percent that provider had set for that plan) | provider address, plan index| 

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
None
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Withdraw Money**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| withdraw | change state | This function is for providers to withdraw the amount that is now ready to withdraw(this is the money that we lock in the contract when a user subscribes to a plan according to max refund percent, and when their plan is finished, that money can be withdrawn). We used an optimized LinkedList solution, which is really cheap to execute and fast. | None |

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
amount of money you are paid
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Check subscription status of User**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| check_subscription | view |This function is for users or anyone to check that if a user has an active subscription in a specific plan of a provider | address of the user, address of provider, plan index|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return boolean
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Check Authentication of a user**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| check_auth | view |This function is used to check if the given passphrase can authenticate a specific user for a provider(the auth mechanism will be explained thoroughly in Auth Section) | address of the user, address of provider, passPhrase|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return boolean
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Check Authentication of a user with their username**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| check_auth_with_username | view |This function is used to check if the given passphrase can authenticate a specific user for a provider(the auth mechanism will be explained thoroughly in Auth Section) | username of the user, address of provider, passPhrase|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return boolean
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Check Authentication of a provider in SubsCrypt Dashboard**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| provider_check_auth | view |This function is used to check if the given passphrase can authenticate a specific provider in SubsCrypt Dashboard(the auth mechanism will be explained thoroughly in Auth Section) | address of provider, passPhrase|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return boolean
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Check Authentication of a provider in SubsCrypt Dashboard with their username**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| provider_check_auth_with_username | view |This function indicate if provider can authenticate in SubsCrypt Dashboard with given pass_phrase and username(the auth mechanism will be explained thoroughly in Auth Section) | username of the provider, passPhrase|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return boolean
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Check Authentication of a user in SubsCrypt Dashboard**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| user_check_auth | view |This function is used to check if the given passphrase can authenticate a specific user in SubsCrypt Dashboard(the auth mechanism will be explained thoroughly in Auth Section) | address of user, passPhrase|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return boolean
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Check Authentication of a user in SubsCrypt Dashboard with their username**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| user_check_auth_with_username | view |This function indicate if user can authenticate in SubsCrypt Dashboard with given pass_phrase and username(the auth mechanism will be explained thoroughly in Auth Section) | username of the user, passPhrase|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return boolean
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Check if given username is available**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| is_username_available | view |This function indicate if username is available(it will be used for users who want to subscribe for the first time in platform) | username |

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return boolean
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Get Username of given Address**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| get_username | view |This function indicate username of caller|  |

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return String
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Get whole data of a user by its username and password of SubsCrypt**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| retrieve_whole_data_with_username | view | This function is used to get every subscription record of a user with their username and passphrase | Username of user, passphrase|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return Vec<SubscriptionRecord>
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Get whole data of a user by its wallet of SubsCrypt**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| retrieve_whole_data_with_wallet | view |This function is the same as the above function with a slight difference that it is used with user wallet to trigger the contract directly | None |

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return whole records of a user
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Get data of a user by its username and password**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| retrieve_data_with_username | view | This function is used to get every subscription record of a user only related to a specific provider with their passphrase is set once they subscribe to their chosen plan of that provider | Username of user, address of provider, passphrase|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return whole records of a user related to a specefic provider
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Get data of a user by its wallet**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| retrieve_data_with_wallet | view | This function is the same as the above function with a slight difference that it is used with user wallet to directly trigger the contract | address of provider|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return whole records of a user related to a specefic provider
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Get data of a user by its wallet**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| get_plan_data | view | This function can get plan data of provider | address of provider, planIndex|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return PlanConsts
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Get plan characteristic keys**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| get_plan_characteristics | view | This function can get plan characteristic keys in this function | address of provider, planIndex|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return Vec<String>
```
<!-- tabs:end -->
<!-- tabs:start -->

#### ** Request **

**Check subscription status of User using username**

| Name | State mutability | Description | Params |
| --- | --- | --- | --- |
| check_subscription_with_username | view |This function is for users or anyone to check that if a user has an active subscription in a specific plan of a provider using username| Username of the user, address of provider, plan index|

#### ** Response **

**200: OK**

Stats successfully retrieved.
```
return boolean
```
<!-- tabs:end -->