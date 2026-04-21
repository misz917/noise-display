use std::path::Path;
use std::process::Command;

pub fn extract_frames_with_ffmpeg(input: &Path, out_dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(out_dir)?;
    // Example output pattern: out_dir/frame_000001.png
    let output_pattern = out_dir.join("frame_%06d.png");
    let output_str = output_pattern.to_string_lossy();

    // Example command:
    // ffmpeg -i input.mp4 -vsync 0 -frame_pts true out_dir/frame_%06d.png
    let status = Command::new("ffmpeg")
        .arg("-hide_banner")
        .arg("-loglevel")
        .arg("error")
        .arg("-i")
        .arg(input)
        // adjust fps or filters here if desired, e.g. "-vf", "fps=25"
        .arg("-vsync")
        .arg("0")
        .arg(output_str.as_ref())
        .status()?;

    if !status.success() {
        Err(std::io::Error::other(format!(
            "ffmpeg exited with {}",
            status
        )))
    } else {
        Ok(())
    }
}

pub fn get_fps(input: &Path) -> std::io::Result<f64> {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-show_entries")
        .arg("stream=r_frame_rate")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(input)
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::other("ffprobe failed"));
    }

    let fps_str = String::from_utf8_lossy(&output.stdout);
    let fps_str = fps_str.trim();

    // Parse something like "30000/1001"
    let parts: Vec<&str> = fps_str.split('/').collect();
    if parts.len() == 2 {
        let num: f64 = parts[0].parse().unwrap_or(0.0);
        let den: f64 = parts[1].parse().unwrap_or(1.0);
        Ok(num / den)
    } else {
        // Sometimes it's already a single number
        Ok(fps_str.parse().unwrap_or(0.0))
    }
}
