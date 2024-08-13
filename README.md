# JiRust-CLI
JiRust-CLI will be a simple Jira CLI developed in rust languages.
The main goals are
* to provide a simple and easy way to interact with Jira using the terminal
* to experience myself with rust language
* to provide an implementation of the Jira API in rust I cannot find at the moment

## Installation
TO install it simply run:

```bash
cargo install jirust-cli
```

## Usage
To use it simply run:
```bash
jirust-cli --help
```

And you will see the help message.

### Configuration
To configure the CLI you need to create a configuration file in your home directory called `.jirust-cli/jirust-cli.toml`.

You cangenerate it (if the file doesn't exist the CLI will ask you to configure itself at the first run and the it will store the config in the file) by:

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

## Development
The project is still under heavy development and I will prioritize the features I need the most for my daily programmer life.
Use in production at your own risk.
The content of the openapi Jira lib is automatically generated from the Jira API openapi file:

```bash
openapi-generator-cli generate -g rust -o ~/git/priv/jirust-cli/jira_v3_openapi -i jira-v3-openapi-spec/swagger.v3.json --additional-properties=bestFitInt=true,preferUnsignedInt=true,supportMiddleware=true
```

## Documentation
The documentation is generated using `cargo doc --open` and it will open the documentation in your browser.
It is also available upon publish on [jira_v3_openapi](https://docs.rs/jira_v3_openapi) for the Jira REST API v3 lib crate and on [JiRust-CLI docs.rs](https://docs.rs/jirust-cli) for the lib crate used by the binary..
Since everything is still under heavy development the documentation is not complete and it will be updated as the project evolves.

## Release
Everything is currently released upon the [crates.io](https://crates.io) platform, once new updates are available:
* [jira_v3_openapi](https://crates.io/crates/jira_v3_openapi)
* [jirust-cli](https://crates.io/crates/jirust-cli)

## Roadmap
Next features to be integrated and supported are:
* Issues management
* Projects management

Those are mandatory for my work and I will prioritize them; other features will be added as needed.
You can request a feature by opening an issue or you can provide an implementation compliant with what is currently developed in a PR.

## License
Apache 2.0

## Contribution
Feel free to contribute to the project. I will be happy to accept any help and suggestion on how to make the code better or even to integrate new features developed by the community.


## Support the project
Since I work as developer and manager during the day, I will develop this project during the night and weekends, once I have time.
If you want to support the project bringing me a coffee to keep me awake and coding, you can do it by clicking the link below:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/ilpanich)

Thank you for your support!
