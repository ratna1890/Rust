use candid::{Principal, CandidType, Deserialize};
use ic_cdk::api::{management_canister::main::{install_code, CanisterInstallMode, InstallCodeArgument}, call::CallResult};
use ic_cdk_macros::{init, pre_upgrade, post_upgrade};

// const BACKEND1_WASM: &[u8] = include_bytes!("../../../artifacts/Backend1.wasm");
const BACKEND1_WASM: &[u8] = include_bytes!("../../../artifacts/Backend1_gzip.wasm.gz"); // use gzip
// const BACKEND2_WASM: &[u8] = include_bytes!("../../../artifacts/Backend2.wasm");
const BACKEND2_WASM: &[u8] = include_bytes!("../../../artifacts/Backend2_gzip.wasm.gz"); // use gzip

thread_local! {
    static BACKEND1_ADDRESS: std::cell::RefCell<String> = std::cell::RefCell::new("".to_string());
    static BACKEND2_ADDRESS: std::cell::RefCell<String> = std::cell::RefCell::new("".to_string());
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_principal_texts() -> (String, String) {
    (
        BACKEND1_ADDRESS.with(|n| n.borrow().clone()),
        BACKEND2_ADDRESS.with(|a| a.borrow().clone())
    )
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_principals() -> (Principal, Principal) {
    (
        Principal::from_text(BACKEND1_ADDRESS.with(|n| n.borrow().clone())).unwrap(),
        Principal::from_text(BACKEND2_ADDRESS.with(|a| a.borrow().clone())).unwrap()
    )
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn set_principal_texts(addr1: String, addr2: String) {
    BACKEND1_ADDRESS.with(|n| *n.borrow_mut() = addr1);
    BACKEND2_ADDRESS.with(|a| *a.borrow_mut() = addr2);
}

#[derive(CandidType, Deserialize)]
struct _UpgradeResponse {
    principal: String,
    before_module_hash: String,
    after_module_hash: String
}
#[ic_cdk::update]
#[candid::candid_method(update)]
async fn upgrade_backends() {
    let principal = Principal::from_text(BACKEND1_ADDRESS.with(|n| n.borrow().clone())).unwrap();
    install(
        principal,
        BACKEND1_WASM.to_vec(),
        Vec::new()
    ).await.unwrap();

    let principal = Principal::from_text(BACKEND2_ADDRESS.with(|n| n.borrow().clone())).unwrap();
    install(
        principal,
        BACKEND2_WASM.to_vec(),
        Vec::new()
    ).await.unwrap();
}

async fn install(canister_id: Principal, wasm_module: Vec<u8>, arg: Vec<u8>) -> CallResult<()> {
    install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade,
        canister_id,
        wasm_module,
        arg,
    })
    .await
}

#[derive(CandidType, Deserialize)]
struct InitPayload {
    backend_1: String,
    backend_2: String,
}
#[init]
fn init(payload: InitPayload) {
    BACKEND1_ADDRESS.with(|n| *n.borrow_mut() = payload.backend_1);
    BACKEND2_ADDRESS.with(|a| *a.borrow_mut() = payload.backend_2);
}

#[derive(CandidType, Deserialize)]
struct StableStateForUpgrade {
    backend_1: Principal,
    backend_2: Principal,
}
#[pre_upgrade]
fn pre_upgrade() {
    let addrs = get_principals();
    let state = StableStateForUpgrade {
        backend_1: addrs.0,
        backend_2: addrs.1,
    };
    ic_cdk::storage::stable_save((state,)).unwrap();
}
#[post_upgrade]
fn post_upgrade() {
    let (state,) = ic_cdk::storage::stable_restore().unwrap();
    let state: StableStateForUpgrade = state;
    BACKEND1_ADDRESS.with(|n| *n.borrow_mut() = state.backend_1.to_text());
    BACKEND2_ADDRESS.with(|n| *n.borrow_mut() = state.backend_2.to_text());
}

#[cfg(test)]
mod tests {
    use super::*;
    candid::export_service!();

    #[test]
    fn gen_candid() {
        std::fs::write("interface.did", __export_service()).unwrap();
    }
}