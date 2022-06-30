pub struct Test {}

pub struct RepeatTest {}

pub struct SingleThreadTest {}

pub struct ContextTest<TContext: IContext> {
    context: TContext
}

pub struct RepeatSingleThreadTest {}

pub struct RepeatContextTest {}

pub struct SingleThreadContextTest {}

pub struct RepeatSingleThreadContextTest {}

pub fn test() -> Test {
    Test {  }
}

pub trait IContext {
    fn setup() -> Self;
    fn teardown(&mut self);
}

impl Test {
    pub fn repeat(&self, repeats: u32) -> RepeatTest {
        panic!()
    }

    pub fn retry(&self, retries: u32) -> RepeatTest {
        panic!()
    }

    pub fn single_thread(&self) -> SingleThreadTest {
        panic!()
    }

    pub fn context<TContext: IContext>(&self) -> ContextTest<TContext> {
        let context = TContext::setup();
        ContextTest { context }
    }

    pub fn run(test_fn: fn()) {
        test_fn();
    }
}

impl RepeatTest {
    pub fn single_thread() -> RepeatSingleThreadTest {
        panic!()
    }

    pub fn context() -> RepeatContextTest {
        panic!()
    }

    pub fn run(test_fn: fn()){
        panic!()
    }
}

impl SingleThreadTest {
    pub fn context() -> SingleThreadContextTest {
        panic!()
    }

    pub fn run(test_fn: fn()){
        panic!()
    }
}

impl<TContext: IContext> ContextTest<TContext> {
    pub fn run(test_fn: fn()) {
        panic!()
    }
}

impl RepeatSingleThreadTest {
    pub fn context() -> RepeatSingleThreadContextTest {
        panic!()
    }

    pub fn run(test_fn: fn()){
        panic!()
    }
}

impl RepeatContextTest {
    pub fn run(test_fn: fn()) {
        panic!()
    }
}

impl SingleThreadContextTest {
    pub fn run(test_fn: fn()) {
        panic!()
    }
}

impl RepeatSingleThreadContextTest {
    pub fn run(test_fn: fn()) {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyContext {
        number: i32,
    }

    impl IContext for MyContext {
        fn setup() -> Self {
            MyContext { number: 42 }
        }

        fn teardown(&mut self) {}
    }

    #[test]
    fn bruh() {
        test()
        .context::<MyContext>();

        panic!("bruh");
    }
}