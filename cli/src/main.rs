use common::{account, utils::Config};
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    common::init_logger().unwrap_err();

    info!("正在加载...");

    info!("正在释放资源文件...");
    common::resources::ensure_resources().unwrap_err();

    info!("正在加载配置文件...");
    let config = match Config::load_config() {
        Ok(load_config) => {
            log::info!("配置文件加载成功");
            load_config
        }

        Err(e) => {
            error!("配置文件加载失败: {}", e);
            info!("尝试迁移json配置");
            match Config::load_json_config() {
                Ok(load_config) => {
                    info!("配置文件加载成功");
                    match load_config.save_config() {
                        Ok(_) => {
                            info!("配置文件保存成功");
                            match Config::delete_json_config() {
                                Ok(_) => {
                                    info!("旧配置文件删除成功");
                                }
                                Err(e) => {
                                    error!("旧配置文件删除失败: {}", e);
                                }
                            }
                            info!("迁移成功");
                        }
                        Err(e) => {
                            error!("配置文件保存失败: {}", e);
                        }
                    }
                    load_config
                }
                Err(e) => {
                    log::error!("迁移失败: {}", e);
                    let cfg = Config::new();
                    match cfg.save_config() {
                        Ok(_) => {
                            info!("配置文件保存成功");
                        }
                        Err(e) => {
                            error!("配置文件保存失败: {}", e);
                        }
                    }
                    cfg
                }
            }
        }
    };

    info!("正在初始化客户端...");

    Ok(())
}
