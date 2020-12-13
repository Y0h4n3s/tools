#!/usr/bin/bash

file=$1;
blob=`cat $1`
trimmedBlob=`echo "$blob" | sed "s/\s'/~>/g"` 
classes=($trimmedBlob)

index=0
for i in ${classes[*]};
do
  echo $index":" $i
  index=$(($index+1))
done
