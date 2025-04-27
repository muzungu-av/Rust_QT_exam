#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLUGIN_SRC_DIR="$SCRIPT_DIR/plugins"

DEST_DIR="$SCRIPT_DIR/~/run-app"
PLUGIN_DEST_DIR="$DEST_DIR/plugins"


# Создать папку назначения, если её нет
mkdir -p "$DEST_DIR"
mkdir -p "$PLUGIN_DEST_DIR"


# Перейти в папку
cd "$SCRIPT_DIR"
# Собрать проект
cargo build --release
# Найти бинарник (исполняемый файл)
# Ищем исполняемые файлы в target/release (исключая .d, .so, .rlib и т.д.)
MAIN_BINARY=$(find ./target/release -maxdepth 1 -type f -executable ! -name "*.so" | head -n 1)

if [ -n "$MAIN_BINARY" ]; then
  echo "Копирование бинарника $MAIN_BINARY в $DEST_DIR"
  cp "$MAIN_BINARY" "$DEST_DIR"
else
  echo "Бинарник не найден в $SCRIPT_DIR"
fi


# Найти все директории в папке плагинов
for dir in "$PLUGIN_SRC_DIR"/*; do
  if [ -d "$dir" ]; then
    echo "Сборка в папке: $dir"

    # Перейти в папку
    cd "$dir" || continue

    # Собрать проект
    cargo build --release

    # Найти .so файл в target/release
    SO_FILE=$(find target/release -maxdepth 1 -name "*.so" | head -n 1)

    if [ -n "$SO_FILE" ]; then
      echo "Копирование файла $SO_FILE в $PLUGIN_DEST_DIR"
      cp "$SO_FILE" "$PLUGIN_DEST_DIR"
    else
      echo "Файл .so не найден в $dir"
    fi
  fi
done

