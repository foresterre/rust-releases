// The fields of a release which a release manifest can provide on top of the version
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Detail {
    release_date: bool,
    toolchains: bool,
}

impl Detail {
    pub fn all() -> Self {
        Self {
            release_date: true,
            toolchains: true,
        }
    }

    pub fn release_date() -> Self {
        Self {
            release_date: true,
            toolchains: false,
        }
    }

    pub fn toolchains() -> Self {
        Self {
            release_date: false,
            toolchains: true,
        }
    }

    pub fn with_release_date(self) -> Self {
        Self {
            release_date: true,
            ..self
        }
    }

    pub fn with_toolchains(self) -> Self {
        Self {
            toolchains: true,
            ..self
        }
    }

    pub fn includes_release_date(&self) -> bool {
        self.release_date
    }

    pub fn includes_toolchains(&self) -> bool {
        self.toolchains
    }

    pub fn is_empty(&self) -> bool {
        !self.release_date && !self.toolchains
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_detail_by_default() {
        let detail = Detail::default();

        assert!(detail.is_empty());
        assert!(!detail.includes_release_date());
        assert!(!detail.includes_toolchains());
    }

    #[test]
    fn a_single_field() {
        assert!(Detail::release_date().includes_release_date());
        assert!(!Detail::release_date().includes_toolchains());
        assert!(Detail::toolchains().includes_toolchains());
        assert!(!Detail::toolchains().includes_release_date());
    }

    #[test]
    fn every_field() {
        let detail = Detail::all();

        assert!(!detail.is_empty());
        assert!(detail.includes_release_date());
        assert!(detail.includes_toolchains());
    }

    #[test]
    fn add_a_field() {
        assert_eq!(Detail::release_date().with_toolchains(), Detail::all());
        assert_eq!(Detail::toolchains().with_release_date(), Detail::all());
    }
}
