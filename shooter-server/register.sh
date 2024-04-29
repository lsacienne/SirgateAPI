#!/bin/bash

# Assign the arguments to variables
username=$1
email=$2
password=$3

# Define the URL
url=http://webapp:8080/dgs/login

# Send the POST request
curl $url -X POST -H "Content-Type: application/json" -d "{\"username\": \"$username\", \"email\": \"$email\", \"password\": \"$password\"}"