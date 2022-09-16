#[derive(Clone)]
pub enum GameloopState {
    WantsToContinue,
    WantsToQuit,
    WantsToRestart,
    Error(String),
}
