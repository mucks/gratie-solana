#!/bin/sh

anchor build && anchor deploy

yes | cp -r target/idl ../gratie-frontend/src/lib/
yes | cp -r target/types ../gratie-frontend/src/lib/