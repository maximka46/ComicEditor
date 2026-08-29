// ComicEditor.java
import com.beust.jcommander.JCommander;
import com.beust.jcommander.Parameter;

import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.File;
import javax.imageio.ImageIO;

public class ComicEditor {
    @Parameter(names = "--input", required = true)
    private String input;

    @Parameter(names = "--text", required = true)
    private String text;

    @Parameter(names = "--x")
    private int x = 50;

    @Parameter(names = "--y")
    private int y = 50;

    @Parameter(names = "--shape")
    private String shape = "circle";

    @Parameter(names = "--width")
    private int width = 200;

    @Parameter(names = "--height")
    private int height = 100;

    @Parameter(names = "--bubble-color")
    private String bubbleColor = "#FFFFFF";

    @Parameter(names = "--border-color")
    private String borderColor = "#000000";

    @Parameter(names = "--text-color")
    private String textColor = "#000000";

    @Parameter(names = "--font-size")
    private int fontSize = 16;

    @Parameter(names = "--output")
    private String output = "comic.png";

    private Color hexToColor(String hex) {
        return Color.decode(hex);
    }

    public void run() throws Exception {
        BufferedImage img = ImageIO.read(new File(input));
        Graphics2D g = img.createGraphics();
        g.setRenderingHint(RenderingHints.KEY_ANTIALIASING, RenderingHints.VALUE_ANTIALIAS_ON);

        // Draw bubble
        Color fill = hexToColor(bubbleColor);
        Color border = hexToColor(borderColor);
        g.setColor(fill);
        if (shape.equals("circle")) {
            g.fillOval(x, y, width, height);
        } else if (shape.equals("square")) {
            g.fillRect(x, y, width, height);
        } else if (shape.equals("cloud")) {
            g.fillOval(x + width/3, y + height/3, width/2, height/2);
            g.fillOval(x + width/6, y + height/4, width/2, height/2);
            g.fillOval(x + width/2, y + height/4, width/2, height/2);
        }
        g.setColor(border);
        g.setStroke(new BasicStroke(2));
        if (shape.equals("circle")) {
            g.drawOval(x, y, width, height);
        } else if (shape.equals("square")) {
            g.drawRect(x, y, width, height);
        } else if (shape.equals("cloud")) {
            g.drawOval(x + width/3, y + height/3, width/2, height/2);
            g.drawOval(x + width/6, y + height/4, width/2, height/2);
            g.drawOval(x + width/2, y + height/4, width/2, height/2);
        }

        // Draw text
        g.setColor(hexToColor(textColor));
        g.setFont(new Font("Arial", Font.PLAIN, fontSize));
        FontMetrics fm = g.getFontMetrics();
        int textW = fm.stringWidth(text);
        int textH = fm.getHeight();
        int tx = x + (width - textW) / 2;
        int ty = y + (height - textH) / 2 + fm.getAscent();
        g.drawString(text, tx, ty);

        g.dispose();
        ImageIO.write(img, "png", new File(output));
        System.out.println("Comic saved to " + output);
    }

    public static void main(String[] args) throws Exception {
        ComicEditor editor = new ComicEditor();
        JCommander.newBuilder().addObject(editor).build().parse(args);
        editor.run();
    }
}
