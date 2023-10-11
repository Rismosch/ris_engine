use ris_util::ris_error::RisError;

#[derive(Clone)]
pub enum GameloopState {
    WantsToContinue,
    WantsToQuit,
    WantsToRestart,
    Error(RisError),
}
