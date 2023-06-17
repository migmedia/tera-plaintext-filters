# tera-plaintext-filters

Filters for the [Tera](https://github.com/Keats/tera) engine, useful for plaintext file generation.

Howto generate plaintext files with tera with alignment.

### Example

| No |      *Name*        |  Score |
|----|--------------------|--------|
| 1. |      Charly        |   3000 |
| 2. |     Alexander      |    800 |
| 3. |     Josephine      |    760 |


### Tera Template
```
| No |    *Name*   |     Score |
|----|-------------|-----------|
{% for member in team | slice(end=10) %}
| {{- loop.index ~ '.' | left_align(length=4) -}} | {{- member.name | center(length=20) -}} | {{- member.score | right_align(length=10) -}} |
{% endfor %}
```


