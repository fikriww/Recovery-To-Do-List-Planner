#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test_plan_daily_prime() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let tasks = vec![
        &env,
        Task {
            id: String::from_str(&env, "task-1"),
            title: String::from_str(&env, "Write report"),
            cognitive_load: TaskLoad::High,
            physical_load: TaskLoad::Low,
            is_essential: true,
        },
        Task {
            id: String::from_str(&env, "task-2"),
            title: String::from_str(&env, "Email follow-ups"),
            cognitive_load: TaskLoad::Low,
            physical_load: TaskLoad::Low,
            is_essential: false,
        },
    ];

    let (status, do_recommendations, dont_recommendations, optimized_tasks) =
        client.plan_daily(&1_u32, &1_u32, &tasks);

    assert_eq!(status, String::from_str(&env, "Prime"));
    assert_eq!(do_recommendations.len(), 2);
    assert_eq!(dont_recommendations.len(), 1);
    assert_eq!(optimized_tasks.len(), 2);
    assert_eq!(optimized_tasks.get(0).unwrap().title, String::from_str(&env, "Write report"));
}
