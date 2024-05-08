#!/bin/sh -x

set -x

/usr/sbin/sshd

ls -l /
ls -l /git*

/git-web &> /logs/app.log
