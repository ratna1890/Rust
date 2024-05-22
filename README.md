# ws_install_code

Checks

```bash
dfx canister call backend_1 get_player
dfx canister call backend_1 greet '("backend_1")'
dfx canister call backend_2 get_player
dfx canister call backend_2 greet '("backend_2")'
dfx canister call manager get_principal_texts
dfx canister call manager get_principals
```

Upgrade

```bash
# update backend1(or 2) code
dfx build backend_1
# move to artifacts
dfx build manager
dfx canister install manager --argument "(record { backend_1 = \"$(dfx canister id backend_1)\"; backend_2 = \"$(dfx canister id backend_2)\" })" --mode upgrade
dfx canister call manager upgrade_backends
```
