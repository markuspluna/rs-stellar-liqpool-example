#![no_std]
use soroban_sdk::{contractimpl, log, symbol, Env, Symbol};

const COUNTER: Symbol = symbol!("COUNTER");

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env
            .data()
            .get(COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not u32.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Save the count.
        env.data().set(COUNTER, count);

        // Return the count to the caller.
        count
    }
}

mod test;
