#!/bin/bash

for topic in test-redpanda crawl_twitch_account_by_username
do
    rpk topic create $topic --brokers redpanda:29092
done