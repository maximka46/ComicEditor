
```python
#!/usr/bin/env python3
# comic_editor.py
import argparse
import sys
from PIL import Image, ImageDraw, ImageFont, ImageFilter
from PIL.Image import Resampling

class ComicEditor:
    def __init__(self, input_path, text, x, y, shape='circle', width=200, height=100,
                 bubble_color='#FFFFFF', border_color='#000000', text_color='#000000',
                 font_size=16, output='comic.png'):
        self.input_path = input_path
        self.text = text
        self.x = x
        self.y = y
        self.shape = shape
        self.width = width
        self.height = height
        self.bubble_color = bubble_color
        self.border_color = border_color
        self.text_color = text_color
        self.font_size = font_size
        self.output = output

    def _draw_bubble(self, draw):
        x, y = self.x, self.y
        w, h = self.width, self.height
        if self.shape == 'circle':
            draw.ellipse([x, y, x+w, y+h], fill=self.bubble_color, outline=self.border_color, width=2)
        elif self.shape == 'square':
            draw.rectangle([x, y, x+w, y+h], fill=self.bubble_color, outline=self.border_color, width=2)
        elif self.shape == 'cloud':
            # Простая облачная форма: несколько перекрывающихся кругов
            draw.ellipse([x, y, x+w, y+h//2], fill=self.bubble_color, outline=self.border_color, width=2)
            draw.ellipse([x+w//4, y+h//4, x+w//4+w, y+h//4+h], fill=self.bubble_color, outline=self.border_color, width=2)
            draw.ellipse([x-w//4, y+h//4, x-w//4+w, y+h//4+h], fill=self.bubble_color, outline=self.border_color, width=2)
        return draw

    def run(self):
        try:
            img = Image.open(self.input_path).convert('RGB')
        except Exception as e:
            print(f"Error loading image: {e}", file=sys.stderr)
            sys.exit(1)

        draw = ImageDraw.Draw(img)
        self._draw_bubble(draw)

        try:
            font = ImageFont.truetype("arial.ttf", self.font_size)
        except:
            font = ImageFont.load_default()

        # Центрируем текст в облачке
        bbox = draw.textbbox((0, 0), self.text, font=font)
        text_w = bbox[2] - bbox[0]
        text_h = bbox[3] - bbox[1]
        tx = self.x + (self.width - text_w) // 2
        ty = self.y + (self.height - text_h) // 2
        draw.text((tx, ty), self.text, fill=self.text_color, font=font)

        img.save(self.output)
        print(f"Comic saved to {self.output}")

def main():
    parser = argparse.ArgumentParser(description="Comic Editor (speech bubbles)")
    parser.add_argument("--input", required=True, help="Input image file")
    parser.add_argument("--text", required=True, help="Text in the bubble")
    parser.add_argument("--x", type=int, default=50, help="X coordinate")
    parser.add_argument("--y", type=int, default=50, help="Y coordinate")
    parser.add_argument("--shape", choices=["circle", "square", "cloud"], default="circle")
    parser.add_argument("--width", type=int, default=200)
    parser.add_argument("--height", type=int, default=100)
    parser.add_argument("--bubble-color", default="#FFFFFF")
    parser.add_argument("--border-color", default="#000000")
    parser.add_argument("--text-color", default="#000000")
    parser.add_argument("--font-size", type=int, default=16)
    parser.add_argument("--output", default="comic.png")
    args = parser.parse_args()

    editor = ComicEditor(
        input_path=args.input,
        text=args.text,
        x=args.x,
        y=args.y,
        shape=args.shape,
        width=args.width,
        height=args.height,
        bubble_color=args.bubble_color,
        border_color=args.border_color,
        text_color=args.text_color,
        font_size=args.font_size,
        output=args.output
    )
    editor.run()

if __name__ == "__main__":
    main()
