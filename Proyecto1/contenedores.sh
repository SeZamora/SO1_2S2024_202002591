#!/bin/bash
for i in {1..10}
do
        numero=$(tr -dc 1-4 </dev/urandom | head -c 1)
		nombre=$(tr -dc A-Za-z0-9 </dev/urandom | head -c 7)
	if [ $numero -eq 1 ]; then
		docker run -d --name "altocpu_$nombre" alto_cpu
	elif [ $numero -eq 2 ]; then
        docker run -d --name "bajocpu_$nombre" bajo_cpu
	elif [ $numero -eq 3 ]; then
        docker run -d --name "altoram_$nombre" alto_ram
	else
        docker run -d --name "bajoram_$nombre" bajo_ram
	fi
done
