// ComicEditor.cs
using System;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.Drawing.Imaging;
using System.IO;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace ComicEditor
{
    class Program
    {
        static void Main(string[] args)
        {
            var opts = ParseArgs(args);
            if (opts.Input == null || opts.Text == null)
            {
                Console.Error.WriteLine("Error: --input and --text are required");
                return;
            }
            var editor = new ComicEditor(opts);
            editor.Run();
        }

        static Options ParseArgs(string[] args)
        {
            var opts = new Options();
            for (int i = 0; i < args.Length; i++)
            {
                switch (args[i])
                {
                    case "--input": opts.Input = args[++i]; break;
                    case "--text": opts.Text = args[++i]; break;
                    case "--x": opts.X = int.Parse(args[++i]); break;
                    case "--y": opts.Y = int.Parse(args[++i]); break;
                    case "--shape": opts.Shape = args[++i]; break;
                    case "--width": opts.Width = int.Parse(args[++i]); break;
                    case "--height": opts.Height = int.Parse(args[++i]); break;
                    case "--bubble-color": opts.BubbleColor = args[++i]; break;
                    case "--border-color": opts.BorderColor = args[++i]; break;
                    case "--text-color": opts.TextColor = args[++i]; break;
                    case "--font-size": opts.FontSize = int.Parse(args[++i]); break;
                    case "--output": opts.Output = args[++i]; break;
                }
            }
            return opts;
        }

        class Options
        {
            public string Input { get; set; }
            public string Text { get; set; }
            public int X { get; set; } = 50;
            public int Y { get; set; } = 50;
            public string Shape { get; set; } = "circle";
            public int Width { get; set; } = 200;
            public int Height { get; set; } = 100;
            public string BubbleColor { get; set; } = "#FFFFFF";
            public string BorderColor { get; set; } = "#000000";
            public string TextColor { get; set; } = "#000000";
            public int FontSize { get; set; } = 16;
            public string Output { get; set; } = "comic.png";
        }

        class ComicEditor
        {
            private readonly Options opts;

            public ComicEditor(Options opts)
            {
                this.opts = opts;
            }

            private Color HexToColor(string hex)
            {
                return ColorTranslator.FromHtml(hex);
            }

            public void Run()
            {
                using (var img = Image.FromFile(opts.Input))
                using (var g = Graphics.FromImage(img))
                {
                    g.SmoothingMode = SmoothingMode.AntiAlias;

                    var fill = new SolidBrush(HexToColor(opts.BubbleColor));
                    var border = new Pen(HexToColor(opts.BorderColor), 2);

                    int x = opts.X, y = opts.Y, w = opts.Width, h = opts.Height;

                    // Draw bubble
                    if (opts.Shape == "circle")
                    {
                        g.FillEllipse(fill, x, y, w, h);
                        g.DrawEllipse(border, x, y, w, h);
                    }
                    else if (opts.Shape == "square")
                    {
                        g.FillRectangle(fill, x, y, w, h);
                        g.DrawRectangle(border, x, y, w, h);
                    }
                    else if (opts.Shape == "cloud")
                    {
                        g.FillEllipse(fill, x + w/3, y + h/3, w/2, h/2);
                        g.FillEllipse(fill, x + w/6, y + h/4, w/2, h/2);
                        g.FillEllipse(fill, x + w/2, y + h/4, w/2, h/2);
                        g.DrawEllipse(border, x + w/3, y + h/3, w/2, h/2);
                        g.DrawEllipse(border, x + w/6, y + h/4, w/2, h/2);
                        g.DrawEllipse(border, x + w/2, y + h/4, w/2, h/2);
                    }

                    // Draw text
                    using (var font = new Font("Arial", opts.FontSize))
                    {
                        var textColor = new SolidBrush(HexToColor(opts.TextColor));
                        var size = g.MeasureString(opts.Text, font);
                        float tx = x + (w - size.Width) / 2;
                        float ty = y + (h - size.Height) / 2;
                        g.DrawString(opts.Text, font, textColor, tx, ty);
                    }

                    img.Save(opts.Output, ImageFormat.Png);
                    Console.WriteLine($"Comic saved to {opts.Output}");
                }
            }
        }
    }
}
