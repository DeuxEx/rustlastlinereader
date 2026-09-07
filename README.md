I needed to learn Rust coding because I migrated to 100% linux and C# with dotnet isnt available on this platform.

So I started to make a replica of my robust C# code but very much improved.

All developement are made in Void Linux and I´m using Kate and Helix to do the code.

The code runs 100% in shell but will be sending short data via the dbus service to a GUI (relm4) applikations, and it will
be calculated in the GUI app and shows in the window there so the shell runner will not be interfered with calculations.

Later on the project maybe will be crosscompiled to windows, but primarily focus is Linux.

to use the executable:
git clone this path and chmod +x the targetfile

<br><p>



The principle:

The principles here — continuously tailing a stream of log lines, matching patterns in real time, maintaining a state machine, and triggering actions based on events—are the exact same architecture used in professional log management and monitoring.

In enterprise environments, what you are doing goes by a few different names depending on the scale:

Log Parsing & Ingestion (e.g., Logstash, Fluentd, Vector): 
These tools continuously read log files (or network streams from routers, servers, and firewalls), parse strings via keywords or regex, and extract metrics (just like your loot, damage, and shots).

SIEM / Log Management Systems (e.g., Splunk, Elastic Stack/ELK, Graylog):
These systems ingest log data centrally and run analysis rules. If they detect keywords like "Failed password", "Critical Error", or patterns like "5 failed logins in 10 seconds" (comparable to your "Loot received -> calculate and reset for next mob"), the system triggers an alert or automated action.

Stream Processing (e.g., Apache Kafka, Rx/Reactive Extensions):
Treating data as an infinite stream of real-time events rather than static files.

Building this in Rust makes it directly comparable to modern, high-performance log ingestion engines like Vector (by Datadog) or Fluent Bit, which are written in systems languages (Rust/C) to swallow millions of log lines per second with minimal CPU and memory overhead.

So this log parser for Entropia Universe is essentially a highly specialized, miniature SIEM engine!
The underlying data structures, string-splitting optimizations, and thread-safe state management (OnceLock/Mutex) you are using are precisely what you would encounter when working with enterprise-scale server monitoring and log analytics.
