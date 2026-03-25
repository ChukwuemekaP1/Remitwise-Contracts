#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Symbol, Vec};
use remitwise_common::CoverageType;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Policy {
    pub id: u32,
    pub owner: Address,
    pub name: String,
    pub coverage_type: CoverageType,
    pub monthly_premium: i128,
    pub coverage_amount: i128,
    pub active: bool,
    pub next_payment_date: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyPage {
    pub items: Vec<Policy>,
    pub count: u32,
}

#[contract]
pub struct Insurance;

#[contractimpl]
impl Insurance {
    pub fn create_policy(
        _env: Env,
        _owner: Address,
        _name: String,
        _coverage_type: CoverageType,
        _monthly_premium: i128,
        _coverage_amount: i128,
    ) -> u32 {
        1
    }

    pub fn get_policy(_env: Env, _id: u32) -> Option<Policy> {
        None
    }

    pub fn pay_premium(_env: Env, _caller: Address, _policy_id: u32) -> bool {
        true
    }

    pub fn deactivate_policy(_env: Env, _owner: Address, _policy_id: u32) -> bool {
        true
    }

    pub fn get_active_policies(env: Env, _owner: Address, _offset: u32, _limit: u32) -> PolicyPage {
        PolicyPage { items: Vec::new(&env), count: 0 }
    }

    pub fn get_all_policies_for_owner(env: Env, _owner: Address, _offset: u32, _limit: u32) -> PolicyPage {
        PolicyPage { items: Vec::new(&env), count: 0 }
    }

    pub fn get_total_monthly_premium(_env: Env, _owner: Address) -> i128 {
        0
    }

    pub fn create_premium_schedule(_env: Env, _owner: Address, _policy_id: u32, _next_due: u64, _interval: u64) -> u32 {
        1
    }

    pub fn get_premium_schedule(_env: Env, _schedule_id: u32) -> Option<PremiumSchedule> {
        None
    }

    pub fn modify_premium_schedule(_env: Env, _owner: Address, _schedule_id: u32, _next_due: u64, _interval: u64) {
    }

    pub fn cancel_premium_schedule(_env: Env, _owner: Address, _schedule_id: u32) {
    }

    pub fn execute_due_premium_schedules(env: Env) -> Vec<u32> {
        Vec::new(&env)
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PremiumSchedule {
    pub id: u32,
    pub policy_id: u32,
    pub next_due: u64,
    pub interval: u64,
    pub active: bool,
    pub missed_count: u32,
}