use crate::testbed_runners::end_count;
use crate::testbed_runners::start_count;
use crate::testbed_runners::GasMetric;
use near_runtime_fees::RuntimeFeesConfig;
use near_vm_logic::mocks::mock_external::MockedExternal;
use near_vm_logic::{VMConfig, VMContext, VMOutcome};
use near_vm_runner::VMError;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const CURRENT_ACCOUNT_ID: &str = "alice";
const SIGNER_ACCOUNT_ID: &str = "bob";
const SIGNER_ACCOUNT_PK: [u8; 3] = [0, 1, 2];
const PREDECESSOR_ACCOUNT_ID: &str = "carol";

fn create_context(input: Vec<u8>) -> VMContext {
    VMContext {
        current_account_id: CURRENT_ACCOUNT_ID.to_owned(),
        signer_account_id: SIGNER_ACCOUNT_ID.to_owned(),
        signer_account_pk: Vec::from(&SIGNER_ACCOUNT_PK[..]),
        predecessor_account_id: PREDECESSOR_ACCOUNT_ID.to_owned(),
        input,
        block_index: 10,
        block_timestamp: 42,
        epoch_height: 0,
        account_balance: 2u128,
        account_locked_balance: 1u128,
        storage_usage: 12,
        attached_deposit: 2u128,
        prepaid_gas: 10_u64.pow(18),
        random_seed: vec![0, 1, 2],
        is_view: false,
        output_data_receivers: vec![],
    }
}

fn call() -> (Option<VMOutcome>, Option<VMError>) {
    let code = include_bytes!("../test-contract/res/large_contract.wasm");
    let mut fake_external = MockedExternal::new();
    let context = create_context(vec![]);
    let config = VMConfig::default();
    let fees = RuntimeFeesConfig::default();

    let promise_results = vec![];

    let mut hash = DefaultHasher::new();
    code.hash(&mut hash);
    let code_hash = hash.finish().to_le_bytes().to_vec();
    near_vm_runner::run(
        code_hash,
        code,
        b"cpu_ram_soak_test",
        &mut fake_external,
        context,
        &config,
        &fees,
        &promise_results,
    )
}

const NUM_ITERATIONS: u64 = 10;

/// Cost of the most CPU demanding operation.
pub fn cost_per_op(gas_metric: &GasMetric) -> u64 {
    // Call once for the warmup.
    let (outcome, _) = call();
    let outcome = outcome.unwrap();
    let start = start_count(&gas_metric);
    for _ in 0..NUM_ITERATIONS {
        call();
    }
    let cost = end_count(&gas_metric, &start);
    cost / (outcome.burnt_gas * NUM_ITERATIONS)
}
