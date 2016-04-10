#!/bin/bash

rev=$(git rev-parse --short HEAD)

cd stage/_book

git init
git config user.name "blackanger"
git config user.email "blackanger.z@gmail.com"
git remote add upstream "https://$GH_TOKEN@github.com/RustStudy/rust-by-example-cn.git"
git fetch upstream && git reset upstream/gh-pages

# echo "rustbyexample.com" > CNAME
cp -r ../../vendor/gitbook/* gitbook/

touch .

git add -A .

git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
