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



        in property <[TextInfo]> files: [
        ];
        in property <int> vp_height: 0;
        ScrollView {
            width: 600px;
            height: 400px;
            viewport-width: 600px;
            viewport-height: files.length * 20px;
            
            for file[i] in files : Text {
                text: file.filename;
                color: green;
                x: 20px;
                y: i * 20px;
            }
        }
    }
    
}
