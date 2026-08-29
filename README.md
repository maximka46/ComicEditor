# Редактор комиксов (облачные облачка)

Многоязычное консольное приложение для добавления диалоговых облачков и текста на изображения.  
Позволяет создавать простые комиксы из фотографий или иллюстраций, добавляя облачка с репликами персонажей.

## Особенности
- Загрузка изображения (PNG, JPEG) в качестве фона.
- Добавление облачков с текстом в указанные координаты.
- Поддержка различных форм облачков: **круглое** (для мыслей), **квадратное** (для диалогов), **облачное** (для восклицаний).
- Настройка размера и цвета облачка.
- Настройка шрифта, размера и цвета текста.
- Экспорт результата в форматы PNG и PDF.
- Поддержка аргументов командной строки для автоматизации.
- Кроссплатформенность (Windows, Linux, macOS).

## Установка и запуск
Для каждого языка требуются соответствующие инструменты и зависимости.

### Запуск на разных языках

1. **Python**  
   Установка: `pip install pillow reportlab colorama`  
   Запуск: `python comic_editor.py --input scene.jpg --text "Hello!" --x 100 --y 200 --output comic.png`

2. **JavaScript (Node.js)**  
   Установка: `npm install sharp commander chalk`  
   Запуск: `node comic_editor.js --input scene.jpg --text "Hello!" --x 100 --y 200 --output comic.png`

3. **Go**  
   Установка: `go get github.com/fogleman/gg`  
   Запуск: `go run comic_editor.go --input scene.jpg --text "Hello!" --x 100 --y 200 --output comic.png`

4. **Rust**  
   Добавьте `image`, `rusttype`, `clap` в `Cargo.toml`.  
   Запуск: `cargo run -- --input scene.jpg --text "Hello!" --x 100 --y 200 --output comic.png`

5. **Java**  
   Сборка: `javac -cp gson.jar ComicEditor.java` (используется AWT).  
   Запуск: `java -cp .;gson.jar ComicEditor --input scene.jpg --text "Hello!" --x 100 --y 200 --output comic.png`

6. **C# (.NET Core)**  
   Установка: `dotnet add package SixLabors.ImageSharp`  
   Запуск: `dotnet run -- --input scene.jpg --text "Hello!" --x 100 --y 200 --output comic.png`

7. **C++ (Linux)**  
   Требуется ImageMagick++.  
   Сборка: `g++ -std=c++11 -o comic_editor comic_editor.cpp -lMagick++ -lMagickWand -lMagickCore`  
   Запуск: `./comic_editor --input scene.jpg --text "Hello!" --x 100 --y 200 --output comic.png`

8. **Kotlin (JVM)**  
   Сборка: `kotlinc -cp gson.jar ComicEditor.kt` (использует Java AWT).  
   Запуск: `kotlin -cp .;gson.jar ComicEditorKt --input scene.jpg --text "Hello!" --x 100 --y 200 --output comic.png`

## Использование

Общие аргументы командной строки (везде, где поддерживается):

- `--input <файл>` – исходное изображение (обязательно).
- `--text <текст>` – текст в облачке (обязательно).
- `--x <пиксели>` – координата X верхнего левого угла облачка (по умолчанию 50).
- `--y <пиксели>` – координата Y верхнего левого угла облачка (по умолчанию 50).
- `--shape <форма>` – форма облачка: `circle`, `square`, `cloud` (по умолчанию `circle`).
- `--width <пиксели>` – ширина облачка (по умолчанию 200).
- `--height <пиксели>` – высота облачка (по умолчанию 100).
- `--bubble-color <HEX>` – цвет облачка (по умолчанию `#FFFFFF`).
- `--border-color <HEX>` – цвет границы (по умолчанию `#000000`).
- `--text-color <HEX>` – цвет текста (по умолчанию `#000000`).
- `--font-size <число>` – размер шрифта (по умолчанию 16).
- `--output <файл>` – выходной файл (PNG или PDF, по умолчанию `comic.png`).
- `--help` – справка.

Пример (Python):
```bash
python comic_editor.py --input photo.jpg --text "I'm Batman!" --x 50 --y 30 --shape cloud --width 300 --height 150 --bubble-color "#FFFF00" --border-color "#FF0000" --output batman_comic.png
Структура репозитория
text
/
├── README.md
├── comic_editor.py
├── comic_editor.js
├── comic_editor.go
├── comic_editor.rs
├── ComicEditor.java
├── ComicEditor.cs
├── comic_editor.cpp
└── ComicEditor.kt
Лицензия
MIT
