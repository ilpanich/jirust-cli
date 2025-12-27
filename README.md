![icon](https://github.com/ilpanich/jirust-cli/blob/main/images/jirust-cli.png)

# JiRust-CLI

| Component | Build | Crate | docs | rust-clippy | npm | coverage |
|:---------:|:-----:|:-----:|:----:|:-----------:|:---:|:--------:|
| jirust-cli | [![build and release jirust-cli](https://github.com/ilpanich/jirust-cli/actions/workflows/build_jirust_cli.yml/badge.svg)](https://github.com/ilpanich/jirust-cli/actions/workflows/build_jirust_cli.yml) | [![Crate](https://img.shields.io/crates/v/jirust-cli.svg)](https://crates.io/crates/jirust-cli) | [![API](https://docs.rs/jirust-cli/badge.svg)](https://docs.rs/jirust-cli)|[![rust-clippy analyze](https://github.com/ilpanich/jirust-cli/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/ilpanich/jirust-cli/actions/workflows/rust-clippy.yml)|[![npm](https://img.shields.io/npm/v/jirust-cli.svg)](https://www.npmjs.com/package/jirust-cli) | [![Coverage Status](https://coveralls.io/repos/github/ilpanich/jirust-cli/badge.svg?branch=main)](https://coveralls.io/github/ilpanich/jirust-cli?branch=main) |
| jira_v3_openapi | [![build and release jira_v3_openapi](https://github.com/ilpanich/jirust-cli/actions/workflows/build_jira_v3.yml/badge.svg)](https://github.com/ilpanich/jirust-cli/actions/workflows/build_jira_v3.yml) | [![Crate](https://img.shields.io/crates/v/jira_v3_openapi.svg)](https://crates.io/crates/jira_v3_openapi) | [![API](https://docs.rs/jira_v3_openapi/badge.svg)](https://docs.rs/jira_v3_openapi) |

JiRust-CLI will be a simple Jira CLI developed in Rust language.
The main goals are
* to provide a simple and easy way to interact with Jira using the terminal
* to experience myself with Rust language
* to provide an implementation of the Jira API in Rust I cannot find at the moment

## Installation
To install it simply run:

```bash
cargo install jirust-cli
```

## Usage
To use the CLI, simply run:
```bash
jirust-cli --help
```

And you will see the help message.

Starting from version 0.14.0 a WASM version of the library is available to be used in Node.js; you can refer to the following sample code (see the Rust  docs for all the available options and parameters):
```javascript
import { run } from "./pkg/jirust_cli.js";

let cmd = run(["project", "list"], {
  auth: {
    auth_token: "jira_auth_token",
  },
  jira: {
    jira_url: "https://jira.atlassian.net",
    standard_resolution: '{"name": "Done"}',
    standard_resolution_comment: "Autoresolved",
    transitions_names: { resolve: ["Resolve Issue"] },
  },
}).then((v, err) => {
  if (err) {
    console.error(err);
  } else {
    console.log(JSON.stringify(v));
    console.log("Done");
  }
});
```

And then you can run this node.js script using:

```bash
node jirust-cli-example.js
```

The sample code file is also included in the repository.

### Configuration
To configure the CLI you need to create a configuration file in your home directory called `.jirust-cli/jirust-cli.toml`.

You can generate it (if the file doesn't exist the CLI will ask you to configure itself at the first run and the it will store the config in the file) by:

```bash
jirust-cli config setup
```

And you will be asked to insert the configuration parameters.

### Features
The CLI is still under heavy development and the features are not complete.
Currently the basic version management is implemented and you can:
* list the versions of a project
* create a new version
* delete a version
* update a version
* release a version
* archive a version
* list all the version-related workitems
* create a new project
* list all the projects
* list all the issue types available in a given project
* list all the fields available in a given project for a given issue type
* Create a new issue
* Show an issue details
* Update an issue (Not yet working, to be fixed)
* Delete an issue
* Transition an issue
* Assign an issue
* Add an attachment to an issue
* Link issues
* Attach files to an issue (with optional YARA scan when built with the `attachment_scan` feature)
* List all the transitions available for an issue
* Search issue using JQL query (basic implementation, as required for my personal use)

Supported output formats are:
* JSON (default)
* Table


## Development
The project is still under heavy development and I will prioritize the features I need the most for my daily programmer life.
Use in production at your own risk.
The content of the openapi Jira lib is automatically generated from the Jira API openapi file, retrieved form the official [Jira API v3 docs](https://developer.atlassian.com/cloud/jira/platform/rest/v3/):

```bash
openapi-generator-cli generate -g rust -o ~/git/priv/jirust-cli/jira_v3_openapi -i jira-v3-openapi-spec/swagger.v3.json --additional-properties=bestFitInt=true,preferUnsignedInt=true,supportMiddleware=true
```

## Documentation
The documentation is generated using `cargo doc --open` and it will open the documentation in your browser.
It is also available upon publish on [jira_v3_openapi docs.rs](https://docs.rs/jira_v3_openapi) for the Jira REST API v3 lib crate and on [JiRust-CLI docs.rs](https://docs.rs/jirust-cli) for the lib crate used by the binary..
Since everything is still under heavy development the documentation is not complete and it will be updated as the project evolves.
Please notice that jira_v3_openapi docs is autogenerated from the openapi file and it is not complete and not always correct, but I wouldn't spend time to improve it..

Also some of the jirust-cli documentation is auto-generated by AI, I will improve it every time I found something to be fixed inside it.

## Release
Everything is currently released upon the [crates.io](https://crates.io) platform, once new updates are available:
* [jira_v3_openapi](https://crates.io/crates/jira_v3_openapi)
* [jirust-cli](https://crates.io/crates/jirust-cli)

## Roadmap
Next features to be integrated and supported are:
* Update Jira issue: check if it works correctly

Those are mandatory for my work and I will prioritize them; other features will be added as needed.
You can request a feature by opening an issue or you can provide an implementation compliant with what is currently developed in a PR.

### Attachments
Upload a file to an issue:

```bash
jirust-cli issue attach -i ISSUE-123 -p /path/to/file.pdf
```

When built with the `attachment_scan` feature, attachments are scanned with YARA before upload. See the configuration notes below to point the scanner at your local or remote rules.

### Attachment scanning (optional)
Attachment scanning is disabled by default during builds to avoid pulling extra dependencies. Build with the feature flag to enable it:

```bash
cargo build -p jirust-cli --features attachment_scan
```

The BIN version for Windows or Linux distributed via github releases already include this feature enabled by default (_full_ version), otherwise it is not included (_basic_ version).

Configure the rules source in `~/.jirust-cli/jirust-cli.toml` (see `jirust-cli/config_example.toml`):

```toml
[yara]
rules_source = "https://github.com/YARAHQ/yara-forge/releases/latest/download/yara-forge-rules-core.zip"
rules_directory = "yara-rules"
cache_file = "yara_rules.cache"
cache_version_file = "yara_rules.cache.version"
```

Rules are downloaded (git or zip) into `~/.jirust-cli/<rules_directory>` and cached locally; subsequent runs reuse the cache unless the source version changes.

## License
Apache 2.0

## Contribution
Feel free to contribute to the project. I will be happy to accept any help and suggestion on how to make the code better or even to integrate new features developed by the community.


## Support the project
Since I work as developer and manager during the day, I will develop this project during the night and weekends, once I have time.
If you want to support the project bringing me a coffee to keep me awake and coding, you can do it by clicking the link below:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/ilpanich)

Thank you for your support!
