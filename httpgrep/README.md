# Summary:
Does fast async hhtp requests and greps the response for a specified value. Prints urls with matches to stdout.

```
Httpgrep 0.0.1
Takes a file with domains, requests each domains with an optional path and then greps the response for a specified value

USAGE:
    httpgrep.exe [FLAGS] [OPTIONS] --grepValue <grepValue> --inputFile <inputFile>

FLAGS:
    -h, --help           Prints help information
    -k, --ignorecerts    Ignores certificate validation
    -V, --version        Prints version information

OPTIONS:
    -g, --grepValue <grepValue>    Value to match in the response
    -f, --inputFile <inputFile>    A list of domains
    -p, --path <path>              Optional path argument
    -t, --threads <threads>        Specify thread count (default: 10)
```