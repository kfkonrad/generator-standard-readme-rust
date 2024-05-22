{%- let banner_path -%}
{%- match src.additional_parts.banner_path -%}
  {%- when Some with (val) -%}
    {%- let banner_path = val -%}
  {%- when None -%}
    {%- let banner_path = empty_string -%}
{%- endmatch -%}
{%- let custom_domain -%}
{%- match src.collaboration_data.custom_domain -%}
  {%- when Some with (val) -%}
    {%- let custom_domain = val -%}
  {%- when None -%}
    {%- let custom_domain = empty_string -%}
{%- endmatch -%}
{%- let custom_license -%}
{%- match src.collaboration_data.custom_license -%}
  {%- when Some with (val) -%}
    {%- let custom_license = val -%}
  {%- when None -%}
    {%- let custom_license = empty_string -%}
{%- endmatch -%}
{%- let custom_year -%}
{%- match src.collaboration_data.custom_year -%}
  {%- when Some with (val) -%}
    {%- let custom_year = val -%}
  {%- when None -%}
    {%- let custom_year = empty_string -%}
{%- endmatch -%}

# {{ src.repo_data.name }}
{% if src.additional_parts.has_banner %}
![banner]({{ banner_path }})
{% endif -%}
{% if src.additional_parts.add_standard_readme_badge %}
[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
{%- endif -%}
{% if src.additional_parts.add_more_badges_todo %}
TODO: Mehr Badges einfügen.{% endif %}

{{ src.repo_data.description }}

{% if src.additional_parts.add_long_description_todo -%}
TODO: Lange Beschreibung ausfüllen.

{% endif -%}
## Inhaltsverzeichnis
{% if src.additional_parts.add_security_section -%}
- [Sicherheit](#sicherheit)
{%- endif -%}
{%- if src.additional_parts.add_background_section %}
- [Hintergrund](#hintergrund)
{%- endif %}
- [Installation](#installation)
- [Verwendung](#verwendung)
{% if src.additional_parts.add_api_section -%}
- [API](#api)
{% endif -%}
- [Maintainer](#maintainer)
- [Contributing](#contributing)
- [Lizenz](#lizenz)

{% if src.additional_parts.add_security_section -%}
## Sicherheit

{% endif -%}
{% if src.additional_parts.add_background_section -%}
## Hintergrund

{% endif -%}
## Installation

```sh
```

## Verwendung

```sh
```

{% if src.additional_parts.add_api_section -%}
## API

{% endif -%}
## Maintainer

{% let maintainer_domain -%}
{% if src.collaboration_data.use_github_com -%}
  {%- let maintainer_domain = "github.com" -%}
{%- else -%}
  {%- let maintainer_domain = custom_domain -%}
{%- endif -%}

[@{{ src.collaboration_data.maintainer_handle }}](https://{{ maintainer_domain }}/{{ src.collaboration_data.maintainer_handle }})

## Contributing

{% if src.collaboration_data.mention_contributing_file -%}
Siehe [contributing-Datei](contributing.md)!

{% endif -%}
{%- if src.collaboration_data.allow_prs -%}
PRs werden akzeptiert.
{%- endif %}

Kurze Anmerkung: Beim Editieren des README bitte an die
[standard-readme](https://github.com/RichardLitt/standard-readme) Spezifikation halten.

## Lizenz

{% if src.collaboration_data.use_mit -%}
  MIT
{%- else -%}
  {{ custom_license }}
{%- endif %} ©

{{- " " }}
{%- if src.collaboration_data.use_current_year -%}
  {{- current_year -}}
{% else %}
  {{ custom_year }}
{%- endif -%}

{{ " " }}
{{- src.collaboration_data.license_holder }}
