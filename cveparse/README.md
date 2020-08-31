# Summary:
Parses cve data and spits out all cve's above a specified severity rating and tries to extract web paths to turn into a wordlist.


# Usage:
Download cve data in json format from https://nvd.nist.gov/vuln/data-feeds#JSON_FEED

```
./cveparse <cve json file> <min cve score as float> <cve output file> <wordlist output file>
```
