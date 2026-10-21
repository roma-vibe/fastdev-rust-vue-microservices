#!/bin/sh
# Runs every service binary of the image (/app/bin) in one container. When one of them exits the
# container stops (Docker restarts it per the Compose restart policy); SIGTERM stops them all.
set -eu

pids=""
for binary in /app/bin/*; do
  "$binary" &
  pids="$pids $!"
done

stop() {
  # shellcheck disable=SC2086
  kill -TERM $pids 2>/dev/null || true
  wait
  exit 0
}
trap stop TERM INT

# Poll the children: stop the whole container as soon as one of them is gone.
while :; do
  for pid in $pids; do
    if ! kill -0 "$pid" 2>/dev/null; then
      echo "a service exited (pid $pid); stopping the container" >&2
      stop
    fi
  done
  sleep 2
done
