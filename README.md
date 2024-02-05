# `evtxtools`

This package aims to be a collection of tools for forensic analysis of evtx files


> **Warning**
> This Repository has been moved to <https://github.com/janstarke/dfir-toolkit>
>
> You can install the tools by running `cargo install dfir-toolkit`
>
> The tool `processtree` is now part of `evtxanalyze` (https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/evtxanalyze.md#evtxanalyze-pstree)
> 


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

# `evtxls`

Display one or more events from an evtx file

## Usage 

```
Usage: evtxls [OPTIONS] [EVTX_FILE]...

Arguments:
  [EVTX_FILE]...  Name of the evtx file to read from

Options:
  -d, --delimiter <DELIMITER>        use this delimiter instead of generating fixed space columns
  -b, --bodyfile                     produce bodyfile output (ignores the `delimiter` option)
  -i, --event-id <FILTER_EVENT_IDS>  List events with only the specified event ids
  -c, --colors                       highlight interesting content using colors
  -f, --from <NOT_BEFORE>            hide events older than the specified date (hint: use RFC 3339 syntax)
  -t, --to <NOT_AFTER>               hide events newer than the specified date (hint: use RFC 3339 syntax)
  -r, --regex <HIGHLIGHT>            highlight event data based on this regular expression
  -h, --help                         Print help information
  -V, --version                      Print version information

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

## Example (markdown Output)

- `C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe` (`0x89d0`, created *`2022-12-07T23:02:49`*)
  - `C:\Windows\System32\conhost.exe` (`0x78ec`, created *`2022-12-07T23:02:49`*)
  - `C:\Windows\System32\net.exe` (`0x43c4`, created *`2022-12-07T23:03:10`*)
    - `C:\Windows\System32\net1.exe` (`0x59fc`, created *`2022-12-07T23:03:10`*)
  - `C:\Windows\System32\WindowsPowerShell\v1.0\powershell_ise.exe` (`0x952c`, created *`2022-12-07T23:03:18`*)
    - `C:\Windows\System32\conhost.exe` (`0x4f3c`, created *`2022-12-07T23:07:22`*)
    - `C:\Windows\System32\PING.EXE` (`0x85a8`, created *`2022-12-07T23:07:22`*)
    - `C:\Windows\System32\PING.EXE` (`0x86fc`, created *`2022-12-07T23:07:49`*)
    - `C:\Windows\System32\PING.EXE` (`0x7928`, created *`2022-12-07T23:07:59`*)
  - `C:\Windows\System32\net.exe` (`0x8774`, created *`2022-12-07T23:05:31`*)
    - `C:\Windows\System32\net1.exe` (`0x7b5c`, created *`2022-12-07T23:05:31`*)
  - `C:\Windows\System32\net.exe` (`0x9b64`, created *`2022-12-07T23:06:10`*)
    - `C:\Windows\System32\net1.exe` (`0x4fc4`, created *`2022-12-07T23:06:10`*)
  - `C:\Windows\System32\nltest.exe` (`0x5274`, created *`2022-12-07T23:06:31`*)
- `C:\Windows\System32\mstsc.exe` (`0x6494`, created *`2022-12-07T23:08:15`*)

## Example (JSON output)

```json
 "2022-12-07T23:03:10.374631+00:00": {
          "2022-12-07T23:03:10.561683+00:00": {
            "CommandLine": "",
            "MandatoryLabel": "S-1-16-8192",
            "NewProcessId": 23036,
            "NewProcessName": "C:\\Windows\\System32\\net1.exe",
            "ParentProcessName": "C:\\Windows\\System32\\net.exe",
            "ProcessId": 17348,
            "SubjectDomainName": "SAMPLE",
            "SubjectLogonId": "0x101501af",
            "SubjectUserName": "malicious_user",
            "SubjectUserSid": "S-1-5-21-2123242984-816922040-331643106-37430",
            "TargetDomainName": "-",
            "TargetLogonId": "0x0",
            "TargetUserName": "-",
            "TargetUserSid": "S-1-0-0",
            "TokenElevationType": "%%1936",
            "event_record_id": 243719861,
            "timestamp": "2022-12-07T23:03:10.561683Z"
          },
          "CommandLine": "",
          "MandatoryLabel": "S-1-16-8192",
          "NewProcessId": 17348,
          "NewProcessName": "C:\\Windows\\System32\\net.exe",
          "ParentProcessName": "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
          "ProcessId": 35280,
          "SubjectDomainName": "SAMPLE",
          "SubjectLogonId": "0x101501af",
          "SubjectUserName": "malicious_user",
          "SubjectUserSid": "S-1-5-21-2123242984-816922040-331643106-37430",
          "TargetDomainName": "-",
          "TargetLogonId": "0x0",
          "TargetUserName": "-",
          "TargetUserSid": "S-1-0-0",
          "TokenElevationType": "%%1936",
          "event_record_id": 243719860,
          "timestamp": "2022-12-07T23:03:10.374631Z"
        },
```
