use std::fs::DirEntry;

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
    let mut current_dir = "".to_string();
    let title =  "Rust File Explorer";
    let main_window = MainWindow::new().unwrap();

    let main_window_weak = main_window.as_weak();

    main_window.on_set_files( move |selected_file| {
        let main_window_weak = &main_window_weak.clone();
        main_window_weak.unwrap().set_files(std::rc::Rc::new(slint::VecModel::from(vec![])).into());
        let mut files: Vec<TextInfo> = Vec::new();
        files.push(TextInfo {
            filename: "..".into(),
            is_dir: true,
            is_selected: false,
        });
        if &selected_file.filename == ".." {
            let mut split_filename: Vec<&str> = current_dir.split('/').collect();
            split_filename.pop();
            split_filename.pop();
            current_dir = split_filename.join("/");
        } else {
            current_dir.push_str(&selected_file.filename);
        }

        main_window_weak.unwrap().set_custom_title(format!("{} ({})", title, current_dir).into());
        current_dir.push('/');
        println!("CurrentDir: {:?}; Selected file: {:?}", current_dir, selected_file);

        match read_directory(&current_dir) {
            Ok(entries) => {
                for entry in &entries {
                    let filename = entry.file_name().into_string().unwrap().into();
                    let is_dir = entry.file_type().unwrap().is_dir();
                    let text_info = TextInfo {
                        filename,
                        is_dir,
                        is_selected: false,
                    };
                    files.push(text_info);
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
        files.sort_by(|a: &TextInfo, b: &TextInfo| {
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
        });
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
    main_window.run().unwrap();
}

fn read_directory(path: &str) -> Result<Vec<DirEntry>, std::io::Error> {
    let entries = std::fs::read_dir(path)?
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    Ok(entries)
}


slint::slint! {
    import { ScrollView } from "std-widgets.slint";
    
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

        pure callback reset_selected_files(TextInfo);
        pure callback set_files(TextInfo);

        scroll := ScrollView {
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
