# Summary:
Rapid7 offers data from SSL certificates under https://opendata.rapid7.com/sonar.ssl/. They are split up into multiple files: hosts,names,certs,endpoints.
If you are just interested in the CN->IP mappings you need to parse and combine the hosts and names files as that data is split up. This little program just maps the CN back to the IP's.

# Usage:
```
./certsort.exe <names file> <hosts file>
```