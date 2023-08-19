# Release

_DotHyphen Napi_ is published in [npmjs](https://www.npmjs.com/package/@isfegu/dothyphen-napi).

## How to release DotHyphen Napi

1. The code to publish must be in _main_ branch, merged after a _pull request_.
2. The crate version must be updated in:
    1. the [Cargo.toml](./Cargo.toml) file using SemVer.
    2. the [package.json](./package.json) file using SemVer.
3. A tag must be created in the last commit with the code to publish. Read [how to add tags](https://github.com/isfegu/samuel/blob/main/README.md#versioning-and-tags) in _Samuel_.
4. The publication of _DotHyphen Napi_ must be done using the [Github Action Workflow](../.github/workflows/cd.yml):
    1. Selecting the member to publish.
    2. Selecting the tag to publish.
