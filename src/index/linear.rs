use crate::Release;
use std::iter;

/// An iterator over the latest stable releases, with only the latest patch version included.
/// NB: Assumes releases are ordered from most to least recent on iterator initialisation.
pub struct StableReleaseIterator<'release, I: Iterator<Item = &'release Release>> {
    pub(crate) iter: iter::Peekable<I>,
}

impl<'release, I: Iterator<Item = &'release Release>> Iterator
    for StableReleaseIterator<'release, I>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.iter.next();

        current.map(|it| {
            let minor = it.version().minor;

            while let Some(release) = self.iter.peek() {
                if release.version().minor == minor {
                    self.iter.next();
                } else {
                    break;
                }
            }

            it
        })
    }
}
