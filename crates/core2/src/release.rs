use crate::Distribution;

#[derive(Debug)]
pub struct StableRelease(Distribution);

#[derive(Debug)]
pub struct BetaRelease(Distribution);

#[derive(Debug)]
pub struct NightlyRelease(Distribution);
