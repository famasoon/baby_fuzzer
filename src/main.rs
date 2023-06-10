extern crate libafl;
use std::path::PathBuf;

use libafl::{
    bolts::AsSlice,
    executors::ExitKind,
    inputs::{BytesInput, HasTargetBytes},
    prelude::{
        current_nanos, tuple_list, InMemoryCorpus, InProcessExecutor, OnDiskCorpus,
        RandPrintablesGenerator, SimpleEventManager, SimpleMonitor, StdRand,
    },
    schedulers::QueueScheduler,
    state::StdState,
    StdFuzzer,
};

fn main() {
    let mut harness = |input: &BytesInput| {
        let target = input.target_bytes();
        let buf = target.as_slice();
        if buf.len() > 0 && buf[0] == 'a' as u8 {
            if buf.len() > 1 && buf[1] == 'b' as u8 {
                if buf.len() > 2 && buf[2] == 'c' as u8 {
                    panic!("=)");
                }
            }
        }
        ExitKind::Ok
    };

    let mut state = StdState::new(
        StdRand::with_seed(current_nanos()),
        InMemoryCorpus::<BytesInput>::new(),
        OnDiskCorpus::new(PathBuf::from("./crashes")).unwrap(),
        &mut (),
        &mut (),
    )
    .unwrap();

    let mon = SimpleMonitor::new(|s| println!("{s}"));
    let mut mgr = SimpleEventManager::new(mon);

    let scheduler = QueueScheduler::new();
    let mut fuzzer = StdFuzzer::new(scheduler, (), ());

    let mut executor = InProcessExecutor::new(&mut harness, (), &mut fuzzer, &mut state, &mut mgr)
        .expect("Failed to create the executor");

    let mut generator = RandPrintablesGenerator::new(32);
    state
        .generate_initial_inputs(&mut fuzzer, &mut executor, &mut generator, &mut mgr, 8)
        .expect("failed to generate the initial corpus");

    // To test the panic:
    let input = BytesInput::new(Vec::from("abc"));
    #[cfg(feature = "panic")]
    harness(&input);
}
