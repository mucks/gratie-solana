#!/bin/sh

anchor build && anchor deploy && anchor run test

yes | cp -r target/idl ../gratie-frontend/src/gratie_solana_contract/
yes | cp -r target/types ../gratie-frontend/src/gratie_solana_contract/