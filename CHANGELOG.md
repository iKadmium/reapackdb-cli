# Changelog

## [0.3.0](https://github.com/iKadmium/reapackdb-cli/compare/v0.2.0...v0.3.0) (2026-05-24)


### Features

* Add auto-merge for Dependabot PRs ([d360141](https://github.com/iKadmium/reapackdb-cli/commit/d3601412c00ef00e41ff991d89e9894328407b3d))
* Add automated release workflow with cross-platform builds ([8b0445f](https://github.com/iKadmium/reapackdb-cli/commit/8b0445fd5dfff6da65d4f6eb18726d2ef68364c9))
* Add CI workflow for linting and testing ([3d35b4a](https://github.com/iKadmium/reapackdb-cli/commit/3d35b4a1891c3fd956621ec9dd68324131c0808a))
* initial implementation ([0d05fdc](https://github.com/iKadmium/reapackdb-cli/commit/0d05fdc3365ad775197ef2eaf9accbdd2e07d60d))


### Bug Fixes

* Call release workflow directly from release-please ([#21](https://github.com/iKadmium/reapackdb-cli/issues/21)) ([67e3ca2](https://github.com/iKadmium/reapackdb-cli/commit/67e3ca2ce3d5b8af744ba2cc31452b82a5a70b9d))
* Create draft releases, publish after assets attached ([#26](https://github.com/iKadmium/reapackdb-cli/issues/26)) ([d142eab](https://github.com/iKadmium/reapackdb-cli/commit/d142eab902e177fe920445034131ddcb785e0242))
* Exclude aws-lc-rs to avoid GCC memcmp bug ([#23](https://github.com/iKadmium/reapackdb-cli/issues/23)) ([8d3efbe](https://github.com/iKadmium/reapackdb-cli/commit/8d3efbe54c1030de3fbe1d0b1978e6a0695432c0))
* Remove capitalization requirement from PR title linting ([a86c77f](https://github.com/iKadmium/reapackdb-cli/commit/a86c77faf67ed99b5aa1f7c58d56724386592e77))
* style, ini location ([3d953da](https://github.com/iKadmium/reapackdb-cli/commit/3d953dafc1903d182321ae9b82e4f812770201e2))
* Trigger release workflow when release-please creates release ([3b7a7e0](https://github.com/iKadmium/reapackdb-cli/commit/3b7a7e03f324eb2a3868aa08f6ead5bf5fcd4771))
* Use inputs.tag for workflow_call trigger type ([#24](https://github.com/iKadmium/reapackdb-cli/issues/24)) ([02043e1](https://github.com/iKadmium/reapackdb-cli/commit/02043e18da30716cd954dd94d3e4128496db3bca))
* Use native-tls and bundled SQLite for cross-platform builds ([#15](https://github.com/iKadmium/reapackdb-cli/issues/15)) ([48f3fbb](https://github.com/iKadmium/reapackdb-cli/commit/48f3fbbe0961794c87c049d47437faff1d034ffe))
* Use rustls with ring backend and OS certificate store ([#20](https://github.com/iKadmium/reapackdb-cli/issues/20)) ([f9cd214](https://github.com/iKadmium/reapackdb-cli/commit/f9cd214bead15f5ec8249a07723942ddd2c71384))
* Vendor OpenSSL for cross-platform builds ([#17](https://github.com/iKadmium/reapackdb-cli/issues/17)) ([63e0ac9](https://github.com/iKadmium/reapackdb-cli/commit/63e0ac91606b33a76413657bf13dca2a5cac3a83))

## [0.2.0](https://github.com/iKadmium/reapackdb-cli/compare/v0.1.4...v0.2.0) (2026-05-24)


### Features

* Add auto-merge for Dependabot PRs ([d360141](https://github.com/iKadmium/reapackdb-cli/commit/d3601412c00ef00e41ff991d89e9894328407b3d))
* Add automated release workflow with cross-platform builds ([8b0445f](https://github.com/iKadmium/reapackdb-cli/commit/8b0445fd5dfff6da65d4f6eb18726d2ef68364c9))
* Add CI workflow for linting and testing ([3d35b4a](https://github.com/iKadmium/reapackdb-cli/commit/3d35b4a1891c3fd956621ec9dd68324131c0808a))
* initial implementation ([0d05fdc](https://github.com/iKadmium/reapackdb-cli/commit/0d05fdc3365ad775197ef2eaf9accbdd2e07d60d))


### Bug Fixes

* Call release workflow directly from release-please ([#21](https://github.com/iKadmium/reapackdb-cli/issues/21)) ([67e3ca2](https://github.com/iKadmium/reapackdb-cli/commit/67e3ca2ce3d5b8af744ba2cc31452b82a5a70b9d))
* Exclude aws-lc-rs to avoid GCC memcmp bug ([#23](https://github.com/iKadmium/reapackdb-cli/issues/23)) ([8d3efbe](https://github.com/iKadmium/reapackdb-cli/commit/8d3efbe54c1030de3fbe1d0b1978e6a0695432c0))
* Remove capitalization requirement from PR title linting ([a86c77f](https://github.com/iKadmium/reapackdb-cli/commit/a86c77faf67ed99b5aa1f7c58d56724386592e77))
* style, ini location ([3d953da](https://github.com/iKadmium/reapackdb-cli/commit/3d953dafc1903d182321ae9b82e4f812770201e2))
* Trigger release workflow when release-please creates release ([3b7a7e0](https://github.com/iKadmium/reapackdb-cli/commit/3b7a7e03f324eb2a3868aa08f6ead5bf5fcd4771))
* Use inputs.tag for workflow_call trigger type ([#24](https://github.com/iKadmium/reapackdb-cli/issues/24)) ([02043e1](https://github.com/iKadmium/reapackdb-cli/commit/02043e18da30716cd954dd94d3e4128496db3bca))
* Use native-tls and bundled SQLite for cross-platform builds ([#15](https://github.com/iKadmium/reapackdb-cli/issues/15)) ([48f3fbb](https://github.com/iKadmium/reapackdb-cli/commit/48f3fbbe0961794c87c049d47437faff1d034ffe))
* Use rustls with ring backend and OS certificate store ([#20](https://github.com/iKadmium/reapackdb-cli/issues/20)) ([f9cd214](https://github.com/iKadmium/reapackdb-cli/commit/f9cd214bead15f5ec8249a07723942ddd2c71384))
* Vendor OpenSSL for cross-platform builds ([#17](https://github.com/iKadmium/reapackdb-cli/issues/17)) ([63e0ac9](https://github.com/iKadmium/reapackdb-cli/commit/63e0ac91606b33a76413657bf13dca2a5cac3a83))

## [0.1.4](https://github.com/iKadmium/reapackdb-cli/compare/v0.1.3...v0.1.4) (2026-05-24)


### Bug Fixes

* Call release workflow directly from release-please ([#21](https://github.com/iKadmium/reapackdb-cli/issues/21)) ([67e3ca2](https://github.com/iKadmium/reapackdb-cli/commit/67e3ca2ce3d5b8af744ba2cc31452b82a5a70b9d))
* Exclude aws-lc-rs to avoid GCC memcmp bug ([#23](https://github.com/iKadmium/reapackdb-cli/issues/23)) ([8d3efbe](https://github.com/iKadmium/reapackdb-cli/commit/8d3efbe54c1030de3fbe1d0b1978e6a0695432c0))

## [0.1.3](https://github.com/iKadmium/reapackdb-cli/compare/v0.1.2...v0.1.3) (2026-05-24)


### Bug Fixes

* Trigger release workflow when release-please creates release ([3b7a7e0](https://github.com/iKadmium/reapackdb-cli/commit/3b7a7e03f324eb2a3868aa08f6ead5bf5fcd4771))
* Use rustls with ring backend and OS certificate store ([#20](https://github.com/iKadmium/reapackdb-cli/issues/20)) ([f9cd214](https://github.com/iKadmium/reapackdb-cli/commit/f9cd214bead15f5ec8249a07723942ddd2c71384))

## [0.1.2](https://github.com/iKadmium/reapackdb-cli/compare/v0.1.1...v0.1.2) (2026-05-24)


### Bug Fixes

* Vendor OpenSSL for cross-platform builds ([#17](https://github.com/iKadmium/reapackdb-cli/issues/17)) ([63e0ac9](https://github.com/iKadmium/reapackdb-cli/commit/63e0ac91606b33a76413657bf13dca2a5cac3a83))

## [0.1.1](https://github.com/iKadmium/reapackdb-cli/compare/v0.1.0...v0.1.1) (2026-05-24)


### Bug Fixes

* Use native-tls and bundled SQLite for cross-platform builds ([#15](https://github.com/iKadmium/reapackdb-cli/issues/15)) ([48f3fbb](https://github.com/iKadmium/reapackdb-cli/commit/48f3fbbe0961794c87c049d47437faff1d034ffe))

## 0.1.0 (2026-05-24)


### Features

* Add auto-merge for Dependabot PRs ([d360141](https://github.com/iKadmium/reapackdb-cli/commit/d3601412c00ef00e41ff991d89e9894328407b3d))
* Add automated release workflow with cross-platform builds ([8b0445f](https://github.com/iKadmium/reapackdb-cli/commit/8b0445fd5dfff6da65d4f6eb18726d2ef68364c9))
* Add CI workflow for linting and testing ([3d35b4a](https://github.com/iKadmium/reapackdb-cli/commit/3d35b4a1891c3fd956621ec9dd68324131c0808a))
* initial implementation ([0d05fdc](https://github.com/iKadmium/reapackdb-cli/commit/0d05fdc3365ad775197ef2eaf9accbdd2e07d60d))


### Bug Fixes

* Remove capitalization requirement from PR title linting ([a86c77f](https://github.com/iKadmium/reapackdb-cli/commit/a86c77faf67ed99b5aa1f7c58d56724386592e77))
* style, ini location ([3d953da](https://github.com/iKadmium/reapackdb-cli/commit/3d953dafc1903d182321ae9b82e4f812770201e2))
