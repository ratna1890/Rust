use candid::Principal;
use ic_cdk::api::{call::CallResult, management_canister::{main::{UpdateSettingsArgument, update_settings}, provisional::CanisterSettings}};
use ic_cdk_macros::init;

thread_local! {
    static NAME: std::cell::RefCell<String> = std::cell::RefCell::new("".to_string());
    static AGE: std::cell::RefCell<u32> = std::cell::RefCell::new(0);
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}! I'm upgraded!", name)
}
#[ic_cdk::query]
#[candid::candid_method(query)]
fn greet_with_msg(name: String, msg: String) -> String {
    format!("{} {}", greet(name), msg)
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn get_player() -> (String, u32) {
    (
        NAME.with(|n| n.borrow().clone()),
        AGE.with(|a| a.borrow().clone())
    )
}
#[ic_cdk::update]
#[candid::candid_method]
fn set_player(name: String, age: u32) {
    NAME.with(|n| *n.borrow_mut() = name);
    AGE.with(|a| *a.borrow_mut() = age);
}

#[ic_cdk::update]
#[candid::candid_method]
async fn add_controller(controller: Principal) -> CallResult<()> {
    let this_canister = ic_cdk::api::id();
    update_settings(UpdateSettingsArgument {
        canister_id: this_canister.clone(),
        settings: CanisterSettings {
            controllers: Some(vec![this_canister, controller]), // NOTE: overwrite only
            compute_allocation: None,
            freezing_threshold: None,
            memory_allocation: None,
        },
    })
    .await
}

#[init]
fn init() {
    NAME.with(|n| *n.borrow_mut() = "Robot".to_string());
    AGE.with(|a| *a.borrow_mut() = 999);
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