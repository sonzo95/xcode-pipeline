# xcode-pipeline

## Goal

This project aims to automate the 'archive' and 'publish' phases of iOS apps. It will be based on xcode `.xcworkspace` files and 'schemes'.

## Design

The main function should be able to parse arguments and create an XCodebuildContext, that allows to execute specific operations.

## Desired Features

### Args

- `-a`, `--all`: process all schemes
- `-s`, `--schema`: add a specific schema to the TODO list
- `-g`, `--group`: add a group of schemas to the TODO list (i.e. "noprod")
- `-d`, `--dry-run`: run all commands without them having effect

### Envs

- `WORKSPACE`: define the `.xcworkspace` path
- `SCHEMES`: list all possible schemes for validation and `-a` arg
