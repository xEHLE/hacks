#!/usr/bin/python3
import json
import re
import sys

if len(sys.argv) < 2:
    print("Usage: <cve json file> <min cve score> <cve output file> <wordlist output file>")
    exit()
if len(sys.argv) != 5:
    print("wrong number of arguments")



cve_score = float(sys.argv[2])
file = open(sys.argv[1], "r", encoding="utf8")
outfile = open(sys.argv[3], "w")

wordlist = open(sys.argv[4], "a")

data = json.load(file)

matches = []
out = []

for cve in data["CVE_Items"]:
    try:
        cve_id = cve["cve"]["CVE_data_meta"]["ID"]
        cve_description = cve["cve"]["description"]["description_data"][0]["value"]
        cve_score = cve["impact"]["baseMetricV3"]["cvssV3"]["baseScore"]
    except:
        continue
    if cve_score > 6.0:
        out.append({"cve_num":cve_id,"description":cve_description,"score":cve_score})
        m = re.match('\ ?[^\ ]*?\/[^\ ]*?\.?[^\ ]*?\ ',cve_description)
        if m != None:
            match = m.group(0).strip().strip("'")
            if not match.endswith(".c"):
                if not match.startswith("http:"):
                    if not match.startswith("https:"):
                        if match not in matches:
                            matches.append(match)
    else:
        continue

outfile.write(json.dumps(out))

for i in matches:
    wordlist.write(f'{i}\n')
wordlist.close()
outfile.close()
file.close()