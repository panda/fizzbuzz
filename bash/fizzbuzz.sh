#!/bin/bash

for num in {1..100} ; do
  { (( num % 5 == 0 )) && (( num % 3 == 0 )) && echo "fizzbuzz"; } ||
  { (( num % 5 == 0 )) && echo "buzz"; } ||
  { (( num % 3 == 0 )) && echo "fizz"; } ||
  echo $num
done
