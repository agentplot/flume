#!/bin/sh
# Open one of the plugin's panes: the popup of places.
exec "${HERDR_BIN_PATH:-herdr}" plugin pane open --plugin agentplot.flume --entrypoint "$1"
