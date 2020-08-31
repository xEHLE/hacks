#!/usr/bin/python3
import json
import sys

if len(sys.argv) < 2:
    print("Usage: {} <json masscan output> <optional nmap flags> \ndefault command: nmap -p <masscan ports> -oA <ip> -sC -sV -T5 -Pn <ip> (-p and -oA cannot be changed)".format(sys.argv[0]))
    exit()


masscan_output = sys.argv[1]

ranges = open(masscan_output, "r")
ranges = json.load(ranges)

data = []

for entry in ranges:
    block = {}
    
    ip = entry["ip"]
    for i in data:
        if ip == i["ip"]:
            i["ports"].append(entry["ports"][0]["port"])

    
    block["ip"] = ip
    block["ports"] = []

    port = entry["ports"][0]["port"]
    block["ports"].append(port)
        
        
    data.append(block)
    

command = "nmap -p {} -oA {} "

if len(sys.argv) > 2:
    for i in data:
        temp_command = command
        ip = i["ip"]
        ports = ",".join([str(k) for k in i["ports"]])

        arguments = " ".join([i for i in sys.argv[2:]])
        temp_command += arguments
        temp_command += " " + str(ip)

        print(temp_command.format(ports, ip))
else:
    for i in data:
        
        ip = i["ip"]
        ports = ",".join([str(k) for k in i["ports"]])
        
        command = "nmap -p {} -oA {} -sC -sV -T5 -Pn {}".format(ports, ip, ip)
        print(command)
