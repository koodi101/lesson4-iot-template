#!/bin/sh

set -eux

# Add raw networking kernel capabilities to node executable
sudo setcap cap_net_raw+eip /usr/bin/node
