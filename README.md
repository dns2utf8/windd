# windd

Creates a zero filled file on disk.
Removes said file once the disk runs full.

# Usage

    windd.exe /h
    windd.exe [file_name]

## Why use this tool?

Because it can cleanup after it filled your disk like this:

    dd if=/dev/zero of=/tmp/null bs=16M ; sync ; rm /tmp/null
