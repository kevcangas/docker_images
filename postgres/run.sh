#!/bin/bash

ssh-keygen -R [localhost]:2222

curr_dir=$(pwd)

"C:\Program Files\Docker\Docker\Docker Desktop.exe" 
cd $curr_dir

echo "Starting Docker Engine..."
sleep 10

echo "Starting Container..."
echo "Host: local:8080"
echo "Password: Kevin123"

docker compose up

cd $curr_dir

taskkill //IM 'Docker*' //F
taskkill //IM 'com.docker*' //F

sleep 3

wsl -t Docker-Desktop

echo 'Docker Terminated'