#!/bin/bash

# Replace the name of the host
sed -i "s/%RS_INSTANCE_UUID%/$RS_INSTANCE_UUID/g" /opt/collectd/etc/collectd.conf
sed -i "s/%RS_RLL_PORT%/$RS_RLL_PORT/g" /opt/collectd/etc/collectd.conf

# Start collectd daemon
/opt/collectd/sbin/collectd 

# dummy tail to keep the container running
tail -f /dev/null
