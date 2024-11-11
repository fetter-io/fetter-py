# fetter

<a href="https://github.com/fetter-io/fetter-rs/actions/workflows/ci.yml">
    <img style="display: inline!important" src="https://img.shields.io/github/actions/workflow/status/fetter-io/fetter-rs/ci.yml?branch=default&label=CI&logo=Github"></img>
</a>
<a href="https://crates.io/crates/fetter">
    <img src="https://img.shields.io/crates/v/fetter?label=crates.io&logo=rust"></img>
</a>
<a href="https://pypi.org/project/fetter/">
    <img src="https://img.shields.io/pypi/v/fetter?label=PyPI&logo=pypi"></img>
</a>
<!-- <a href="https://crates.io/crates/fetter">
    <img src="https://img.shields.io/crates/d/fetter?label=Downloads&logo=rust"></img>
</a> -->

## System-wide Python package discovery, validation, and allow-listing.


The `fetter` command-line tool scans and validates Python packages across virtual environments or entire systems, ensuring packages conform to specified requirements or lock files. It identifies unapproved or vulnerable packages, supports continuous integration through 'pre-commit', and offers excellent performance thanks to a multi-threaded Rust implementation.


* 🔎 System Scanning: Finds Python packages across system environments.
* ⚖️ Package Validation: Checks installed packages against requirements.txt, pyproject.toml, or lock files sourced locally, via URLs, or via `git` repositories.
* 🛡️ Vulnerability Audit: Scans packages for security vulnerabilites in the Open Source Vulnerability database.
* ⚙️ CI Integration: Validate and audit with `pre-commit` [hooks](#Using-fetter-with-pre-commit).
* 🚀 Fast: Multi-threaded Rust implementation.
* 🪢 Bound Requirements: Derive lock-file-like bound requirements from observed system packages.
* 🧹 Search and Purge: Find and remove packages across environments.
* 🧩 Flexible Output: Display results in terminal or export to delimited files.




## Installing `fetter`

While available as a pure Rust binary ([crates](https://crates.io/crates/fetter)), `fetter` is easily installed via a Python package ([pypi](https://pypi.org/project/fetter)):

```shell
$ pip install fetter
$ fetter --help
```

Alternatively, as `fetter` can operate on multiple virtual environments, installation via [`pipx`](https://pipx.pypa.io) might be desirable:

```shell
$ pipx install fetter
$ fetter --version
```



## Using `fetter` from the command line

For complete command-line documentation, see [CLI Documentation](#Command-Line-Interface-Documentation).

By default, `fetter` will search for all packages in `site-packages` directories discoverable from all Python executables found in the system or user virtual environments. Depending on your system, this command might take several seconds.

```shell
$ fetter scan
```

To limit scanning to `site-packages` directories associated with a specific Python executable, the `--exe` (or `-e`) argument can be supplied.

```shell
$ fetter -e python3 scan
Package                   Site
certifi-2024.8.30         ~/.env-wp/lib/python3.12/site-packages
charset_normalizer-3.4.0  ~/.env-wp/lib/python3.12/site-packages
idna-3.10                 ~/.env-wp/lib/python3.12/site-packages
jinja2-3.1.3              ~/.env-wp/lib/python3.12/site-packages
markupsafe-2.1.5          ~/.env-wp/lib/python3.12/site-packages
pip-21.1.1                ~/.env-wp/lib/python3.12/site-packages
requests-2.32.3           ~/.env-wp/lib/python3.12/site-packages
setuptools-56.0.0         ~/.env-wp/lib/python3.12/site-packages
urllib3-2.2.3             ~/.env-wp/lib/python3.12/site-packages
zipp-3.18.1               ~/.env-wp/lib/python3.12/site-packages
```

This evnironment was built from this "requirements.txt":

```
jinja2==3.1.3
zipp==3.18.1
requests==2.32.3
```

To validate that the installed packages match the packages specified in "requirements.txt", we can use the `fetter validate` command, again targeting our active Python with `-e python3`, and providing "requirements.txt" to the `--bound` argument.

```shell
$ fetter -e python3 validate --bound requirements.txt
Package                   Dependency  Explain     Sites
certifi-2024.8.30                     Unrequired  ~/.env-wp/lib/python3.12/site-packages
charset_normalizer-3.4.0              Unrequired  ~/.env-wp/lib/python3.12/site-packages
idna-3.10                             Unrequired  ~/.env-wp/lib/python3.12/site-packages
markupsafe-2.1.5                      Unrequired  ~/.env-wp/lib/python3.12/site-packages
pip-21.1.1                            Unrequired  ~/.env-wp/lib/python3.12/site-packages
setuptools-56.0.0                     Unrequired  ~/.env-wp/lib/python3.12/site-packages
urllib3-2.2.3                         Unrequired  ~/.env-wp/lib/python3.12/site-packages
```

The `--superset` command can be provided to accept packages that are not defined in the bound requirements.

```shell
$ fetter -e python3 validate --bound requirements.txt --superset
```

If we update `zipp` to version 3.20.2 and re-run validation, `fetter` will report these as "Misdefined" records

```shell
$ fetter -e python3 validate --bound requirements.txt --superset
Package      Dependency    Explain     Sites
zipp-3.20.2  zipp==3.18.1  Misdefined  ~/.env-wp/lib/python3.12/site-packages
```

If we remove the `zipp` package entirely, `fetter` identifies this as a "Missing" record:

```shell
$ fetter -e python3 validate --bound requirements.txt --superset
Package  Dependency    Explain  Sites
         zipp==3.18.1  Missing
```

If we want to permit the absence of specified packages, the `--subset` flag can be used:

```shell
$ fetter -e python3 validate --bound requirements.txt --superset --subset
```

Using the `fetter audit` command, details are provided for every vulnerability associated with installed packages.

```shell
$ fetter -e python3 audit
Package            Vulnerabilities      Attribute  Value
jinja2-3.1.3       GHSA-h75v-3vvj-5mfj  URL        https://osv.dev/vulnerability/GHSA-h75v-3vvj-5mfj
                                        Summary    Jinja vulnerable to HTML attribute injection when passing ...
                                        Reference  https://nvd.nist.gov/vuln/detail/CVE-2024-34064
                                        Severity   CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:U/C:L/I:L/A:N
pip-21.1.1         GHSA-mq26-g339-26xf  URL        https://osv.dev/vulnerability/GHSA-mq26-g339-26xf
                                        Summary    Command Injection in pip when used with Mercurial
                                        Reference  https://nvd.nist.gov/vuln/detail/CVE-2023-5752
                                        Severity   CVSS:4.0/AV:L/AC:L/AT:N/PR:L/UI:N/VC:N/VI:H/VA:N/SC:N/SI:N/SA
                   PYSEC-2023-228       URL        https://osv.dev/vulnerability/PYSEC-2023-228
                                        Reference  https://mail.python.org/archives/list/security-announce@py...
                                        Severity   CVSS:3.1/AV:L/AC:L/PR:L/UI:N/S:U/C:N/I:L/A:N
setuptools-56.0.0  GHSA-cx63-2mw6-8hw5  URL        https://osv.dev/vulnerability/GHSA-cx63-2mw6-8hw5
                                        Summary    setuptools vulnerable to Command Injection via package URL
                                        Reference  https://nvd.nist.gov/vuln/detail/CVE-2024-6345
                                        Severity   CVSS:4.0/AV:N/AC:L/AT:P/PR:N/UI:A/VC:H/VI:H/VA:H/SC:N/SI:N/SA
                   GHSA-r9hx-vwmv-q579  URL        https://osv.dev/vulnerability/GHSA-r9hx-vwmv-q579
                                        Summary    pypa/setuptools vulnerable to Regular Expression Denial of...
                                        Reference  https://nvd.nist.gov/vuln/detail/CVE-2022-40897
                                        Severity   CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:H
                   PYSEC-2022-43012     URL        https://osv.dev/vulnerability/PYSEC-2022-43012
                                        Reference  https://github.com/pypa/setuptools/blob/fe8a98e696241487ba...
zipp-3.18.1        GHSA-jfmj-5v4g-7637  URL        https://osv.dev/vulnerability/GHSA-jfmj-5v4g-7637
                                        Summary    zipp Denial of Service vulnerability
                                        Reference  https://nvd.nist.gov/vuln/detail/CVE-2024-5569
                                        Severity   CVSS:4.0/AV:L/AC:L/AT:N/PR:N/UI:N/VC:N/VI:N/VA:H/SC:N/SI:N/SA
```

For additional discussion and examples of `fetter` commands and functionality, see [System-Wide Python Package Control](https://github.com/fetter-io/fetter-rs/blob/default/doc/articles/swppc/swppc.md).




## Using `fetter` with pre-commit

Two `fetter` commands can be run via [pre-commit](https://pre-commit.com/) hooks for continuous integration of Python package controls.


### Running `fetter validate` with `pre-commit`.


The `fetter validate` command permits validating that the actually installed Python packages in the current environment are what are defined to be installed, as specified by a requirements.txt file, a pyproject.toml file, or a lock file such as one produced by `uv`.

The `fetter validate` command takes a required argument, `--bound`, to specify that path or URL to the file to be used to define the bound requirements. The optional `--superset` argument permits packages not defined in the bound requirements to be present. The optional `--subset` argument permits not all packages in the bound requirements to be present.

To run `fetter validate` with `pre-commit`, add the following to your `.pre-commit-config.yaml`.


```yaml
repos:
- repo: https://github.com/fetter-io/fetter-rs
  rev: v1.1.0
  hooks:
    - id: fetter-validate
      args: [--bound, {FILE}, --superset, --subset]

```


### Running `fetter audit` with `pre-commit`.

The `fetter audit` command will check for cybersecurity vulnerabilities issued for all installed Python packages in the current environment. Vulnerabilities are searched for in the Open Source Vulnerability (OSV) database.

To run `fetter audit` with `pre-commit`, add the following to your `.pre-commit-config.yaml`. Note that, as searching vulnerabilities can take time, this hook is likely better deployed as a `pre-push` rather than a `pre-commit` hook.

```yaml
repos:
- repo: https://github.com/fetter-io/fetter-rs
  rev: v1.1.0
  hooks:
    - id: fetter-audit
```



## Command-Line-Interface Documentation

### Global Options

- `--exe, -e <FILES>`: Provide zero or more executable paths to derive site package locations. If omitted, all discoverable executables will be used.
- `--quiet, -q`: Disable logging and terminal animation.
- `--user_site`: Force inclusion of the user site-packages, even if it is not activated. Defaults to only including if the interpreter is configured to use it.

### Command: `fetter scan`

- Description: Scan the environment to report on installed packages.
- Subcommands
  - `display`: Show scan results in the terminal.
  - `write`: Save scan results to a file.
    - `--output, -o <FILE>`: Specify the output file.
    - `--delimiter, -d <char>`: Set the delimiter for the file (default: `,`).

### Command: `fetter search`

- Description: Search the environment to report on installed packages based on a pattern.
- Options
  - `--pattern, -p <STRING>`: Specify a glob-like pattern to match packages.
  - `--case`: Enable case-sensitive pattern matching.
- Subcommands
  - `display`: Show search results in the terminal.
  - `write`: Save search results to a file.
    - `--output, -o <FILE>`: Specify the output file.
    - `--delimiter, -d <char>`: Set the delimiter for the file (default: `,`).

### Command: `fetter count`

- Description: Count discovered executables, sites, and packages.
- Subcommands
  - `display`: Show count results in the terminal.
  - `write`: Save count results to a file.
    - `--output, -o <FILE>`: Specify the output file.
    - `--delimiter, -d <char>`: Set the delimiter for the file (default: `,`).

### Command: `fetter derive`

- Description: Derive new requirements from discovered packages.
- Options
  - `--anchor, -a <BOUND>`: Set the nature of the bound in the derived requirements. (Use a value from `CliAnchor`)
- Subcommands
  - `display`: Show derived requirements in the terminal.
  - `write`: Save derived requirements to a file.
    - `--output, -o <FILE>`: Specify the output file.

### Command: `fetter validate`

- Description: Validate if packages conform to a specified validation target.
- Options
  - `--bound, -b <FILE>`: Path or URL to the file containing bound requirements.
  - `--bound-options <OPTIONS>`: Names of additional optional dependency groups.
  - `--subset`: Allow the observed packages to be a subset of the bound requirements.
  - `--superset`: Allow the observed packages to be a superset of the bound requirements.
- Subcommands
  - `display`: Show validation results in the terminal.
  - `json`: Print validation results in JSON format.
  - `write`: Save validation results to a file.
    - `--output, -o <FILE>`: Specify the output file.
    - `--delimiter, -d <char>`: Set the delimiter for the file (default: `,`).
  - `exit`: Return an exit code (0 for success, customizable for errors).
    - `--code, -c <INT>`: Specify the error code (default: `3`).

### Command: `fetter audit`

- Description: Search for security vulnerabilities in packages via the OSV DB.
- Options
  - `--pattern, -p <STRING>`: Specify a glob-like pattern to select packages (default: `*`).
  - `--case`: Enable case-sensitive pattern matching.
- Subcommands
  - `display`: Show audit results in the terminal.
  - `write`: Save audit results to a file.
    - `--output, -o <FILE>`: Specify the output file.
    - `--delimiter, -d <char>`: Set the delimiter for the file (default: `,`).

### Command: `fetter unpack-count`

- Description: Count all installed package artifacts.
- Options
  - `--pattern, -p <STRING>`: Specify a glob-like pattern to select packages (default: `*`).
  - `--case`: Enable case-sensitive pattern matching.

- Subcommands
  - `display`: Show artifact counts in the terminal.
  - `write`: Save artifact counts to a file.
    - `--output, -o <FILE>`: Specify the output file.
    - `--delimiter, -d <char>`: Set the delimiter for the file (default: `,`).

### Command: `fetter unpack-files`

- Description: List the file names of all installed package artifacts.
- Options
  - `--pattern, -p <STRING>`: Specify a glob-like pattern to select packages (default: `*`).
  - `--case`: Enable case-sensitive pattern matching.
- Subcommands
  - `display`: Show artifact file names in the terminal.
  - `write`: Save artifact file names to a file.
    - `--output, -o <FILE>`: Specify the output file.
    - `--delimiter, -d <char>`: Set the delimiter for the file (default: `,`).

### Command: `fetter purge-pattern`

- Description: Purge packages that match a specific pattern.
- Options
  - `--pattern, -p <STRING>`: Specify a glob-like pattern to select packages (default: `*`).
  - `--case`: Enable case-sensitive pattern matching.

### Command: `fetter purge-invalid`

- Description: Purge packages that are invalid based on dependency specification.
- Options
  - `--bound, -b <FILE>`: Path or URL to the file containing bound requirements.
  - `--bound-options <OPTIONS>`: Names of additional optional dependency groups.
  - `--subset`: Allow the observed packages to be a subset of the bound requirements.
  - `--superset`: Allow the observed packages to be a superset of the bound requirements.




## What is New in Fetter

### 1.1.0

Implemented `bound-options` to permit selecting optional dependencies in pyproject.toml files.

### 1.0.0

Implemented home-path display abbreviation with `~`.

Handle combining multiple `DepSpec` in producing `DepManifest`.

Added `--pattern` and `--case` options to `audit`.

Added support for creating `DepManifest` from pyproject.toml.

Permit `DepManifest` to be retrieved from a URL.


### 0.13.0

All subcommands now have their output sub-subcommands set to `display` by default.

The `validate` and `audit` subcommands now return a non-zero exit code when items are found.

The CLI now exits for unsupported platforms.


### 0.12.0

Extended `validate` and `audit` commands to return a non-zero error code if `display` prints records.


### 0.11.0

Implemented variable-width and colored terminal displays.

Implemented terminal spinner for long-running commands.

Added `purge-invalid` and `purge-pattern` commands.

Split `unpack` command into `unpack-count` and `unpack-files`.

Added support to specify `--bound` with a git repository.


### 0.10.0

Added `--user-site` flag to force inclusion of user site packages; otherwise, user site packages are only included if `ENABLE_USER_SITE` is set.

Reimplemented display and delimited table outputs to use a generic trait implementation.


### 0.9.0

Support `--requirement` in requirements files.


### 0.8.0

Package and DepSpec comparisons now remove user.

Package and DepSpec comparisons now accept matching either on requested_revision or commit_id.

URLs are now shown in DepSpec displays.

Delimited file output no longer pads spaces.


### 0.7.0

Validate display now shows paths properly.

Updated validate json output to terminate line and flush buffer.


### 0.6.0

Package and dependency keys are case insensitive.

Improved URL validation between dependency and package by removing user components.

Improved validation JSON output to provided labelled objects.

Improved valiation output to show sorted missing packages.

Renamed validation explain values.

Implemented support for nested requirements.txt.


### 0.5.0

Implemented search command with basic wildcard matching.

Implemented `Arc`-wrapped `PathBuf` for sharable site paths.

Added explanation column to validation results.

Added support for both `--subset` and `--superset` validations.

Implemented `ValidationDigest` for simplified JSON serialization.

Added `JSON` CLI output option for validation results.






