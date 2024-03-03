#!/bin/bash

while ! mysqladmin ping -h"$DB_HOST" --silent; do
    echo "mysql not ready"
    sleep 1
done

echo "mysql ready"
echo "running migrations"
~/.cargo/bin/diesel migration run
echo "migrations finished"