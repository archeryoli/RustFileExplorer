use std::fs::DirEntry;

fn main() {
    let mut current_dir = "/home/oliver/".to_string();

    let main_window = MainWindow::new().unwrap();
    let mut files: Vec<TextInfo> = Vec::new();
    match read_directory(&current_dir) {
        Ok(entries) => {
            for entry in &entries {
                let filename = entry.file_name().into_string().unwrap().into();
                let is_dir = entry.file_type().unwrap().is_dir();
                let text_info = TextInfo {
                    filename,
                    is_dir,
                };
                files.push(text_info);
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
    files.sort_by(|a: &TextInfo, b: &TextInfo| {
        let a_is_dir = a.is_dir;
        let b_is_dir = b.is_dir;
        if a_is_dir && !b_is_dir {
            return std::cmp::Ordering::Less;
        } else if !a_is_dir && b_is_dir {
            return std::cmp::Ordering::Greater;
        }
        a.filename.cmp(&b.filename)
    });
    let file_models = std::rc::Rc::new(slint::VecModel::from(files));
    main_window.set_files(file_models.into());

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
    }
    export component MainWindow inherits Window {
        width: 600px;
        height: 400px;
        title: "File Explorer";

        in property <[TextInfo]> files: [
        ];
        in property <int> vp_height: 0;

        in property <length> file_paddings_left: 10px;
        ScrollView {
            viewport-height: files.length * 20px;
            
            for file[i] in files : TouchArea {
                width: parent.width;
                height: 20px;
                x: file_paddings_left;
                y: i * 20px;

                text:= Text {
                    text: file.filename;
                    color: file.is_dir ? green : blue;
                    x: parent.x;
                }

                clicked => {
                    text.color = #f00;
                }
            }
        }
    }
    
}
