pub fn denoise_bilateral(
    pixels: &[u8],
    width: usize,
    height: usize,
) -> Vec<u8> {
    let mut output = vec![0u8; pixels.len()];

    const RADIUS: i32 = 2;

    let sigma_space = 2.0f32;
    let sigma_color = 25.0f32;

    let two_sigma_space_sq = 2.0 * sigma_space * sigma_space;
    let two_sigma_color_sq = 2.0 * sigma_color * sigma_color;

    for y in 0..height {
        for x in 0..width {
            let center_idx = (y * width + x) * 4;

            let center_r = pixels[center_idx] as f32;
            let center_g = pixels[center_idx + 1] as f32;
            let center_b = pixels[center_idx + 2] as f32;

            let mut r_sum = 0.0;
            let mut g_sum = 0.0;
            let mut b_sum = 0.0;
            let mut weight_sum = 0.0;

            for oy in -RADIUS..=RADIUS {
                for ox in -RADIUS..=RADIUS {
                    let nx = x as i32 + ox;
                    let ny = y as i32 + oy;

                    if nx < 0
                        || ny < 0
                        || nx >= width as i32
                        || ny >= height as i32
                    {
                        continue;
                    }

                    let idx =
                        ((ny as usize * width + nx as usize) * 4);

                    let r = pixels[idx] as f32;
                    let g = pixels[idx + 1] as f32;
                    let b = pixels[idx + 2] as f32;

                    // Spatial weight
                    let dist_sq = (ox * ox + oy * oy) as f32;
                    let spatial =
                        (-dist_sq / two_sigma_space_sq).exp();

                    // Color weight
                    let dr = r - center_r;
                    let dg = g - center_g;
                    let db = b - center_b;

                    let color_sq =
                        dr * dr + dg * dg + db * db;

                    let color =
                        (-color_sq / two_sigma_color_sq).exp();

                    let weight = spatial * color;

                    r_sum += r * weight;
                    g_sum += g * weight;
                    b_sum += b * weight;

                    weight_sum += weight;
                }
            }

            output[center_idx] =
                (r_sum / weight_sum).round() as u8;
            output[center_idx + 1] =
                (g_sum / weight_sum).round() as u8;
            output[center_idx + 2] =
                (b_sum / weight_sum).round() as u8;

            // Preserve alpha
            output[center_idx + 3] = pixels[center_idx + 3];
        }
    }

    output
}