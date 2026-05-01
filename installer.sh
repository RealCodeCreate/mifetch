#!/bin/bash
set -e

echo "🔨 Сборка mifetch в режиме release..."
cargo build --release

echo "🚀 Установка в /usr/local/bin..."
sudo cp target/release/mifetch /usr/local/bin/

echo "✅ Готово! Теперь введи 'mifetch' в терминале."
