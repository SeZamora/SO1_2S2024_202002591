#!/bin/bash

for i in {1..10}
do
        random=$(tr -dc A-Za-z0-9 </dev/urandom | head -c 13)
        docker run -d --name "$random" alpine
done

