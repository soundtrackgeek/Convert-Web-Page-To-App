import { createCanvas } from 'canvas';
import { writeFileSync } from 'fs';
import { join } from 'path';
import { fileURLToPath } from 'url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));

// Create a 512x512 canvas (large enough for all icon sizes)
const canvas = createCanvas(512, 512);
const ctx = canvas.getContext('2d');

// Fill background
ctx.fillStyle = '#2196F3';
ctx.fillRect(0, 0, 512, 512);

// Add some simple design
ctx.fillStyle = '#FFFFFF';
ctx.font = 'bold 200px Arial';
ctx.textAlign = 'center';
ctx.textBaseline = 'middle';
ctx.fillText('W', 256, 256);

// Save the image
const buffer = canvas.toBuffer('image/png');
writeFileSync(join(__dirname, 'app-icon.png'), buffer);
