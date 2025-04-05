# narp
## Description
Effectively, this is a network traffick monitoring tool designed to run with root privileges that scans the subnet for devices. It proceeds
to enter that device into a database, and will queue it for scanning in order to detect the operating system. For a large part, this is a
integration of the functionalities of both Nmap and Arpwatch, along with extended functionality such as sending SNMP traps upon scan. Nmap
and its documentation, especially its services and os database has been extremely helpful for this project
## Usage
This tool is meant to be running as a process in the background. Most often this is achieved through some startup script, whether it is 
`narp &` in a script, or for example in Hyprland using `exec-once = narp`. There will be a utility provided to read and get information
stored by the tool, to be made at a later date.
## Getting Started
Setup PostgreSQL as you normally would. Add the PGS_ADMIN='your_admin_username'
