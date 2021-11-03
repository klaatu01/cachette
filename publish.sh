#!/bin/bash
help_string="requires destination. build.sh x86_64|arm64 region"
if [ -d $1 ]; then
  echo $help_string
  exit 1
fi
if [ -d $2 ]; then
  echo $help_string
  exit 1
fi


cachette_name="cachette_$1"
aws lambda publish-layer-version --layer-name $cachette_name --zip-file "fileb://extensions.zip" --region $2 --compatible-architectures $1
