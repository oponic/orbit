#!/usr/bin/env bash
wget -q -p /tmp/orbit/ # some link rea
cd /tmp/orbit
unzip -q ./orbit.zip ./nec/
mkdir /usr/local/orbit/narfs
mkdir /usr/local/orbit/nec
mkdir /usr/local/orbit/saves
mkdir /usr/local/orbit/scripts
cp -a ./nec/import/*.lua ./narfs/lib/
cp -a ./nec/import/example.orb ./saves
cp -a ./nec/import/nec/*.* ./scripts/gamelogic/
rm -rf ./nec/import
echo "it finished :thumbsup:"
exit
