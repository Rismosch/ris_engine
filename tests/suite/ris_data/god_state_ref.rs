use ris_data::god_state::GodStateCommand;
use ris_data::god_state::GodStateRef;
use ris_data::god_state::InnerGodState;
use ris_data::settings::Settings;

#[test]
fn is_safe() {
    let settings = Settings::default();
    let mut inner = InnerGodState::new(settings);

    {
        let ptr = inner.get() as *const InnerGodState;
        let state_ref = unsafe { GodStateRef::from(ptr) };

        let _data = state_ref.data.debug;
        let _events = state_ref.events.save_settings_requested;
        let queue = &state_ref.command_queue;
        queue.push(GodStateCommand::SetJobWorkersSetting(Some(20)));
    }

    let inner = inner.get_mut();
    inner.command_queue.start_iter();
    let first = inner.command_queue.next().unwrap();

    assert!(first == GodStateCommand::SetJobWorkersSetting(Some(20)));
}

#[test]
fn is_thread_safe() {
    let thread_count = 100;

    let settings = Settings::default();
    let mut inner = InnerGodState::new(settings);

    {
        let ptr = inner.get() as *const InnerGodState;
        let state_ref = unsafe { GodStateRef::from(ptr) };

        let mut handles = Vec::new();
        for i in 0..thread_count {
            let state_ref_clone = state_ref.clone();
            let handle = std::thread::spawn(move || {
                let my_ref = state_ref_clone;

                let _data = my_ref.data.debug;
                let _events = my_ref.events.save_settings_requested;
                my_ref
                    .command_queue
                    .push(GodStateCommand::SetJobWorkersSetting(Some(i)))
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    let mut results = Vec::with_capacity(thread_count);
    let queue = &inner.get_mut().command_queue;
    queue.start_iter();
    while let Some(result) = queue.next() {
        results.push(result);
    }

    for i in 0..thread_count {
        let mut result_found = false;

        for result in results.iter() {
            if *result == GodStateCommand::SetJobWorkersSetting(Some(i)) {
                result_found = true;
                break;
            }
        }

        assert!(result_found, "{} was not found", i);
    }
}
