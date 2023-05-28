use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Editor, Result};
use rustyline::history::DefaultHistory;

fn main() -> Result<()> {
  let mut rl = DefaultEditor::new()?;

  let entry = get_entry(&mut rl, "请输入入口地址: ");
  let output = get_entry(&mut rl, "请输入输出地址: ");
  let entry= if entry.is_empty() { "D:\\t1" } else {&entry};
  let output= if output.is_empty() { "D:\\t2" } else {&output};
  let entry_dir = Path::new(entry);
  let output_dir = Path::new(output);
  find_repositories(entry_dir, output_dir);


  println!("拷贝完成");
  // println!("{}", output);
  Ok(())
}

fn get_entry(rl: &mut Editor<(), DefaultHistory>, prompt: &str) -> String {
  let readline = rl.readline(prompt);
  let mut entry = String::from("");
  match readline {
    Ok(line) => {
      entry.push_str(&line);
    },
    Err(ReadlineError::Interrupted) => {
      println!("CTRL-C");
    },
    Err(ReadlineError::Eof) => {
      println!("CTRL-D");
    },
    Err(err) => {
      println!("Error: {:?}", err);
    }
  }
  entry
}


fn find_repositories(source_path: &Path, target_path: &Path) {
  let metadata = fs::metadata(source_path);



  if let Ok(entry) = metadata {
    if entry.is_dir() {
      let entries = fs::read_dir(source_path).unwrap();

      for entry in entries {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let file_name = entry.file_name();
        if let Ok(git_dir) = fs::metadata(source_path.join(".git")) {
          if git_dir.is_dir() {
            copy_git(&*source_path, &target_path.join(file_name.clone()));
          }
        } else {
          find_repositories(&source_path, &target_path.join(file_name.clone()))
        }

        // if entry.file_type()?.is_dir() {
        //
        // }
        println!("{:?}", source_path);
        println!("{:?}", file_name);

      }
    }

  }
}

fn copy_git(entry: &Path, output: &Path) {
  let ent = entry.join(Path::new(".git"));
  let ent2 = output.join(Path::new(".git"));
  fs::create_dir_all(output.join(Path::new(".git"))).expect("TODO: panic message");
  // fs::copy(ent, ent2).expect("TODO: panic message");
  copy_folder(ent.to_str().unwrap(), ent2.to_str().unwrap()).expect("TODO: panic message");
}

fn copy_folder(source: &str, destination: &str) -> io::Result<()> {
  // 创建目标文件夹
  fs::create_dir_all(destination)?;

  // 获取源文件夹中的所有条目（子文件夹和文件）
  let entries = fs::read_dir(source)?;

  // 遍历源文件夹中的所有条目
  for entry in entries {
    let entry = entry?;
    let entry_path = entry.path();

    // 构建目标路径
    let dest_path = std::path::Path::new(destination).join(entry.file_name());

    // 判断条目是文件还是文件夹
    if entry_path.is_file() {
      // 如果是文件，执行文件拷贝
      fs::copy(entry_path, dest_path)?;
    } else if entry_path.is_dir() {
      // 如果是文件夹，递归调用自身进行文件夹拷贝
      copy_folder(entry_path.to_str().unwrap(), dest_path.to_str().unwrap())?;
    }
  }

  Ok(())
}