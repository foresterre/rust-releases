#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Component {
    pub name: &'static str,
    // In the Rust distribution there are both Components and Extensions.
    // The latter are optionally installed by Rustup,
    // while the former are installed by default.
    //
    // While this is useful to convey to other tooling (which might depend on
    // this library), they're not different in their implementation; therefor,
    // both are used under the name `Component` here.
    //
    // See also: https://forge.rust-lang.org/infra/channel-layout.html#content-of-channel-manifests
    pub optional: bool,
    // todo!
}
