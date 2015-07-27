#!/bin/bash

rm -rf /tmp/LUX_WEB_OUT/

cp -R ./out/ /tmp/LUX_WEB_OUT &&
git checkout gh-pages &&
cd ../ &&

rm $0
rm -rf *

mv /tmp/LUX_WEB_OUT/* ./ &&
git add -A &&
git commit -m "publish" &&
git push origin gh-pages &&
yes n | git checkout master &&
cd website

rm -rf /tmp/LUX_WEB_OUT/
