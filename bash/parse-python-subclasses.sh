#!/usr/bin/bash

file=$1;
blob=`cat $1`
trimmedBlob=`echo "$blob" | sed -e 's/\<\\<class\>//g' | sed -e 's/\<enum\>//g' | sed -e 's/\<type\>//g' | tr -d '<>,'`
classes=($trimmedBlob)

index=0
for i in ${classes[*]};
do
  echo $index":" $i
  index=$(($index+1))
done
