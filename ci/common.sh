#!/bin/bash

fn_die() {
  echo -e "\n\033[1;31m${1}\033[0m\n" >&2
  exit "${2:-1}"
}

log() {
  # styles
  local normal=0
  local bold=1
  local shadow=2
  local italic=3

  # colors
  local black=30
  local red=31
  local green=32
  local yellow=33

  local usage="Usage: log style color \"message\"\nStyles: bold, italic, normal, light\nColors: black, red, green, yellow\nExample: log bold red \"Error: Something went wrong\""
  [ "$#" -lt 3 ] && {
    echo -e "\033[${bold};${red}m${FUNCNAME[0]} error: function requires three arguments.\n${usage}\033[0m"
    exit 1
  }
  # vars
  local style="${1}"
  local color="${2}"
  local message="${3}"
  # validate style is in bold, italic, normal, shadow
  if [[ ! "${style}" =~ ^(bold|italic|normal|shadow)$ ]]; then
    message="Error: Invalid style. Must be one of normal, bold, italic, shadow."
    echo -e "\033[${bold};${red}m${message}\033[0m"
    exit 1
  fi
  # validate color is in black, red, green
  if [[ ! "${color}" =~ ^(black|red|green|yellow)$ ]]; then
    message="Error: Invalid color. Must be one of black, red, green, yellow."
    echo -e "\033[${bold};${red}m${message}\033[0m"
    exit 1
  fi

  echo -e "\033[${!style};${!color}m${message}\033[0m"
}