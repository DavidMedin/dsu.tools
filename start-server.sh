#!/bin/env bash
# LINUX ONLY
# This script is used to build and run the DSU Tools project.

cd database
if [ ! -f "dsutools.sqlite" ]; then
    echo 'Generating Database'
    ./generate.sh
fi
cd ..

echo 'Building Frontend'
cd dsutools-frontend
npm run build
cd ..

echo 'Building & Running Backend'
cd dsutools-backend
cargo run
