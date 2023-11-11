#[derive(Clone, PartialEq, Eq)]
pub enum GameloopState {
    WantsToContinue,
    WantsToQuit,
    WantsToRestart,
}
