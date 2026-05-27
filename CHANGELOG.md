# Changelog

All notable changes to this project will be documented in this file.
See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---

## Version [0.1.4](https://github.com/johnstonskj/rust-lat-long/compare/v0.1.3..v0.1.4) - 2026-05-27

### Bug Fixes

- **(elevation)** correct typo in elv macro call - ([6dec5d7](https://github.com/johnstonskj/rust-lat-long/commit/6dec5d7e393279133632073f2608d92662c06632))by @johnstonskj
- feature mis-match issues - ([9fdcff5](https://github.com/johnstonskj/rust-lat-long/commit/9fdcff54a15bc0b719db05797203bcbf0e77ab29)) by @johnstonskj
- add missing tarpaulin file - ([058e8e3](https://github.com/johnstonskj/rust-lat-long/commit/058e8e328e016712b499dcf76ec502d7713aaa3a)) by @johnstonskj

### Documentation

- updated examples in README - ([852478f](https://github.com/johnstonskj/rust-lat-long/commit/852478fca7502467ff01bbca91dc4856a162fee8)) by @johnstonskj
- add default changelog for cliff - ([973c6c4](https://github.com/johnstonskj/rust-lat-long/commit/973c6c48d5187381deb15e8ed3f7510bf6186b5b)) by @johnstonskj
- add agent guideline file - ([e8d88b7](https://github.com/johnstonskj/rust-lat-long/commit/e8d88b788781de44f1758e3f486b40d0597d9ee9)) by @johnstonskj
- update changelog - ([d3df695](https://github.com/johnstonskj/rust-lat-long/commit/d3df695e91973e78015448a09ef6ff9ee05362d9)) by @johnstonskj

### Features

- adding tool config files - ([098a895](https://github.com/johnstonskj/rust-lat-long/commit/098a895586e59215ee18f72e5f12bb402f22a8a4)) by @johnstonskj
- add support for labeled decimals - ([017f1b4](https://github.com/johnstonskj/rust-lat-long/commit/017f1b4af8b7213babf909f1bf9bc7f5eca7b57a)) by @johnstonskj
- add UTM band and zone - ([9895452](https://github.com/johnstonskj/rust-lat-long/commit/9895452274aac8f85de57ea44fba1bf273fa6f23)) by @johnstonskj

### Miscellaneous Chores

- **(cargo)** add repository and documentation links - ([6c3ce8c](https://github.com/johnstonskj/rust-lat-long/commit/6c3ce8cdacc74c9303c77fed91dd8b81eaf36dfe))by @johnstonskj
- fix cliff template copy/paste - ([125ddab](https://github.com/johnstonskj/rust-lat-long/commit/125ddab1caaa827e0e631316e86c2e31d6c82a27)) by @johnstonskj
- create changelog for 0.1.3 - ([9566733](https://github.com/johnstonskj/rust-lat-long/commit/95667336799bc0a4f3531a1bcaa548bdb45b494c)) by @johnstonskj
- prepare for version 0.1.4 - ([798681d](https://github.com/johnstonskj/rust-lat-long/commit/798681d415f39b69db05e38580f4682dcbdfad8f)) by @johnstonskj

### Style

- fix unused import warning - ([deed2f5](https://github.com/johnstonskj/rust-lat-long/commit/deed2f57c2dfa88b60264e0c8cb631d0aeb1ac72)) by @johnstonskj
- cargo fmt/clippy - ([cc25f79](https://github.com/johnstonskj/rust-lat-long/commit/cc25f79a47df67a3ddddf398899b85f36c0c4d1f)) by @johnstonskj

### Tests

- **(elevation)** add unit tests for elevation types - ([728115c](https://github.com/johnstonskj/rust-lat-long/commit/728115c3ccf11edd2117a83575cbc92102c50960))by @johnstonskj
- complete tests especially parser - ([9a3e539](https://github.com/johnstonskj/rust-lat-long/commit/9a3e539192b9292430de1f4884b48e6c8f2aabfd)) by @johnstonskj


---

## Version [0.1.3](https://github.com/johnstonskj/rust-lat-long/compare/v0.1.1..v0.1.3) - 2026-05-26

### Documentation

- add licence and readme - ([3e87924](https://github.com/johnstonskj/rust-lat-long/commit/3e87924565d0ea24582e2816eb15a437fdddcc52)) by @johnstonskj

### Features

- complete build system - ([e174cfa](https://github.com/johnstonskj/rust-lat-long/commit/e174cfa89569167876a4f61b092743eeca083b57)) by @johnstonskj
- implement serde support. - ([e75a8f9](https://github.com/johnstonskj/rust-lat-long/commit/e75a8f913a355f7bb281b8bf5fa9ee74b26f2925)) by @johnstonskj
- add altitude and 3d coordinate types - ([31d9a0b](https://github.com/johnstonskj/rust-lat-long/commit/31d9a0b885e45b371932882616304d0573b5dd11)) by @johnstonskj
- enhance Angle trait with conversion and mathematical methods - ([47f8743](https://github.com/johnstonskj/rust-lat-long/commit/47f8743150e671b1ef76b1382cc80758f04ad5bb)) by @johnstonskj

### Miscellaneous Chores

- version bump - ([86986a3](https://github.com/johnstonskj/rust-lat-long/commit/86986a382b0e5fdef5295080919b39474e542de8)) by @johnstonskj
- bump version - ([f3667e5](https://github.com/johnstonskj/rust-lat-long/commit/f3667e531d76fbb8237faac0ecd639b763ec2a10)) by @johnstonskj
- bump package version to 0.1.3 - ([a8c6821](https://github.com/johnstonskj/rust-lat-long/commit/a8c6821b55dbffb848c4f4d9b159435fa71bce18)) by @johnstonskj

### Refactoring

- rename `lat` and `long` modules to `latitude` and `longitude` - ([e192360](https://github.com/johnstonskj/rust-lat-long/commit/e192360982a3f31a87fdef135aaad8cd3bd0b778)) by @johnstonskj
- rename `3d` feature and types to `elevation` - ([5cb6b6c](https://github.com/johnstonskj/rust-lat-long/commit/5cb6b6c81edc94e02b8001bdc3432a7a221eca22)) by @johnstonskj

### Style

- cargo clippy - ([e4e4b1d](https://github.com/johnstonskj/rust-lat-long/commit/e4e4b1d806ceafdc2f6bb9513e1da9ecf0d6d5af)) by @johnstonskj

### New Contributors

* @johnstonskj

<!-- generated by git-cliff | https://git-cliff.org/ -->
