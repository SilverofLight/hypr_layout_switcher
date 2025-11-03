use hyprland::keyword::Keyword;
use std::process::Command;

fn main() -> hyprland::Result<()> {
    let current_layout = Keyword::get("general:layout")?.value.to_string();
    println!("Current layout: {current_layout}");

    if current_layout != "master" {
        switch_to_master();
        Command::new("notify-send")
            .arg("Hyprland Layout") // 标题
            .arg(format!("switch to master")) // 内容
            .output() // 实际执行
            .expect("notify-send 未安装？");
    } else if current_layout == "master" {
        switch_to_scrolling();
        Command::new("notify-send")
            .arg("Hyprland Layout") // 标题
            .arg(format!("switch to scrolling")) // 内容
            .output() // 实际执行
            .expect("notify-send 未安装？");
    }

    Ok(())
}

fn switch_to_master() {
    let _ = Keyword::set("general:layout", "master");

    let _ = Keyword::set("unbind", "ALT SHIFT, h");
    let _ = Keyword::set("unbind", "ALT SHIFT, i");
    let _ = Keyword::set("unbind", "ALT SHIFT, n");
    let _ = Keyword::set("unbind", "ALT SHIFT, e");

    let _ = Keyword::set("bind", "ALT SHIFT, h, movewindow, l");
    let _ = Keyword::set("bind", "ALT SHIFT, i, movewindow, r");
    let _ = Keyword::set("bind", "ALT SHIFT, n, movewindow, u");
    let _ = Keyword::set("bind", "ALT SHIFT, e, movewindow, d");

    let _ = Keyword::set("unbind", "ALT CTRL, m");
    let _ = Keyword::set("unbind", "ALT CTRL, s");
    let _ = Keyword::set("unbind", "ALT, Minus");
    let _ = Keyword::set("unbind", "ALT SHIFT, Equal");
}

fn switch_to_scrolling() {
    let _ = Keyword::set("general:layout", "scrolling");

    let _ = Keyword::set("unbind", "ALT SHIFT, h");
    let _ = Keyword::set("unbind", "ALT SHIFT, i");
    let _ = Keyword::set("unbind", "ALT SHIFT, n");
    let _ = Keyword::set("unbind", "ALT SHIFT, e");

    let _ = Keyword::set("bind", "ALT SHIFT, h, layoutmsg, swapcol l");
    let _ = Keyword::set("bind", "ALT SHIFT, i, layoutmsg, swapcol r");
    let _ = Keyword::set("bind", "ALT SHIFT, n, layoutmsg, movewindowto d");
    let _ = Keyword::set("bind", "ALT SHIFT, e, layoutmsg, movewindowto u");

    let _ = Keyword::set("bind", "ALT CTRL, m, layoutmsg, colresize 0.55");
    let _ = Keyword::set("bind", "ALT CTRL, s, layoutmsg, colresize 0.45");
    let _ = Keyword::set("bind", "ALT, Minus, layoutmsg, colresize -conf");
    let _ = Keyword::set("bind", "ALT SHIFT, Equal, layoutmsg, colresize +conf");
}
