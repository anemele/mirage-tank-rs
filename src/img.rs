use image::{GrayAlphaImage, GrayImage, Luma, LumaA};

pub fn make(top_path: &str, bottom_path: &str, output: &str) -> anyhow::Result<()> {
    let mut i1 = read_to_gray(top_path)?;
    let mut i2 = read_to_gray(bottom_path)?;

    lighten(&mut i1)?;
    darken(&mut i2)?;

    let (i11, i22) = same_size_and_center(&i1, &i2);

    let res = merge(&i11, &i22);
    let output = if output.ends_with(".png") {
        output
    } else {
        &format!("{}.png", output)
    };
    res.save(output)?;

    Ok(())
}

fn read_to_gray(path: &str) -> anyhow::Result<GrayImage> {
    let img = image::open(path)?.to_luma8();
    Ok(img)
}

fn lighten(image: &mut GrayImage) -> anyhow::Result<()> {
    for pixel in image.pixels_mut() {
        *pixel = Luma([(pixel.0[0] >> 1) + 128]);
    }

    Ok(())
}

fn darken(image: &mut GrayImage) -> anyhow::Result<()> {
    for pixel in image.pixels_mut() {
        *pixel = Luma([pixel.0[0] >> 1]);
    }

    Ok(())
}

fn same_size_and_center(top: &GrayImage, bottom: &GrayImage) -> (GrayImage, GrayImage) {
    let (tw, th) = top.dimensions();
    let (bw, bh) = bottom.dimensions();

    let width = tw.max(bw);
    let height = th.max(bh);

    let top_x_offset = (width - tw) / 2;
    let top_y_offset = (height - th) / 2;
    let bottom_x_offset = (width - bw) / 2;
    let bottom_y_offset = (height - bh) / 2;

    let mut new_top = GrayImage::new(width, height);
    for pixel in new_top.pixels_mut() {
        *pixel = Luma([255]);
    }

    let mut new_bottom = GrayImage::new(width, height);

    for (x, y, pixel) in top.enumerate_pixels() {
        new_top.put_pixel(x + top_x_offset, y + top_y_offset, *pixel);
    }

    for (x, y, pixel) in bottom.enumerate_pixels() {
        new_bottom.put_pixel(x + bottom_x_offset, y + bottom_y_offset, *pixel);
    }

    (new_top, new_bottom)
}

/// 此处输入图片尺寸一致
fn merge(top: &GrayImage, bottom: &GrayImage) -> GrayAlphaImage {
    let mut res = GrayAlphaImage::new(top.width(), top.height());

    for x in 0..res.width() {
        for y in 0..res.height() {
            let tp = top.get_pixel(x, y).0[0];
            let bp = bottom.get_pixel(x, y).0[0];
            let alpha: u8;
            let gray: u8;
            if tp == bp {
                alpha = 0;
                gray = 0;
            } else {
                alpha = 255 - (tp - bp);
                gray = (bp as f32 / alpha as f32 * 255.0) as u8;
            }
            res.put_pixel(x, y, LumaA([gray, alpha]));
        }
    }

    res
}
