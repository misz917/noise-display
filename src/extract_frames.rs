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
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("ffmpeg exited with {}", status),
        ))
    } else {
        Ok(())
    }
}
