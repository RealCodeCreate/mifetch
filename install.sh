#!/bin/bash
USER="RealCodeCreate"
REPO="mifetch"
SCRIPT_NAME="mifetch"
RAW_URL="https://raw.githubusercontent.com/$USER/$REPO/main/$SCRIPT_NAME"
echo -e "\e[32m[*] Начинаю установку $SCRIPT_NAME...\e[0m"
if curl -sSL "$RAW_URL" -o "$SCRIPT_NAME"; then
    echo "[+] Скрипт успешно скачан."
else
    echo "[-] Ошибка при скачивании скрипта."
    exit 1
fi
chmod +x "$SCRIPT_NAME"
sudo mv "$SCRIPT_NAME" /usr/local/bin/
if [ $? -eq 0 ]; then
    echo -e "\e[32m[!] Готово! Теперь вы можете просто ввести 'mifetch' в терминале.\e[0m"
else
    echo "[-] Ошибка при переносе файла. Проверьте права sudo."
    exit 1
fi
