// comic_editor.go
package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/fogleman/gg"
)

type ComicEditor struct {
	inputPath   string
	text        string
	x, y        int
	shape       string
	width, height int
	bubbleColor string
	borderColor string
	textColor   string
	fontSize    float64
	output      string
}

func NewComicEditor(input, text string, x, y int, shape string, w, h int,
	bubble, border, textColor string, fontSize float64, output string) *ComicEditor {
	return &ComicEditor{
		inputPath:   input,
		text:        text,
		x:           x,
		y:           y,
		shape:       shape,
		width:       w,
		height:      h,
		bubbleColor: bubble,
		borderColor: border,
		textColor:   textColor,
		fontSize:    fontSize,
		output:      output,
	}
}

func (e *ComicEditor) Run() error {
	// Load image
	dc, err := gg.LoadImage(e.inputPath)
	if err != nil {
		return err
	}
	ctx := gg.NewContextForImage(dc)

	// Draw bubble
	x, y := float64(e.x), float64(e.y)
	w, h := float64(e.width), float64(e.height)
	ctx.SetHexColor(e.bubbleColor)
	ctx.SetHexColor(e.borderColor)

	switch e.shape {
	case "circle":
		ctx.DrawEllipse(x+w/2, y+h/2, w/2, h/2)
		ctx.FillPreserve()
		ctx.SetLineWidth(2)
		ctx.Stroke()
	case "square":
		ctx.DrawRectangle(x, y, w, h)
		ctx.FillPreserve()
		ctx.SetLineWidth(2)
		ctx.Stroke()
	case "cloud":
		ctx.DrawEllipse(x+w/3, y+h/3, w/3, h/3)
		ctx.FillPreserve()
		ctx.SetLineWidth(2)
		ctx.Stroke()
		ctx.DrawEllipse(x+w*2/3, y+h/3, w/3, h/3)
		ctx.FillPreserve()
		ctx.SetLineWidth(2)
		ctx.Stroke()
		ctx.DrawEllipse(x+w/2, y+h*2/3, w/3, h/3)
		ctx.FillPreserve()
		ctx.SetLineWidth(2)
		ctx.Stroke()
	}

	// Draw text
	ctx.SetHexColor(e.textColor)
	if err := ctx.LoadFontFace("arial.ttf", e.fontSize); err != nil {
		ctx.LoadFontFace("/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf", e.fontSize)
	}
	ctx.DrawStringAnchored(e.text, x+w/2, y+h/2, 0.5, 0.5)

	// Save
	return ctx.SavePNG(e.output)
}

func main() {
	var (
		input       string
		text        string
		x, y        int
		shape       string
		width, height int
		bubbleColor string
		borderColor string
		textColor   string
		fontSize    float64
		output      string
	)
	flag.StringVar(&input, "input", "", "Input image file")
	flag.StringVar(&text, "text", "", "Text in the bubble")
	flag.IntVar(&x, "x", 50, "X coordinate")
	flag.IntVar(&y, "y", 50, "Y coordinate")
	flag.StringVar(&shape, "shape", "circle", "circle, square, cloud")
	flag.IntVar(&width, "width", 200, "Bubble width")
	flag.IntVar(&height, "height", 100, "Bubble height")
	flag.StringVar(&bubbleColor, "bubble-color", "#FFFFFF", "Bubble color")
	flag.StringVar(&borderColor, "border-color", "#000000", "Border color")
	flag.StringVar(&textColor, "text-color", "#000000", "Text color")
	flag.Float64Var(&fontSize, "font-size", 16, "Font size")
	flag.StringVar(&output, "output", "comic.png", "Output file")
	flag.Parse()

	if input == "" || text == "" {
		fmt.Println("Error: --input and --text are required")
		os.Exit(1)
	}

	editor := NewComicEditor(input, text, x, y, shape, width, height,
		bubbleColor, borderColor, textColor, fontSize, output)
	if err := editor.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("Comic saved to %s\n", output)
}
