//! `se.reciba.api.submit.RecipePullRequests`.

use crate::recipe_source::GeneratedRecipe;

/// `sealed trait PullRequestFailure`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PullRequestFailure {
    /// `case object BranchAlreadyExists`.
    BranchAlreadyExists,
    /// `final case class GithubRejected(message: String)`.
    GithubRejected(String),
}

impl PullRequestFailure {
    /// The message carried by `GithubRejected`, if any. `BranchAlreadyExists`
    /// has no message, like the Scala case object.
    pub fn message(&self) -> Option<&str> {
        match self {
            PullRequestFailure::BranchAlreadyExists => None,
            PullRequestFailure::GithubRejected(message) => Some(message),
        }
    }
}

pub trait RecipePullRequests {
    fn open(
        &self,
        recipe: &GeneratedRecipe,
        commit_message: &str,
        pull_title: &str,
        pull_body: &str,
    ) -> Result<String, PullRequestFailure>;
}
