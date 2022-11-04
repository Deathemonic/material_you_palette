//import {QuantizerCelebi} from '../quantize/quantizer_celebi';
use crate::score::score;
use color::argbFromRgb;

/**
 * Get the source color from an image.
 *
 * @param image The image element
 * @return Source color - the color most suitable for creating a UI theme
 */
/* export async function sourceColorFromImage(image: HTMLImageElement) { */

  //pub fn sourceColorFromImage(image) -> {
  /* Convert Image data to Pixel Array */
  //const imageBytes = await new Promise<Uint8ClampedArray>((resolve, reject) => {
  //  const canvas = document.createElement('canvas');
  //  const context = canvas.getContext('2d');
  //  if (!context) {
  //      return reject(new Error('Could not get canvas context'));
  //  }
  //  image.onload = () => {
  //    canvas.width = image.width;
  //    canvas.height = image.height;
  //    context.drawImage(image, 0, 0);
  //    resolve(context.getImageData(0, 0, image.width, image.height).data);
  //  };
  //});

  /* Convert Image data to Pixel Array */
  //const pixels: number[] = [];
  //for (let i = 0; i < imageBytes.length; i += 4) {
  //  const r = imageBytes[i];
  //  const g = imageBytes[i + 1];
  //  const b = imageBytes[i + 2];
  //  const a = imageBytes[i + 3];
  //  if (a < 255) {
  //    continue;
  //  }
  //  const argb = argbFromRgb(r, g, b);
  //  pixels.push(argb);
  //}

  /* Convert Pixels to Material Colors */
  // const result = QuantizerCelebi.quantize(pixels, 128);
  // const ranked = Score.score(result);
  // const top = ranked[0];
  // return top;
//}
