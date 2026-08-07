use std::fmt;

const DEFAULT_MAX_PAGES: u32 = 64;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct PageSize(u8);

impl PageSize {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(100);

    pub fn new(size: u8) -> Result<Self, PagingError> {
        if !(Self::MIN.0..=Self::MAX.0).contains(&size) {
            return Err(PagingError::PageSizeOutOfRange { given: size });
        }

        Ok(Self(size))
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

impl Default for PageSize {
    fn default() -> Self {
        Self::MAX
    }
}

impl fmt::Display for PageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct MaxPages(u32);

impl MaxPages {
    pub fn new(pages: u32) -> Result<Self, PagingError> {
        if pages == 0 {
            return Err(PagingError::MaxPagesIsZero);
        }

        Ok(Self(pages))
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

impl Default for MaxPages {
    fn default() -> Self {
        Self(DEFAULT_MAX_PAGES)
    }
}

impl fmt::Display for MaxPages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum PagingError {
    #[error(
        "A page size of '{given}' is out of range: the GitHub API accepts a page size of '{min}' up to and including '{max}'",
        min = PageSize::MIN,
        max = PageSize::MAX,
    )]
    PageSizeOutOfRange { given: u8 },

    #[error("At least one page of releases must be fetched, but the maximum was set to '0'")]
    MaxPagesIsZero,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[yare::parameterized(
        min = { 1 },
        some = { 30 },
        max = { 100 },
    )]
    fn accepted_page_size(size: u8) {
        assert_eq!(PageSize::new(size).unwrap().get(), size);
    }

    #[yare::parameterized(
        zero = { 0 },
        above_max = { 101 },
        way_above_max = { u8::MAX },
    )]
    fn rejected_page_size(size: u8) {
        assert_eq!(
            PageSize::new(size).unwrap_err(),
            PagingError::PageSizeOutOfRange { given: size }
        );
    }

    #[test]
    fn default_page_size_is_the_maximum_accepted_by_the_github_api() {
        assert_eq!(PageSize::default().get(), 100);
    }

    #[test]
    fn accepted_max_pages() {
        assert_eq!(MaxPages::new(1).unwrap().get(), 1);
        assert_eq!(MaxPages::new(u32::MAX).unwrap().get(), u32::MAX);
    }

    #[test]
    fn rejected_max_pages() {
        assert_eq!(MaxPages::new(0).unwrap_err(), PagingError::MaxPagesIsZero);
    }
}
