use anyhow::{Context, Result};
use fs_extra::dir::copy;
use include_dir::{Dir, include_dir};

static RESOURCES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources");

pub fn ensure_resources() -> Result<()> {
    // 获取当前工作目录
    let current_dir = std::env::current_dir().context("Failed to get current working directory")?;

    // 设置目标目录为当前目录下的 resources 文件夹
    let target_dir = current_dir.join("");

    log::info!("Checking resources in: {:?}", target_dir);

    // 创建资源目录
    std::fs::create_dir_all(&target_dir).context("Failed to create resources directory")?;

    // 临时目录用于解压资源
    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;

    log::debug!(
        "Extracting embedded resources to temp dir: {:?}",
        temp_dir.path()
    );

    // 释放嵌入的资源到临时目录
    RESOURCES_DIR
        .extract(temp_dir.path())
        .context("Failed to extract embedded resources")?;

    // 复制到当前目录下的 resources 目录
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true; // 复制目录内容
    options.overwrite = true; // 覆盖已存在的文件
    options.content_only = true; // 只复制内容，不复制目录本身

    log::debug!("Copying resources to: {:?}", target_dir);

    copy(temp_dir.path(), &target_dir, &options)
        .context("Failed to copy resources to target directory")?;

    log::info!("Resources released to: {:?}", target_dir);

    Ok(())
}
