Image Logstash
==============

Created from the docker image pblittle/docker-logstash because of the need of a custom logstash configuration file, that i can't specify on startup.

This image allow the use of collectd as statistic collector.

How to run :
docker run -d -p 9292:9292 -p 9200:9200 -p 25826:25826/udp rmessner/logstash
