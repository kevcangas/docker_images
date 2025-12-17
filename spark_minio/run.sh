#!/bin/bash

ssh-keygen -R [localhost]:2222

curr_dir=$(pwd)

"C:\Program Files\Docker\Docker\Docker Desktop.exe" 
cd $curr_dir

echo "Starting Docker Engine..."
sleep 10

echo "Starting Container..."
echo "Paste this in other terminal: ssh root@localhost -p 2222"
echo "Password: root123"

docker compose up

cd $curr_dir

taskkill //IM 'Docker*' //F
taskkill //IM 'com.docker*' //F

sleep 3

wsl -t Docker-Desktop

echo 'Docker Terminated'