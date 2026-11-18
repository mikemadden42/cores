# cores
[![build](https://github.com/mikemadden42/cores/actions/workflows/build.yml/badge.svg)](https://github.com/mikemadden42/cores/actions/workflows/build.yml)

List CPU cores

The logical count is the number of CPUs available to the process, so it
respects CPU affinity and cgroup quotas (e.g. inside a container). The physical
count reports the machine's physical cores.

MSRV: 1.63
