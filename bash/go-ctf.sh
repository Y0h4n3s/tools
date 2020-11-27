#!/bin/bash

basedir=~/Desktop/Boxes


function makedirs {
  `mkdir $basedir/$1/$2 && cd $basedir/$1/$2 && mkdir nmap ffuf burp`
  basedir=$basedir/$1/$2
}

while getopts ":h:t:v:i:" o; do
  case "${o}" in
      h) makedirs HTB $OPTARG;;
      t) makedirs TryHackMe $OPTARG;;
      v) makedirs Vulnhub $OPTARG;;
      i) echo "Sudo Password:"
         sudo nmap -sSVC -p 0-65535 -v -oA $basedir/nmap/all-ports $OPTARG
  esac
done
