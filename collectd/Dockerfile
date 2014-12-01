FROM centos:centos6

RUN yum install -y gcc-c++ cmake wget tar

RUN wget https://collectd.org/files/collectd-5.4.1.tar.gz && tar -xzvf collectd-5.4.1.tar.gz 

RUN cd collectd-5.4.1 && grep -rl /proc/ . | xargs sed -i "s/\/proc\//\/proc_host\//g" && ./configure && make all install

COPY collectd.conf /opt/collectd/etc/collectd.conf

COPY start.sh /start.sh

RUN chmod +x /start.sh

ENV HOST_HOSTNAME to_be_replace
ENV LOGSTASH_IP 127.0.0.1

CMD /start.sh

