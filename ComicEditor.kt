// ComicEditor.kt
import com.beust.jcommander.JCommander
import com.beust.jcommander.Parameter
import java.awt.*
import java.awt.image.BufferedImage
import java.io.File
import javax.imageio.ImageIO

class ComicEditor {
    @Parameter(names = ["--input"], required = true)
    private lateinit var input: String

    @Parameter(names = ["--text"], required = true)
    private lateinit var text: String

    @Parameter(names = ["--x"])
    private var x: Int = 50

    @Parameter(names = ["--y"])
    private var y: Int = 50

    @Parameter(names = ["--shape"])
    private var shape: String = "circle"

    @Parameter(names = ["--width"])
    private var width: Int = 200

    @Parameter(names = ["--height"])
    private var height: Int = 100

    @Parameter(names = ["--bubble-color"])
    private var bubbleColor: String = "#FFFFFF"

    @Parameter(names = ["--border-color"])
    private var borderColor: String = "#000000"

    @Parameter(names = ["--text-color"])
    private var textColor: String = "#000000"

    @Parameter(names = ["--font-size"])
    private var fontSize: Int = 16

    @Parameter(names = ["--output"])
    private var output: String = "comic.png"

    private fun hexToColor(hex: String): Color {
        return Color.decode(hex)
    }

    fun run() {
        val img = ImageIO.read(File(input))
        val g = img.createGraphics()
        g.setRenderingHint(RenderingHints.KEY_ANTIALIASING, RenderingHints.VALUE_ANTIALIAS_ON)

        val fill = hexToColor(bubbleColor)
        val border = hexToColor(borderColor)

        if (shape == "circle") {
            g.color = fill
            g.fillOval(x, y, width, height)
            g.color = border
            g.stroke = BasicStroke(2f)
            g.drawOval(x, y, width, height)
        } else if (shape == "square") {
            g.color = fill
            g.fillRect(x, y, width, height)
            g.color = border
            g.stroke = BasicStroke(2f)
            g.drawRect(x, y, width, height)
        } else if (shape == "cloud") {
            g.color = fill
            g.fillOval(x + width / 3, y + height / 3, width / 2, height / 2)
            g.fillOval(x + width / 6, y + height / 4, width / 2, height / 2)
            g.fillOval(x + width / 2, y + height / 4, width / 2, height / 2)
            g.color = border
            g.stroke = BasicStroke(2f)
            g.drawOval(x + width / 3, y + height / 3, width / 2, height / 2)
            g.drawOval(x + width / 6, y + height / 4, width / 2, height / 2)
            g.drawOval(x + width / 2, y + height / 4, width / 2, height / 2)
        }

        g.color = hexToColor(textColor)
        g.font = Font("Arial", Font.PLAIN, fontSize)
        val fm = g.fontMetrics
        val textW = fm.stringWidth(text)
        val textH = fm.height
        val tx = x + (width - textW) / 2
        val ty = y + (height - textH) / 2 + fm.ascent
        g.drawString(text, tx, ty)

        g.dispose()
        ImageIO.write(img, "png", File(output))
        println("Comic saved to $output")
    }
}

fun main(args: Array<String>) {
    val editor = ComicEditor()
    JCommander.newBuilder().addObject(editor).build().parse(*args)
    editor.run()
}
