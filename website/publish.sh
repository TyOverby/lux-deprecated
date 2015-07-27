#!/bin/bash

cd out

git add -A &&
git commit -m "update" &&
git push origin gh-pages

cd -

