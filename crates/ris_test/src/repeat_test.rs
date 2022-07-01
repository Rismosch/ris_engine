use crate::{repeat_single_thread_test::RepeatSingleThreadTest, repeat_context_test::RepeatContextTest, icontext::IContext};

pub struct RepeatTest {
    data: RepeatData
}

#[derive(Clone)]
pub struct RepeatData {
    pub count: u32,
    pub kind: RepeatKind,
}

#[derive(Clone)]
pub enum RepeatKind{
    Repeat,
    Retry
}

struct Bruh {

}

impl IContext for Bruh {
    fn setup() -> Self {
        Bruh {  }
    }

    fn teardown(&mut self) {
        
    }
}

impl RepeatTest {
    pub fn new(count: u32, kind: RepeatKind) -> Self {
        let data = RepeatData {count, kind};
        RepeatTest { data }
    }

    pub fn single_thread(&self) -> RepeatSingleThreadTest {
        RepeatSingleThreadTest::new(self.data.clone())
    }

    pub fn context<TContext: IContext + std::panic::RefUnwindSafe + std::panic::UnwindSafe>(&self) -> RepeatContextTest<TContext> {
        RepeatContextTest::new(self.data.clone())
    }

    pub fn run(&self, test: fn()){
        execute_repeat_test(self.data.count, self.data.kind.clone(), test)
    }
}

// pub fn execute_repeat_experimentation<TFnMut: FnMut() + std::panic::UnwindSafe>(count: u32, kind: RepeatKind, test: TFnMut){}

// pub fn execute_repeat_test(count: u32, kind: RepeatKind, test: fn())
// {
//     match kind {
//         RepeatKind::Repeat => {
//             for _ in 0..count {
//                 test();
//             }
//         },
//         RepeatKind::Retry => {
//             for _ in 0..count - 1 {
                
//                 let result = std::panic::catch_unwind(test);

//                 if result.is_ok() {
//                     return;
//                 }
//             }

//             test();
//         },
//     }
// }

pub fn execute_repeat_test<TFnMut: FnMut() + Clone + std::panic::UnwindSafe>(count: u32, kind: RepeatKind, test: TFnMut)
{
    match kind {
        RepeatKind::Repeat => {
            for _ in 0..count {
                test.clone()();
            }
        },
        RepeatKind::Retry => {
            for _ in 0..count - 1 {
                
                let result = std::panic::catch_unwind(test.clone());

                if result.is_ok() {
                    return;
                }
            }

            test.clone()();
        },
    }
}