# `evtxtools`

This package aims to be a collection of tools for forensic analysis of evtx files


# `evtxscan`

Finds time skews in an evtx file

## Example

<img src="https://github.com/janstarke/evtxtools/blob/master/doc/img/evtxscan1.png?raw=true">

<img src="https://github.com/janstarke/evtxtools/blob/master/doc/img/evtxscan2.png?raw=true">

## Usage

```
evtxscan 0.2.0
Find time skews in an evtx file

USAGE:
    evtxscan [OPTIONS] <EVTX_FILE>

ARGS:
    <EVTX_FILE>    name of the evtx file to scan

OPTIONS:
    -h, --help
            Print help information

    -N, --negative-tolerance <NEGATIVE_TOLERANCE>
            negative tolerance limit (in seconds): time skews to the past below this limit will be
            ignored [default: 5]

    -S, --show-records
            display also the contents of the records befor and after a time skew

    -V, --version
            Print version information
```

# `evtxcat`

Displays one or more events from an evtx file.

## Example

<img src="https://github.com/janstarke/evtxtools/blob/master/doc/img/evtxls.png?raw=true">

## Usage
```
evtxcat 1.1.0
Display one or more events from an evtx file

USAGE:
    evtxcat [OPTIONS] <EVTX_FILE>

ARGS:
    <EVTX_FILE>    Name of the evtx file to read from

OPTIONS:
    -F, --format <FORMAT>    [possible values: json, xml]
    -h, --help               Print help information
    -i, --id <ID>            show only the one event with this record identifier
        --max <MAX>          filter: maximal event record identifier
        --min <MIN>          filter: minimal event record identifier
    -T, --hide-table         don't display the records in a table format
    -V, --version            Print version information
```

# `processtree`

## Usage

```
reconstructs a process tree, based on Windows audit logs

Usage: processtree [OPTIONS] <EVTX_FILE>

Arguments:
  <EVTX_FILE>  Name of the evtx file to parse

Options:
  -U, --username <USERNAME>  display only processes of this user (case insensitive regex search)
  -F, --format <FORMAT>      [default: json] [possible values: json, markdown]
  -v, --verbose...           More output per occurrence
  -q, --quiet...             Less output per occurrence
  -h, --help                 Print help information
  -V, --version              Print version information
```