#!/bin/bash

ip netns add peer1
ip netns add peer2

ip link add dev veth0 type veth peer name veth1
ip link add dev veth2 type veth peer name veth3
ip link add dev veth4 type veth peer name veth5

sudo ip link set dev veth0 up
sudo ip link set dev veth1 up
sudo ip link set dev veth2 up
sudo ip link set dev veth3 up
sudo ip link set dev veth4 up
sudo ip link set dev veth5 up
sudo ip link set dev pkt1 up
sudo ip link set dev pkt2 up
sudo ip link set dev pkt3 up

ip link set veth1 netns peer1
ip link set veth3 netns peer2

ip netns exec peer1 ip link set dev lo up
ip netns exec peer1 ip link set dev veth1 up
ip netns exec peer1 ip addr add 192.168.1.1/24 dev veth1
ip netns exec peer1 ip link set dev veth1 address 00:01:02:03:04:01

ip netns exec peer2 ip link set dev lo up
ip netns exec peer2 ip link set dev veth3 up
ip netns exec peer2 ip addr add 192.168.1.2/24 dev veth3
ip netns exec peer2 ip link set dev veth3 address 00:01:02:03:04:02
