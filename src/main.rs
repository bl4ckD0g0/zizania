use druid::{Data, widget::{ Button, Flex}, Widget, WindowDesc, AppLauncher};

#[derive(Clone, Data)]
struct FunnyData {
    num: i32
}

fn foo(){
    println!("grabando"); 
}


fn bar(){
    println!("reproduciendo"); 
}

fn ui_builder() -> impl Widget<FunnyData> {

    let rec_button = Button::new("o")
        .on_click(|_ctx, _data: &mut FunnyData, _env| foo());
    let play_button = Button::new(">")
        .on_click(|_ctx, _data: &mut FunnyData, _env| bar());

    Flex::row().with_child(rec_button).with_child(play_button)
}

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("Funny Window");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(FunnyData { num: 0 }).unwrap();
}
