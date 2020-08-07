#!/bin/bash

ip netns del peer1
ip netns del peer2

ip link del dev veth0
ip link del dev veth2
ip link del dev veth4
