{% set rd = readme.repo_data -%}
{% set ap = readme.additional_parts -%}
{% set cd = readme.collaboration_data -%}
# {{ rd.name }}
{% if ap.has_banner %}
![banner]({{ ap.banner_path }})
{% endif -%}
{% if ap.add_standard_readme_badge %}
[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
{%- endif -%}
{% if badges %}
TODO: Put more badges here.{% endif %}

{{ rd.description }}

{% if ap.add_long_description_todo -%}
TODO: Fill out this long description.

{% endif -%}
## Table of Contents

{% if ap.add_security_section -%}
- [Security](#security)
{%- endif -%}
{%- if ap.add_background_section %}
- [Background](#background)
{%- endif %}
- [Install](#install)
- [Usage](#usage)
{% if ap.add_api_section -%}
- [API](#api)
{% endif -%}
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

{% if ap.add_security_section -%}
## Security

{% endif -%}
{% if ap.add_background_section -%}
## Background

{% endif -%}
## Install

```sh
```

## Usage

```sh
```

{% if ap.add_api_section -%}
## API

{% endif -%}
## Maintainers

{% if not cd.use_github_com -%}
  {%- set maintainer_domain = "github.com" -%}
{%- else -%}
  {%- set maintainer_domain = cd.custom_domain -%}
{%- endif -%}

[@{{ cd.maintainer_handle }}](https://{{ maintainer_domain }}/{{ cd.maintainer_handle }})

## Contributing

{% if cd.mention_contributing_file -%}
See [the contributing file](contributing.md)!

{% endif -%}
{%- if cd.allow_prs -%}
PRs accepted.
{%- endif %}

Small note: If editing the README, please conform to the
[standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

{% if cd.use_mit -%}
  MIT
{%- else -%}
  {{ cd.custom_license }}
{%- endif %} ©

{{- " " }}
{%- if cd.use_current_year -%}
  {{- now() | date(format="%Y") -}}
{% else %}
  {{ cd.custom_year }}
{%- endif -%}

{{ " " }}
{{- cd.license_holder }}
