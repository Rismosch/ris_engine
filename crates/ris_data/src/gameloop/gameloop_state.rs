pub enum GameloopState {
    WantsToContinue,
    WantsToQuit,
    Error(String),
}