#!/bin/bash
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'
echo -e "${BLUE}==>${NC} Начинаю установку mifetch..."
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Ошибка: Rust и Cargo не найдены.${NC}"
    echo -e "Для сборки mifetch необходимо установить Rust."
    echo -e "Вы можете сделать это через ваш пакетный менеджер"
    exit 1
else
    echo -e "${GREEN}==>${NC} Rust найден (версия: $(cargo --version | cut -d' ' -f2))."
fi
echo -e "${BLUE}==>${NC} Клонирую репозиторий..."
rm -rf /tmp/mifetch
git clone https://github.com/RealCodeCreate/mifetch /tmp/mifetch
cd /tmp/mifetch || exit 1
echo -e "${BLUE}==>${NC} Сборка и установка..."
if cargo install --path .; then
    echo -e "${GREEN}==>${NC} Сборка завершена успешно."
else
    echo -e "${RED}❌ Ошибка при сборке проекта.${NC}"
    exit 1
fi
if [[ $SHELL == *"fish"* ]]; then
    if ! echo $PATH | grep -q "$HOME/.cargo/bin"; then
        echo -e "${BLUE}==>${NC} Добавляю путь Cargo в настройки fish..."
        fish -c "set -U fish_user_paths \$HOME/.cargo/bin \$fish_user_paths"
    fi
fi
echo -e "${GREEN}✅ Готово! Теперь можно запустить: mifetch${NC}"
