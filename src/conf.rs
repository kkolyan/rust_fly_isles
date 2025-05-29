use ini::Ini;
use std::fs::File;
use macroquad::logging::error;
use crate::Conf;

pub fn deal_with_config_file(default_conf: Conf) -> Conf {
    let path = "config.conf";
    if File::open(path).is_ok() {
        match Ini::load_from_file(path) {
            Ok(ini) => ini_to_conf(ini, default_conf),
            Err(err) => {
                error!("invalid config file: {}", err);
                default_conf
            }
        }
    } else {
        let default = conf_to_ini(&default_conf);
        default.write_to_file(path);
        default_conf
    }
}

fn conf_to_ini(conf: &Conf) -> Ini {
    let mut ini = Ini::new();
    ini.with_general_section()
        .set("window_width", conf.window_width.to_string())
        .set("window_height", conf.window_height.to_string())
        .set("fullscreen", conf.fullscreen.to_string());
    ini
}

fn ini_to_conf(ini: Ini, conf: Conf) -> Conf {
    let mut conf = conf;
    if let Some(v) = ini.general_section().get("window_width") {
        if let Ok(v) = v.parse() {
            conf.window_width = v;
        }
    }
    if let Some(v) = ini.general_section().get("window_height") {
        if let Ok(v) = v.parse() {
            conf.window_height = v;
        }
    }
    if let Some(v) = ini.general_section().get("fullscreen") {
        if let Ok(v) = v.parse() {
            conf.fullscreen = v;
        }
    }
    conf
}
