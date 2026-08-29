#!/usr/bin/env node
// comic_editor.js
const { program } = require('commander');
const sharp = require('sharp');
const fs = require('fs');
const chalk = require('chalk');

class ComicEditor {
    constructor(options) {
        this.inputPath = options.input;
        this.text = options.text;
        this.x = options.x || 50;
        this.y = options.y || 50;
        this.shape = options.shape || 'circle';
        this.width = options.width || 200;
        this.height = options.height || 100;
        this.bubbleColor = options.bubbleColor || '#FFFFFF';
        this.borderColor = options.borderColor || '#000000';
        this.textColor = options.textColor || '#000000';
        this.fontSize = options.fontSize || 16;
        this.output = options.output || 'comic.png';
    }

    async run() {
        try {
            // Создаём SVG для облачка
            let svg = '';
            const x = this.x, y = this.y, w = this.width, h = this.height;
            const shape = this.shape;
            const bc = this.bubbleColor;
            const br = this.borderColor;

            if (shape === 'circle') {
                svg = `<ellipse cx="${x + w/2}" cy="${y + h/2}" rx="${w/2}" ry="${h/2}" fill="${bc}" stroke="${br}" stroke-width="2"/>`;
            } else if (shape === 'square') {
                svg = `<rect x="${x}" y="${y}" width="${w}" height="${h}" fill="${bc}" stroke="${br}" stroke-width="2"/>`;
            } else if (shape === 'cloud') {
                svg = `<circle cx="${x + w/3}" cy="${y + h/3}" r="${w/3}" fill="${bc}" stroke="${br}" stroke-width="2"/>
                       <circle cx="${x + w*2/3}" cy="${y + h/3}" r="${w/3}" fill="${bc}" stroke="${br}" stroke-width="2"/>
                       <circle cx="${x + w/2}" cy="${y + h*2/3}" r="${w/3}" fill="${bc}" stroke="${br}" stroke-width="2"/>`;
            }

            // Текст
            const textSvg = `<text x="${x + w/2}" y="${y + h/2}" text-anchor="middle" dominant-baseline="central"
                             font-family="Arial" font-size="${this.fontSize}" fill="${this.textColor}">${this.text}</text>`;
            const fullSvg = `<svg width="${this.width}" height="${this.height}">${svg}${textSvg}</svg>`;

            // Накладываем на изображение
            const overlay = Buffer.from(fullSvg);
            await sharp(this.inputPath)
                .composite([{ input: overlay, top: 0, left: 0 }])
                .toFile(this.output);
            console.log(chalk.green(`Comic saved to ${this.output}`));
        } catch (err) {
            console.error(chalk.red(`Error: ${err.message}`));
            process.exit(1);
        }
    }
}

program
    .requiredOption('--input <file>', 'Input image file')
    .requiredOption('--text <text>', 'Text in the bubble')
    .option('--x <number>', 'X coordinate', parseInt, 50)
    .option('--y <number>', 'Y coordinate', parseInt, 50)
    .option('--shape <type>', 'circle, square, cloud', 'circle')
    .option('--width <number>', 'Bubble width', parseInt, 200)
    .option('--height <number>', 'Bubble height', parseInt, 100)
    .option('--bubble-color <color>', 'Bubble color', '#FFFFFF')
    .option('--border-color <color>', 'Border color', '#000000')
    .option('--text-color <color>', 'Text color', '#000000')
    .option('--font-size <number>', 'Font size', parseInt, 16)
    .option('--output <file>', 'Output file', 'comic.png')
    .parse(process.argv);

const opts = program.opts();
const editor = new ComicEditor(opts);
editor.run();
