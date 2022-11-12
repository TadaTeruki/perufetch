use image::GenericImageView;

const IMAGE_SOURCE: &str =
    "https://github.com/TadaTeruki/perufetch/blob/main/image/Peruki.png?raw=true";

const INFO: [&str; 16] = [
    "\x1b[1;33mPeruki@future-university-hakodate\x1b[0m",
    "---------------------------------",
    "\x1b[1;33mName\x1b[0m: Teruki TADA",
    "\x1b[1;33mShell\x1b[0m: Japanese",
    "\x1b[1;33mLocation\x1b[0m: Hakodate, Japan",
    "\x1b[1;33mAffiliation\x1b[0m: Future University Hakodate",
    "\x1b[1;33mMascot\x1b[0m: Perukun",
    "\x1b[1;33mTwitter\x1b[0m: @PerukiFUN",
    "\x1b[1;33mGitHub\x1b[0m: TadaTeruki",
    "\x1b[1;33mNative Language\x1b[0m: Go",
    "\x1b[1;33mTrained Languages\x1b[0m: C, C++, Go, (Rust)",
    "\x1b[1;33mFields\x1b[0m: Procedural Generation (CG)",
    "\x1b[1;33m      \x1b[0m: Backend Web Development",
    "",
    "\x1b[30m███\x1b[31m███\x1b[32m███\x1b[33m███\x1b[34m███\x1b[35m███\x1b[36m███\x1b[37m███",
    "\x1b[90m███\x1b[91m███\x1b[92m███\x1b[93m███\x1b[94m███\x1b[95m███\x1b[96m███\x1b[97m███",
];

fn get_image() -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let img_bytes = reqwest::blocking::get(IMAGE_SOURCE)?.bytes()?;
    let img = image::load_from_memory(&img_bytes)?;
    Ok(img)
}

fn main() {
    let img = get_image().expect("image not found.");

    let swidth = 45i32;
    let sheight = 18i32;
    let trim_swidth = 6i32;
    let trim_sheight = 2i32;

    let iwidth = img.dimensions().0 as i32;
    let iheight = img.dimensions().1 as i32;

    let mut surface = vec![vec![0f32; swidth as usize]; sheight as usize];

    for p in img.pixels() {
        let x = p.0 as i32;
        let y = p.1 as i32;
        let ix = (x * swidth / iwidth) as usize;
        let iy = (y * sheight / iheight) as usize;

        let [r, g, b, _] = (p.2).0;

        if ix as i32 >= swidth || iy as i32 >= sheight {
            continue;
        }
        surface[iy][ix] += (r as i32 + g as i32 + b as i32) as f32 / (255.0 * 3.0);
    }

    let max_sum = ((iwidth / swidth) * (iheight / sheight)) as f32;

    for (iy, sr) in surface
        .iter()
        .enumerate()
        .take(sheight as usize)
        .skip(trim_sheight as usize)
    {
        for (_, s) in sr
            .iter()
            .enumerate()
            .take(swidth as usize)
            .skip(trim_swidth as usize)
        {
            let c = if iy < (sheight - trim_sheight) as usize {
                1.0 - s / max_sum
            } else {
                0.0
            };
            print!(
                "\x1b[1;32m{}\x1b[0m",
                if c > 0.38 {
                    "#"
                } else if c > 0.2 {
                    "="
                } else if c > 0.15 {
                    "."
                } else {
                    " "
                }
            );
        }

        let info_y = iy - trim_sheight as usize;
        println!(
            "  {}",
            if info_y < INFO.len() {
                INFO[info_y]
            } else {
                ""
            }
        );
    }
}
