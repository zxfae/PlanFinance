#!/bin/bash

if [ ! -d "frontend" ]; then
  echo "Le répertoire src/frontend n'existe pas."
  exit 1
fi

while inotifywait -r -e close_write,moved_to,create frontend/; do
  (
    cd frontend && npm run build:css
  )
  wasm-pack build frontend --target web
done
