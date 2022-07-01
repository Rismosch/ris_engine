use std::marker::PhantomData;

use crate::icontext::IContext;

pub struct ContextTest<TContext: IContext> {
    phantom_data: PhantomData<TContext>,
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe> Default
    for ContextTest<TContext>
{
    fn default() -> Self {
        ContextTest {
            phantom_data: PhantomData::default(),
        }
    }
}

impl<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe>
    ContextTest<TContext>
{
    pub fn run<TFn: FnMut(&mut TContext) + std::panic::UnwindSafe>(&self, test: TFn) {
        execute_context_test::<TContext, TFn>(test)
    }
}

pub fn execute_context_test<
    TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe,
    TFnMut: FnMut(&mut TContext) + std::panic::UnwindSafe,
>(
    mut test: TFnMut,
) {
    let result;

    let mut context = TContext::setup();
    let raw_context = &mut context as *mut TContext;

    unsafe {
        result = std::panic::catch_unwind(move || {
            test(raw_context.as_mut().unwrap());
        });

        raw_context.as_mut().unwrap().teardown();
    }

    assert!(result.is_ok());
}
