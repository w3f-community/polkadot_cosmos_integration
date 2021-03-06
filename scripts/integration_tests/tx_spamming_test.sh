#!/usr/bin/env bash
trap "exit" INT TERM ERR
trap "kill 0" EXIT

## import
source ./testing_setup/test_utils.sh
source ./testing_setup/basic_setup.sh

## Run cosmos and substrate nodes
start_all
sleep 20s

## sync bug test

for i in {1..200}
do
    nscli tx nameservice buy-name jack.id 5nametoken --from jack --chain-id namechain -y
done

for i in {1..200}
do
    nscli tx nameservice set-name jack.id hello_world --from jack --chain-id namechain -y
done
sleep 20s

value=$(nscli query nameservice resolve jack.id)
assert_eq "$value" "value: hello_world"

test_passed "tx spamming test"

