# `evtxtools`

This package aims to be a collection of tools for forensic analysis of evtx files


# `evtxscan`

Finds time skews in an evtx file

## Example

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

# `evtxls`

Displays one or more events from an evtx file.

## Example

## Usage
```
evtxls 0.2.0
Display one or more events from an evtx file

USAGE:
    evtxls [OPTIONS] <EVTX_FILE>

ARGS:
    <EVTX_FILE>    Name of the evtx file to read from

OPTIONS:
    -h, --help         Print help information
    -i, --id <ID>      show only the one event with this record identifier
        --max <MAX>    filter: maximal event record identifier
        --min <MIN>    filter: minimal event record identifier
    -V, --version      Print version information
```
