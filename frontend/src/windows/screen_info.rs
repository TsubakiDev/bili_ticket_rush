use crate::app::Myapp;
use chrono::TimeZone;
use eframe::egui;
use serde_json::Value;

pub fn show(app: &mut Myapp, ctx: &egui::Context, uid: i64) {
    let bilibili_ticket = app
        .bilibiliticket_list
        .iter_mut()
        .find(|ticket| ticket.uid == uid)
        .unwrap();
    let mut window_open = app.show_screen_info.is_some();

    let ticket_data = match bilibili_ticket.project_info.clone() {
        Some(ticket) => {
            app.is_loading = false;
            ticket
        }
        None => {
            app.is_loading = true;
            return;
        }
    };

    // 默认选择第一个场次（如果尚未选择）
    if app.selected_screen_index.is_none() && !ticket_data.screen_list.is_empty() {
        app.selected_screen_index = Some(0);
    }

    bilibili_ticket.id_bind = ticket_data.id_bind as usize;

    // 创建局部变量用于窗口控制
    let mut close_requested = false;
    let mut refresh_requested = false;

    // 获取屏幕尺寸用于约束窗口大小
    let screen_size = ctx.input(|i| i.screen_rect.size());

    egui::Window::new("项目详情")
    .open(&mut window_open)
    .default_size([800.0, 600.0])
    .resizable(true)
    .auto_sized() // 自动调整大小
    .constrain(true) // 确保窗口不会超出屏幕
    //.max_size([max_width, max_height]) // 设置最大尺寸
    .show(ctx, |ui| {
        egui::ScrollArea::vertical()
        .id_source("project_details_scroll")
        .show(ui, |ui| {
            // 项目标题区 - 使用卡片样式
            egui::Frame::group(ui.style())
                .fill(if ctx.style().visuals.dark_mode {
                    egui::Color32::from_rgb(30, 30, 40)
                } else {
                    egui::Color32::from_rgb(245, 245, 255)
                })
                .inner_margin(egui::Margin::symmetric(15.0, 10.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new(&ticket_data.name)
                                .size(22.0)
                                .color(if ctx.style().visuals.dark_mode {
                                    egui::Color32::from_rgb(255, 215, 100)
                                } else {
                                    egui::Color32::from_rgb(200, 50, 50)
                                })
                        );
                        ui.add_space(5.0);

                        // 状态标签 - 根据状态显示不同颜色
                        let status_color = match ticket_data.sale_flag.as_str() {
                            "在售" => egui::Color32::from_rgb(50, 200, 50),
                            "预售" => egui::Color32::from_rgb(100, 180, 255),
                            "售罄" => egui::Color32::from_rgb(200, 50, 50),
                            _ => ui.visuals().text_color(),
                        };
                        ui.label(
                            egui::RichText::new(format!("状态: {}", ticket_data.sale_flag))
                                .color(status_color)
                                .strong()
                        );
                    });
                });

            ui.add_space(10.0);

            // 场次选择区
            egui::Frame::group(ui.style())
                .inner_margin(egui::Margin::symmetric(10.0, 5.0))
                .show(ui, |ui| {
                    ui.heading("选择场次");
                    ui.add_space(5.0);

                    // 场次选择栏 - 使用按钮组样式
                    egui::ScrollArea::horizontal()
                        .id_source("screen_selection_scroll")
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                for (idx, screen) in ticket_data.screen_list.iter().enumerate() {
                                    let is_selected = app.selected_screen_index == Some(idx);
                                    // 根据销售状态设置按钮颜色
                                    let button_color = match screen.sale_flag.display_name.as_str() {
                                        "在售" => egui::Color32::from_rgb(50, 200, 50),
                                        "预售" => egui::Color32::from_rgb(100, 180, 255),
                                        "售罄" => egui::Color32::from_rgb(200, 50, 50),
                                        _ => ui.visuals().widgets.inactive.bg_fill,
                                    };

                                    let response = ui.add(
                                        egui::Button::new(
                                            egui::RichText::new(format!("{} ({})", screen.name, &screen.sale_flag.display_name))
                                                .color(if is_selected {
                                                    egui::Color32::WHITE
                                                } else {
                                                    ui.visuals().text_color()
                                                })
                                        )
                                        .fill(if is_selected { button_color } else { ui.visuals().widgets.inactive.bg_fill })
                                        .min_size(egui::Vec2::new(120.0, 30.0))
                                    );

                                    if response.clicked() {
                                        app.selected_screen_index = Some(idx);
                                    }
                                }
                            });
                        });
                });

            ui.add_space(15.0);

            // 显示选中场次的票种信息
            if let Some(idx) = app.selected_screen_index {
                if idx < ticket_data.screen_list.len() {
                    let selected_screen = &ticket_data.screen_list[idx];
                    // 场次信息卡片 - 增强视觉样式
                    let bg_color = if !ctx.style().visuals.dark_mode {
                        egui::Color32::from_rgb(245, 245, 250)
                    } else {
                        egui::Color32::from_rgb(20, 20, 25)
                    };
                    egui::Frame::none()
                        .fill(bg_color)
                        .rounding(8.0)
                        .inner_margin(15.0)
                        .outer_margin(5.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(100, 100, 100, 50)))
                        .show(ui, |ui| {
                            // 场次基本信息
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.heading("场次信息");
                                    ui.add_space(10.0);
                                    // 场次状态标签
                                    let status_color = match selected_screen.sale_flag.display_name.as_str() {
                                        "在售" => egui::Color32::from_rgb(50, 200, 50),
                                        "预售" => egui::Color32::from_rgb(100, 180, 255),
                                        "售罄" => egui::Color32::from_rgb(200, 50, 50),
                                        _ => ui.visuals().text_color(),
                                    };
                                    ui.label(
                                        egui::RichText::new(&selected_screen.sale_flag.display_name)
                                            .color(status_color)
                                            .strong()
                                    );
                                });
                                ui.add_space(5.0);
                                // 时间信息网格布局
                                egui::Grid::new("screen_info_grid")
                                    .num_columns(2)
                                    .spacing([40.0, 8.0])
                                    .min_col_width(100.0)
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new("开始时间:").strong());
                                        ui.label(format_timestamp(selected_screen.start_time));
                                        ui.end_row();
                                        ui.label(egui::RichText::new("售票开始:").strong());
                                        ui.label(format_timestamp(selected_screen.sale_start));
                                        ui.end_row();
                                        ui.label(egui::RichText::new("售票结束:").strong());
                                        ui.label(format_timestamp(selected_screen.sale_end));
                                        ui.end_row();
                                    });
                            });

                            ui.add_space(12.0);
                            ui.separator();
                            ui.add_space(12.0);

                            ui.heading("票种列表");

                            // 票种表格头 - 使用网格布局确保对齐
                            egui::Grid::new("ticket_header_grid")
                                .num_columns(4)
                                .spacing([10.0, 5.0])
                                .min_col_width(100.0)
                                .show(ui, |ui| {
                                    ui.strong("票种名称");
                                    ui.strong("价格");
                                    ui.strong("状态");
                                    ui.strong("操作");
                                    ui.end_row();
                                });

                            ui.separator();
                            ui.add_space(8.0);
                            // 票种列表 - 使用网格布局
                            for ticket in &selected_screen.ticket_list {
                                egui::Grid::new(format!("ticket_row_{}", ticket.id))
                                    .num_columns(4)
                                    .spacing([10.0, 10.0])
                                    .min_col_width(100.0)
                                    .show(ui, |ui| {
                                        // 票种名称
                                        ui.label(&ticket.desc);
                                        // 价格
                                        let price = format!("¥{:.2}", ticket.price as f64 / 100.0);
                                        ui.label(
                                            egui::RichText::new(price)
                                                .strong()
                                                .color(egui::Color32::from_rgb(245, 108, 108))
                                        );
                                        // 状态
                                        let status_color = match ticket.sale_flag.display_name.as_str() {
                                            "在售" => egui::Color32::from_rgb(50, 200, 50),
                                            "预售" => egui::Color32::from_rgb(100, 180, 255),
                                            "售罄" => egui::Color32::from_rgb(200, 50, 50),
                                            _ => ui.visuals().text_color(),
                                        };
                                        ui.label(
                                            egui::RichText::new(&ticket.sale_flag.display_name)
                                                .color(status_color)
                                        );
                                        // 操作按钮
                                        let (button_text, button_color) = if ticket.clickable {
                                            ("选择", egui::Color32::from_rgb(65, 150, 65))
                                        } else if ticket.sale_flag_number == 1 {
                                            ("定时预选", egui::Color32::from_rgb(100, 150, 220))
                                        } else {
                                            ("不可选", egui::Color32::from_rgb(150, 150, 150))
                                        };
                                        let button = egui::Button::new(button_text)
                                            .fill(button_color)
                                            .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
                                            .min_size(egui::Vec2::new(80.0, 30.0));
                                        if ui.add(button).clicked() {
                                            if !ticket.clickable {
                                                log::error!("请注意！该票种目前不可售！但是会尝试下单, 如果该票持续不可售, 多次下单不可售票种可能会被b站拉黑");
                                            }
                                            app.selected_screen_id = Some(selected_screen.id as i64);
                                            app.selected_ticket_id = Some(ticket.id as i64);
                                            app.show_screen_info = None;
                                            bilibili_ticket.screen_id = selected_screen.id.to_string();
                                            log::debug!("场次ID: {}, 票种ID: {}, 项目ID: {}", selected_screen.id, ticket.id, ticket.project_id);
                                            app.ticket_id = ticket.project_id.to_string();
                                            bilibili_ticket.select_ticket_id = Some(ticket.id.to_string());
                                            app.confirm_ticket_info = Some(bilibili_ticket.uid.to_string());
                                            log::info!("已选择: {} [{}]", &ticket.desc, ticket.id);
                                        }
                                        ui.end_row();
                                    });
                                ui.add_space(5.0);
                            }
                        });
                }
            }

            ui.add_space(15.0);

            // 项目详细信息区
            ui.collapsing(
                egui::RichText::new("查看详细信息").strong(),
                |ui| {
                    ui.add_space(5.0);
                    // 基本信息部分
                    ui.label(egui::RichText::new("基本信息").heading());
                    ui.add_space(5.0);
                    egui::Frame::group(ui.style())
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            ui.label(format!("项目ID: {}", ticket_data.id));
                            ui.add_space(8.0);
                            if let Some(desc) = &ticket_data.performance_desc {
                                for item in &desc.list {
                                    if item.module == "base_info" {
                                        if let Some(array) = item.details.as_array() {
                                            for info_item in array {
                                                if let (Some(title), Some(content)) = (
                                                    info_item.get("title").and_then(Value::as_str),
                                                    info_item.get("content").and_then(Value::as_str)
                                                ) {
                                                    ui.horizontal(|ui| {
                                                        ui.label(egui::RichText::new(format!("{}:", title)).strong());
                                                        ui.label(content);
                                                    });
                                                    ui.add_space(3.0);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    ui.add_space(15.0);
                    // JSON元数据部分 - 固定高度, 避免展开时窗口大小变化
                    ui.label(egui::RichText::new("JSON元数据").heading());
                    ui.add_space(5.0);
                    let json_str = serde_json::to_string_pretty(&ticket_data).unwrap_or_else(|_| "无法格式化JSON".to_string());
                    // 固定高度区域, 避免展开时窗口大小变化
                    egui::Frame::group(ui.style())
                        .inner_margin(5.0)
                        .show(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(200.0) // 固定高度
                                .auto_shrink([false, false]) // 不自动收缩
                                .show(ui, |ui| {
                                    egui::TextEdit::multiline(&mut json_str.as_str())
                                        .font(egui::TextStyle::Monospace)
                                        .desired_width(f32::INFINITY)
                                        .lock_focus(true)
                                        .interactive(false) // 设置为只读
                                        .show(ui);
                                });
                        });
                }
            );
        });

        // 底部按钮 - 使用标志变量而不是直接修改外部变量
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("关闭").clicked() {
                close_requested = true;
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                if ui.button("刷新信息").clicked() {
                    refresh_requested = true;
                }
            });
        });
    });

    // 处理关闭请求
    if close_requested {
        window_open = false;
    }

    // 如果窗口关闭, 清理数据
    if !window_open {
        app.show_screen_info = None;
        bilibili_ticket.project_info = None;
    }
}

// 将时间戳转换为可读时间
// 将时间戳转换为可读时间 (接受usize类型)
fn format_timestamp(timestamp: usize) -> String {
    if timestamp <= 0 {
        return "未设置".to_string();
    }

    // 安全地将usize转为i64
    let timestamp_i64 = match i64::try_from(timestamp) {
        Ok(ts) => ts,
        Err(_) => return "时间戳溢出".to_string(), // 处理极端情况
    };

    match chrono::Local.timestamp_opt(timestamp_i64, 0) {
        chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        _ => "无效时间".to_string(),
    }
}
