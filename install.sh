#!/bin/bash
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m'
echo -e "${BLUE}==>${NC} Начинаю установку mifetch..."
if ! command -v cargo &> /dev/null; then
    echo -e "${BLUE}==>${NC} Rust не найден. Устанавливаю через rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo -e "${GREEN}==>${NC} Rust уже установлен."
fi
echo -e "${BLUE}==>${NC} Клонирую репозиторий..."
rm -rf /tmp/mifetch
git clone https://github.com/RealCodeCreate/mifetch /tmp/mifetch
cd /tmp/mifetch
echo -e "${BLUE}==>${NC} Собираю и устанавливаю через Cargo..."
cargo install --path .
if [[ $SHELL == *"fish"* ]]; then
    if ! echo $PATH | grep -q "$HOME/.cargo/bin"; then
        echo -e "${BLUE}==>${NC} Добавляю путь Cargo в настройки fish..."
        fish -c "set -U fish_user_paths \$HOME/.cargo/bin \$fish_user_paths"
    fi
fi
echo -e "${GREEN}✅ Готово! Напиши 'mifetch', чтобы запустить.${NC}"
