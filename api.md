# Release

- channel: Channel | Version
- on: Date
- platform_support: Collection<PlatformComponents>

- `fn synthesize_toolchains -> impl IntoIterator<Toolchain>`

# PlatformComponents

- host: Host,
- components: Collection<?Component>,

# Toolchain 

_(without custom toolchain support)_

- channel: `Channel | Version`
- date: `Date`
- host: `Host  { Option<TargetTriple> }`
- components: `Components { Collection<?Component> }`


- `fn is_released() -> bool`
- `fn from_release(platform) -> Result<Self , S::Err>`
-  

# Edition


# CargoFeatures

- LockfileVersion(u8)
- ResolverVersion(u8)

