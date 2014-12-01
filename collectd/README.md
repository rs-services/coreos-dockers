Image collectd
==============

Centos based image to run collectd inside au container to monitor the host.

The sources of collectd are altered due to the hard path /proc.

How to run :
docker run -d -v /proc:/proc_host:ro rmessner/collectd
