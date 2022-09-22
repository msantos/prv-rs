# SYNOPSIS

prv *option*

# DESCRIPTION

prv: pressure relief valve for Unix process pipelines

prv flow controls standard input by sampling lines in a time window.

# EXAMPLES

    while :; do date; done | prv -l 1 -w 10
    Fri Aug 26 00:05:45 EDT 2022
    Fri Aug 26 00:05:54 EDT 2022
    Fri Aug 26 00:06:04 EDT 2022
    ^C

# BUILD

    cargo build

# OPTIONS

-l, --limit *number*
:		message rate limit (default: 0 (no limit))

-w, --window *seconds*
:		message rate window (default: 1 second)

-W, --write-error *exit|drop|block*
:		behaviour if write buffer is full

-v, --verbose
:		verbose mode

-h, --help
:		help
