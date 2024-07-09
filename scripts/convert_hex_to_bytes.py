#!/usr/bin/env python3

import argparse
import sys

def main():
    parser = argparse.ArgumentParser(description='Convert a hexadecimal string file to a byte string')
    parser.add_argument('input_file', nargs='?', type=argparse.FileType('r'), default=sys.stdin, help='Input file containing hexadecimal string (default: standard input)')
    parser.add_argument('output_file', nargs='?', type=argparse.FileType('wb'), default=sys.stdout.buffer, help='Output file to store the byte string (default: standard output)')

    args = parser.parse_args()

    hex_string = args.input_file.read().strip()
    
    # Remove optional '0x' prefix
    if hex_string.lower().startswith('0x'):
        hex_string = hex_string[2:]

    byte_string = bytes.fromhex(hex_string)

    args.output_file.write(byte_string)

if __name__ == '__main__':
    main()