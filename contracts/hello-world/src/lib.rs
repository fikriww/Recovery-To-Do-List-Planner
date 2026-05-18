#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskLoad {
    Low,
    Medium,
    High,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub cognitive_load: TaskLoad,
    pub physical_load: TaskLoad,
    pub is_essential: bool,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn plan_daily(
        env: Env,
        current_rhr: u32,
        baseline_rhr: u32,
        tasks: Vec<Task>,
    ) -> (String, Vec<String>, Vec<String>, Vec<Task>) {
        let threshold = baseline_rhr + baseline_rhr / 10;
        let is_fatigued = current_rhr > threshold;
        let is_prime = current_rhr <= baseline_rhr;

        if is_fatigued {
            let mut filtered = Vec::new(&env);
            for task in tasks.iter() {
                let task = task.clone();
                if task.is_essential
                    || !(task.cognitive_load == TaskLoad::High
                        || task.physical_load == TaskLoad::High)
                {
                    filtered.push_back(task);
                }
            }

            let mut essential_high = Vec::new(&env);
            let mut low_load = Vec::new(&env);
            let mut remaining = Vec::new(&env);

            for task in filtered.iter() {
                let task = task.clone();
                if task.is_essential
                    && (task.cognitive_load == TaskLoad::High
                        || task.physical_load == TaskLoad::High)
                {
                    essential_high.push_back(task);
                } else if task.cognitive_load == TaskLoad::Low
                    && task.physical_load == TaskLoad::Low
                {
                    low_load.push_back(task);
                } else {
                    remaining.push_back(task);
                }
            }

            let mut plan = Vec::new(&env);
            for task in essential_high.iter() {
                plan.push_back(task.clone());
            }
            for task in low_load.iter() {
                plan.push_back(task.clone());
            }
            for task in remaining.iter() {
                plan.push_back(task.clone());
            }

            (
                String::from_str(&env, "Fatigued"),
                vec![
                    &env,
                    String::from_str(&env, "Prioritize recovery and avoid high-strain tasks."),
                    String::from_str(&env, "Focus on essential work, short breaks, and hydration."),
                ],
                vec![
                    &env,
                    String::from_str(&env, "Skip heavy workouts today."),
                    String::from_str(&env, "Limit caffeine and avoid late-night deep work."),
                ],
                plan,
            )
        } else if is_prime {
            let mut high = Vec::new(&env);
            let mut medium = Vec::new(&env);
            let mut low = Vec::new(&env);

            for task in tasks.iter() {
                let task = task.clone();
                match task.cognitive_load {
                    TaskLoad::High => high.push_back(task),
                    TaskLoad::Medium => medium.push_back(task),
                    TaskLoad::Low => low.push_back(task),
                }
            }

            let mut plan = Vec::new(&env);
            for task in high.iter() {
                plan.push_back(task.clone());
            }
            for task in medium.iter() {
                plan.push_back(task.clone());
            }
            for task in low.iter() {
                plan.push_back(task.clone());
            }

            (
                String::from_str(&env, "Prime"),
                vec![
                    &env,
                    String::from_str(&env, "Tackle the hardest problems early."),
                    String::from_str(&env, "Use your recovery window for focused deep work."),
                ],
                vec![&env, String::from_str(&env, "Don't procrastinate on your top priority tasks.")],
                plan,
            )
        } else {
            let mut essential_high = Vec::new(&env);
            let mut essential_medium = Vec::new(&env);
            let mut essential_low = Vec::new(&env);
            let mut nonessential_high = Vec::new(&env);
            let mut nonessential_medium = Vec::new(&env);
            let mut nonessential_low = Vec::new(&env);

            for task in tasks.iter() {
                let task = task.clone();
                let target = if task.is_essential {
                    match task.cognitive_load {
                        TaskLoad::High => &mut essential_high,
                        TaskLoad::Medium => &mut essential_medium,
                        TaskLoad::Low => &mut essential_low,
                    }
                } else {
                    match task.cognitive_load {
                        TaskLoad::High => &mut nonessential_high,
                        TaskLoad::Medium => &mut nonessential_medium,
                        TaskLoad::Low => &mut nonessential_low,
                    }
                };
                target.push_back(task);
            }

            let mut plan = Vec::new(&env);
            for task in essential_high.iter() {
                plan.push_back(task.clone());
            }
            for task in essential_medium.iter() {
                plan.push_back(task.clone());
            }
            for task in essential_low.iter() {
                plan.push_back(task.clone());
            }
            for task in nonessential_high.iter() {
                plan.push_back(task.clone());
            }
            for task in nonessential_medium.iter() {
                plan.push_back(task.clone());
            }
            for task in nonessential_low.iter() {
                plan.push_back(task.clone());
            }

            (
                String::from_str(&env, "Normal"),
                vec![
                    &env,
                    String::from_str(&env, "Keep a balanced workload and listen to your energy levels."),
                    String::from_str(&env, "Start with essential tasks and maintain steady pacing."),
                ],
                vec![
                    &env,
                    String::from_str(&env, "Avoid sudden spikes in physical or mental strain."),
                ],
                plan,
            )
        }
    }
}

mod test;
