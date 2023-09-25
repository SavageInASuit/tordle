#!/bin/bash
if sudo service tordle-server status >/dev/null 2>&1; then
    sudo service tordle-server stop
fi
