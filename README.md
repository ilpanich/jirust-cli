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

## Configuration
To configure the CLI you need to create a configuration file in your home directory called `.jirust-cli/jirust-cli.toml`.

You cangenerate it (if the file doesn't exist the CLI will ask you to configure itself at the first run and the it will store the config in the file) by:

```bash
jirust-cli config setup
```

And you will be asked to insert the configuration parameters.

## Development
The project is still under heavy development and I will prioritize the features I need the most for my daily programmer life.
Use in production at your own risk.
The content of the openapi Jira lib is automatically generated from the Jira API openapi file:

```bash
openapi-generator-cli generate -g rust -o ~/git/priv/jirust-cli/openapi -i openapi-spec/jira-v3.v3.json --additional-properties=bestFitInt=true,preferUnsignedInt=true,supportMiddleware=true
```

## License
Apache 2.0

## Contribution
Feel free to contribute to the project. I will be happy to accept any help and suggestion on how to make the code better.
