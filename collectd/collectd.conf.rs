
# Config file for collectd(1).
# Please read collectd.conf(5) for a list of options.
# http://collectd.org/
#

##############################################################################
# Global                                                                     #
#----------------------------------------------------------------------------#
# Global settings for the daemon.                                            #
##############################################################################

Hostname     %RS_INSTANCE_UUID%
FQDNLookup   false
BaseDir     "/opt/collectd/lib/collectd"
PIDFile     "/var/run/collectd.pid"
PluginDir   "/opt/collectd/lib/collectd"
TypesDB     "/opt/collectd/share/collectd/types.db"
Interval 20
ReadThreads 5

LoadPlugin "write_http"
<Plugin "write_http">
  URL "http://127.0.0.1:%RS_RLL_PORT%/rll/tss/collectdv5"
</Plugin>

LoadPlugin "cpu"
LoadPlugin "memory"
LoadPlugin "load"
LoadPlugin "users"
LoadPlugin "disk"
LoadPlugin "interface"
LoadPlugin "df"

<Plugin "interface">
  Interface "eth0"
</Plugin>

<Plugin "df">
  ReportReserved false
  FSType "proc"
  FSType "sysfs"
  FSType "fusectl"
  FSType "debugfs"
  FSType "securityfs"
  FSType "devtmpfs"
  FSType "devpts"
  FSType "tmpfs"
  IgnoreSelected true
</Plugin>


