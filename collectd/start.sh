#!/bin/bash

# Replace the name of the host
sed -i "s/%HOST_HOSTNAME%/$HOST_HOSTNAME/g" /opt/collectd/etc/collectd.conf
sed -i "s/%LOGSTASH_IP%/$LOGSTASH_IP/g" /opt/collectd/etc/collectd.conf

# Start collectd daemon
/opt/collectd/sbin/collectd

# dummy tail to keep the container running
tail -f /dev/null
