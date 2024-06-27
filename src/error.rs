use inquire::InquireError;
use thiserror::Error;

use crate::map::GunfightMap;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not find GameMap {0} in stats.")]
    GunfightMapNotFound(GunfightMap),

    #[error("Failed creating stats. {0:#?}")]
    FailedCreatingStats(Vec<Self>),

    #[error("failed to get user input")]
    FailedUserPrompt(#[from] InquireError),
}
