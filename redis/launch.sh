#!/bin/sh
redis-server &
sleep 3
redis-cli SET ALL_DGS '{"dgs_cluster": "ALL_DGS","dgs": [] }'