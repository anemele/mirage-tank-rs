use image::{GrayAlphaImage, GrayImage, Luma};

/// 解析幻影坦克图片，并分离顶图和底图。
/// 生成幻影坦克的原理是可逆的，因此分离图片是可行的。当然，彩图转灰图是不可逆的。
/// 另外计算过程因为浮点数与整数转换，存在精度损失问题

pub fn operate(path: &str) -> anyhow::Result<()> {
    let img = read_mirage_tank(path)?;
    let (mut top, mut bottom) = separate_mirage_tank(&img);
    darken(&mut top)?;
    lighten(&mut bottom)?;
    let name = path.split_at(path.len() - 4).0;
    top.save(format!("{}_top.jpg", name))?;
    bottom.save(format!("{}_bottom.jpg", name))?;
    Ok(())
}

fn read_mirage_tank(path: &str) -> anyhow::Result<GrayAlphaImage> {
    let img = image::open(path)?.to_luma_alpha8();
    Ok(img)
}

fn separate_mirage_tank(img: &GrayAlphaImage) -> (GrayImage, GrayImage) {
    let (width, height) = img.dimensions();
    let mut top = GrayImage::new(width, height);
    let mut bottom = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let gray = pixel.0[0];
            let alpha = pixel.0[1];
            if alpha == 0 && gray == 0 {
                let pixel = Luma([128]); // ?这里不知是否正确
                top.put_pixel(x, y, pixel);
                bottom.put_pixel(x, y, pixel);
            } else {
                let v = (gray as f32 / 255.0 * alpha as f32).round() as u8;
                let bottom_pixel = Luma([v]);
                let top_pixel = Luma([255 - alpha + v]); // 先减后加，否则会整数溢出
                bottom.put_pixel(x, y, bottom_pixel);
                top.put_pixel(x, y, top_pixel);
            }
        }
    }
    (top, bottom)
}

fn lighten(img: &mut GrayImage) -> anyhow::Result<()> {
    for pixel in img.pixels_mut() {
        let v = pixel.0[0];
        if v > 127 {
            continue;
        }
        *pixel = Luma([v << 1]);
    }
    Ok(())
}

fn darken(img: &mut GrayImage) -> anyhow::Result<()> {
    for pixel in img.pixels_mut() {
        let v = pixel.0[0];
        if v < 128 {
            continue;
        }
        *pixel = Luma([(v - 128) << 1]);
    }
    Ok(())
}

// 还原尺寸存在难度，暂时不做了
