use std::{fs::DirEntry, sync::RwLock};
use std::process::Command;
use std::rc::Rc;

use slint::{Model, Window, platform::WindowAdapter};
// TODO
// Error happens when position of dir is higher than the amount of files in the folder

impl WindowAdapter for MainWindow {
    fn window(&self) -> &Window {
        todo!()
    }

    fn size(&self) -> slint::PhysicalSize {
        todo!()
    }

    fn renderer(&self) -> &dyn slint::platform::Renderer {
        todo!()
    }
}
fn main() {
    let current_dir = Rc::new(RwLock::new("".to_string()));
    let terminal_current_dir_lock = Rc::clone(&current_dir);
    let title =  "Rust File Explorer";
    let main_window = MainWindow::new().unwrap();

    let main_window_weak = main_window.as_weak();

    main_window.on_set_files( move |selected_file| {
        let main_window_weak = &main_window_weak.clone();
        main_window_weak.unwrap().set_files(Rc::new(slint::VecModel::from(vec![])).into());

        let mut files: Vec<TextInfo> = vec![
            TextInfo {
                filename: "..".into(),
                is_dir: true,
                is_selected: false,
            }
        ];
        // create write lock
        let mut curr_dir = current_dir.write().unwrap();

        *curr_dir = if &selected_file.filename == ".." {
            let mut split_filename: Vec<&str> = (*curr_dir).split('/').collect();
            split_filename.pop();
            split_filename.pop();
            split_filename.join("/")
        } else {
            format!("{}{}", *curr_dir, &selected_file.filename)
        };
        // sets title of window
        main_window_weak.unwrap().set_custom_title(format!("{} ({})", title, *curr_dir).into());

        curr_dir.push('/');
        println!("CurrentDir: {:?}; Selected file: {:?}", *curr_dir, selected_file);

        match read_directory(&curr_dir) {
            Ok(entries) => {
                entries.iter().for_each(|entry| {
                    files.push(
                        TextInfo {
                            filename: entry.file_name().into_string().unwrap().into(),
                            is_dir: entry.file_type().unwrap().is_dir(),
                            is_selected: false,
                        }
                    )
                });
            }
            Err(e) => println!("Error: {:?}", e),
        }

        files.sort_by(sort_by_name);
        let file_models = std::rc::Rc::new(slint::VecModel::from(files));
        let main_window_weak = main_window_weak.clone();
        main_window_weak.unwrap().set_files(file_models.into());
        main_window_weak.unwrap().request_redraw();

    });

    let main_window_weak = main_window.as_weak();
    main_window.on_reset_selected_files( move |selected_file| {
        println!("Resetting selected files");
        let main_window_weak = &main_window_weak.clone();
        
        let file_models = main_window_weak.unwrap().get_files();
        let out = file_models.iter().map(|file| {
            let mut file = file;
            file.is_selected = false;
            file
        }).collect::<Vec<_>>();
        let files = std::rc::Rc::new(slint::VecModel::from(out));
        main_window_weak.unwrap().set_files(files.into());


        if selected_file.is_dir {
            main_window_weak.unwrap().invoke_set_files(selected_file);
        }
    });
    main_window.invoke_set_files(TextInfo {
        filename: "/home/oliver".into(),
        is_dir: true,
        is_selected: false,
    });
    main_window.on_open_new_terminal( move || {
        let read = terminal_current_dir_lock.read().unwrap();
        Command::new("gnome-terminal")
            .arg(format!("--working-directory={}", *read))
            .output()
            .expect("Wasn't able to execute command");

        println!("{:?}", Command::new("pwd")
            .output()
            .expect("Failed to run cargo run"));
    });
    main_window.run().unwrap();
}

fn read_directory(path: &str) -> Result<Vec<DirEntry>, std::io::Error> {
    let entries = std::fs::read_dir(path)?
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    Ok(entries)
}

fn sort_by_name(a: &TextInfo, b: &TextInfo) -> std::cmp::Ordering {
    let a_is_dir = a.is_dir;
    let b_is_dir = b.is_dir;
    if a.filename == ".." && b.filename != ".." {
        return std::cmp::Ordering::Less;
    } else if a.filename != ".." && b.filename == ".." {
        return std::cmp::Ordering::Greater;
    }
    if a_is_dir && !b_is_dir {
        return std::cmp::Ordering::Less;
    } else if !a_is_dir && b_is_dir {
        return std::cmp::Ordering::Greater;
    }
    if a.filename.starts_with('.') && !b.filename.starts_with('.') {
        return std::cmp::Ordering::Greater;
    } else if !a.filename.starts_with('.') && b.filename.starts_with('.') {
        return std::cmp::Ordering::Less;
    }
    a.filename.to_lowercase().cmp(&b.filename.to_lowercase())
}

slint::slint! {
    import { ScrollView, Button } from "std-widgets.slint";
    
    struct TextInfo {
        filename: string,
        is_dir: bool,
        is_selected: bool,
    }
    export component MainWindow inherits Window {
        min-width: 600px;
        min-height: 400px;
        title: custom_title;

        in property <string> custom_title: "Rust File Explorer";

        in-out property <[TextInfo]> files: [
        ];
        in property <int> vp_height: 0;

        in property <length> file_paddings_left: 10px;
        property <length> menu_height: 30px;

        pure callback reset_selected_files(TextInfo);
        pure callback set_files(TextInfo);
        pure callback open_new_terminal();

        menu := GridLayout {
            HorizontalLayout {
                row: 1;
                col: 1;
                colspan: 2;
                height: menu_height;
                popup_file := PopupWindow {
                    y: menu_height;
                    VerticalLayout {
                        Rectangle {
                            background: #aaa;
                            Button {
                                min-width: parent.width;
                                text: "New (Ctrl + N)";
                            }
                        }
                        Rectangle {
                            background: #aaa;
                            
                            Button {
                                text: "Open";
                                min-width: parent.width;
                            }
                        }
                        Rectangle {
                            background: #aaa;
                            
                            Button {
                                min-width: parent.width;
                                text: "Close";
                            }
                        }
                    }
                }
                Rectangle {
                    background: gray;
                    HorizontalLayout {

                        Button {
                            width: 50px;
                            text: "File";
                            clicked => {
                                popup_file.show();
                            }
                        }
                        Button {
                            width: 150px;
                            text: "New Terminal";  

                            clicked => {
                                open_new_terminal()
                            } 
                        }
                    }
                }
                
                
            }
            VerticalLayout {
                row: 2;
                col: 1;
                min-width: 80px;
                width: 10%;

                Rectangle {
                    background: gray;
                }
            }
            scroll := ScrollView {
                row: 2;
                col: 2;
                height: 100%;
                width: 100%;
                viewport-height: files.length * 20px;

            
                for file[i] in files : TouchArea {
                    width: parent.width;
                    height: 20px;
                    x: file_paddings_left;
                    y: i * 20px;
                    Rectangle {
                        background: file.is_selected ? #0f03 : transparent;
                        text:= Text {
                            text: file.filename;
                            color: file.is_dir ? green : blue;
                            x: parent.x;    
                        }
                        
                    }
                    clicked => {
                        if(file.is_dir) {
                            scroll.enabled = false;
                            set_files(file);
                            scroll.enabled = true;
                        } else {
                            reset_selected_files(file);
                            file.is_selected = true;
                            
                        }
                    }
                    
                }
            }
        }
    }
}
