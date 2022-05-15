# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
    Add new changelog entries here.
    Each entry may be annotated with "Added", "Changed", "Removed", and "Fixed" titles.

    Example:

    ## [1.0.0] - May 16, 2022

    ### Added
    - New visual identity.

    ### Changed
    - Start using "changelog" over "change log" since it's the common usage.

    ### Removed
    - Section about "changelog" vs "CHANGELOG".

    ### Fixed
    - Fix typos in recent README changes.
    - Update outdated unreleased diff link.
-->

## Unreleased
### Added
- A trait called `Typed` which defines an interface for anything that has a type.
- Various implementations of the former trait so we can retrieve the types of any `Expression`, `Literal` or `Pattern`.
- A new parser error called `NoMatch` which is returned if two patterns don't match.
- Start using human-readable changelogs.

### Removed
- The method  `can_assign_from` in the `Typed` trait has been temporarily commented out.