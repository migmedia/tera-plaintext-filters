# tera-plaintext-filters

[![crates.io](https://img.shields.io/crates/v/tera-plaintext-filters.svg)](https://crates.io/crates/tera-plaintext-filters)
[![crates.io](https://img.shields.io/crates/d/tera-plaintext-filters.svg)](https://crates.io/crates/tera-plaintext-filters)
[![Documentation](https://docs.rs/tera-plaintext-filters/badge.svg)](https://docs.rs/tera-plaintext-filters)

Filters for the [Tera](https://github.com/Keats/tera) engine, useful for plaintext file generation.

Howto generate plaintext files with tera with alignment.

## Example

To render such table:

```markdown
| No |      *Name*        |  Score |
|----|--------------------|--------|
| 1. |      Charly        |   3000 |
| 2. |     Alexander      |    800 |
| 3. |     Josephine      |    760 |
```

### Tera Template

```
| No |    *Name*   |     Score |
|----|-------------|-----------|
{% for member in team | slice(end=10) %}
| {{- loop.index ~ '.' | left_align(length=4) -}} | {{- member.name | center(length=20) -}} | {{- member.score | right_align(length=10) -}} |
{% endfor %}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
