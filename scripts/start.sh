#!/bin/bash

if [ -z $CONFIG_FILE_URL ]; then
  export MINIO_ENDPOINT=$CONFIG_FILE_URL
  mod-downloader
fi

/discord-spam-reporter
