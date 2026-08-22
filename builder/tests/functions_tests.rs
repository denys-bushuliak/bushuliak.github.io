use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use tempfile::tempdir;
use builder::functions::{read_directory, convert_to_html, save_to_disk};
use builder::{MarkdownFile, HtmlFile};

#[test]
fn test_read_directory_recursive() {
    let dir = tempdir().unwrap();
    let sub_dir = dir.path().join("sub");
    fs::create_dir(&sub_dir).unwrap();
    
    let file1 = dir.path().join("file1.md");
    let file2 = sub_dir.join("file2.md");
    let file3 = dir.path().join("file3.txt"); // Should be ignored
    let hidden = dir.path().join(".hidden.md"); // Should be ignored
    
    fs::write(&file1, "content1").unwrap();
    fs::write(&file2, "content2").unwrap();
    fs::write(&file3, "content3").unwrap();
    fs::write(&hidden, "hidden").unwrap();
    
    let results: Vec<_> = read_directory(dir.path().to_path_buf()).unwrap().collect();
    
    assert_eq!(results.len(), 2);
    
    let paths: Vec<_> = results.iter().map(|(p, _)| p.clone()).collect();
    assert!(paths.contains(&file1));
    assert!(paths.contains(&file2));
}

#[test]
fn test_convert_to_html() {
    let input_dir = PathBuf::from("input");
    let output_dir = PathBuf::from("output");
    let layout = "<html><body><!--REPLACE_ME_BY_CONTENT--></body></html>";
    let placeholder = "<!--REPLACE_ME_BY_CONTENT-->";
    
    let md_path = input_dir.join("sub/post.md");
    let md_content = "# Title\nHello";
    let md_file = MarkdownFile::create(md_path, md_content.to_string());
    
    let converter = convert_to_html(&output_dir, layout, placeholder, &input_dir);
    let html_file = converter(md_file);
    
    println!("Path to save: {:?}", html_file.path_to_save);
    assert!(html_file.path_to_save.to_string_lossy().ends_with("sub/post.html"));
    assert!(html_file.content.contains("<h1>Title</h1>"));
    assert!(html_file.content.contains("<p>Hello</p>"));
    assert!(html_file.content.contains("<html><body>"));
}

#[test]
fn test_convert_to_html_injects_page_meta() {
    let input_dir = PathBuf::from("input");
    let output_dir = PathBuf::from("output");
    let layout = "<html><head><title><!--PAGE_TITLE--></title>\
                   <meta name=\"description\" content=\"<!--PAGE_DESCRIPTION-->\"></head>\
                   <body><!--REPLACE_ME_BY_CONTENT--></body></html>";
    let placeholder = "<!--REPLACE_ME_BY_CONTENT-->";

    let md_path = input_dir.join("skills.md");
    let md_file = MarkdownFile::create(md_path, "Content".to_string());

    let converter = convert_to_html(&output_dir, layout, placeholder, &input_dir);
    let html_file = converter(md_file);

    assert!(html_file.content.contains("<title>Skills | Denys Bushuliak</title>"));
    assert!(html_file.content.contains(
        "content=\"Technical skills: Rust, Go, JavaScript, CQRS, microservices, DDD, databases, and cloud.\""
    ));
    assert!(!html_file.content.contains("PAGE_TITLE"));
    assert!(!html_file.content.contains("PAGE_DESCRIPTION"));
}

#[test]
fn test_page_meta_index_keeps_main_site_title() {
    let meta = builder::PageMeta::from_file_stem("index");
    assert_eq!(
        meta.title,
        "Denys Bushuliak | Principal Software Engineer"
    );
}

#[test]
fn test_page_meta_known_pages_get_descriptions() {
    let meta = builder::PageMeta::from_file_stem("recommendation_letters");
    assert_eq!(meta.title, "Recommendation Letters | Denys Bushuliak");
    assert_eq!(
        meta.description,
        "Letters of recommendation from past employers, with downloadable PDFs."
    );
}

#[test]
fn test_page_meta_unknown_stem_falls_back_to_humanized_title() {
    let meta = builder::PageMeta::from_file_stem("my_writing");
    assert_eq!(meta.title, "My Writing | Denys Bushuliak");
    assert_eq!(meta.description, meta.title);
}

#[test]
fn test_markdown_file_from_tuple() {
    let path = PathBuf::from("test.md");
    let content = "content".to_string();
    let md_file = MarkdownFile::from((path.clone(), content.clone()));
    assert_eq!(md_file.path, path);
    assert_eq!(md_file.content, content);
}

#[test]
fn test_save_to_disk_success() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("out/test.html");
    let content = "<html></html>";
    
    let html_file = HtmlFile::new(file_path.clone(), content.to_string());
    save_to_disk(html_file).unwrap();
    
    assert!(file_path.exists());
    assert_eq!(fs::read_to_string(file_path).unwrap(), content);
}

#[test]
fn test_save_to_disk_error_no_parent() {
    let file_path = PathBuf::from("/");
    let html_file = HtmlFile::new(file_path, "content".to_string());
    let result = save_to_disk(html_file);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Cannot determine parent directory"));
}

#[test]
fn test_save_to_disk_io_error() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("read_only_dir/test.html");
    fs::create_dir(dir.path().join("read_only_dir")).unwrap();
    
    // Make directory read-only to trigger IO error
    let mut perms = fs::metadata(dir.path().join("read_only_dir")).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(dir.path().join("read_only_dir"), perms).unwrap();
    
    let html_file = HtmlFile::new(file_path, "content".to_string());
    let result = save_to_disk(html_file);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("IO error"));
    
    // Cleanup: make it writable again so tempdir can delete it
    let mut perms = fs::metadata(dir.path().join("read_only_dir")).unwrap().permissions();
    perms.set_readonly(false);
    fs::set_permissions(dir.path().join("read_only_dir"), perms).unwrap();
}

#[test]
fn test_run_success() {
    let dir = tempdir().unwrap();
    let input_dir = dir.path().join("in");
    let output_dir = dir.path().join("out");
    fs::create_dir(&input_dir).unwrap();
    fs::write(input_dir.join("test.md"), "# Hello").unwrap();
    
    let template_path = "template.html";
    let original_template = if std::path::Path::new(template_path).exists() {
        Some(fs::read_to_string(template_path).unwrap())
    } else {
        None
    };
    
    fs::write(template_path, "<html><body><!--REPLACE_ME_BY_CONTENT--></body></html>").unwrap();
    
    let args = builder::ValidatedArgsDto {
        input_directory: input_dir,
        output_directory: output_dir.clone(),
    };
    
    builder::run(args).unwrap();
    
    assert!(output_dir.join("test.html").exists());
    
    // Restore template
    if let Some(content) = original_template {
        fs::write(template_path, content).unwrap();
    } else {
        fs::remove_file(template_path).unwrap();
    }
}

#[test]
fn test_run_missing_template() {
    let dir = tempdir().unwrap();
    let template_path = "template.html";
    let original_template = if std::path::Path::new(template_path).exists() {
        Some(fs::read_to_string(template_path).unwrap())
    } else {
        None
    };
    
    if std::path::Path::new(template_path).exists() {
        fs::remove_file(template_path).unwrap();
    }
    
    let args = builder::ValidatedArgsDto {
        input_directory: dir.path().to_path_buf(),
        output_directory: dir.path().to_path_buf(),
    };
    
    let result = builder::run(args);
    assert!(result.is_err());
    
    // Restore template
    if let Some(content) = original_template {
        fs::write(template_path, content).unwrap();
    }
}

#[test]
fn test_recur_read_files_unreadable_dir() {
    let dir = tempdir().unwrap();
    let sub_dir = dir.path().join("unreadable");
    fs::create_dir(&sub_dir).unwrap();
    
    let mut perms = fs::metadata(&sub_dir).unwrap().permissions();
    perms.set_mode(0o000); // No permissions
    let _ = fs::set_permissions(&sub_dir, perms);
    
    let results: Vec<_> = read_directory(dir.path().to_path_buf()).unwrap().collect();
    // Should be empty because sub_dir is unreadable and unwrap_or_default() is used
    assert!(results.is_empty());
    
    // Cleanup
    let mut perms = fs::metadata(&sub_dir).unwrap().permissions();
    perms.set_mode(0o755);
    let _ = fs::set_permissions(&sub_dir, perms);
}

#[test]
fn test_recur_read_files_unreadable_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("unreadable.md");
    fs::write(&file_path, "content").unwrap();
    
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o000);
    let _ = fs::set_permissions(&file_path, perms);
    
    let results: Vec<_> = read_directory(dir.path().to_path_buf()).unwrap().collect();
    assert!(results.is_empty());
    
    // Cleanup
    let mut perms = fs::metadata(&file_path).unwrap().permissions();
    perms.set_mode(0o644);
    let _ = fs::set_permissions(&file_path, perms);
}

#[test]
fn test_read_directory_invalid_path() {
    let result = read_directory(PathBuf::from("/non_existent_path_12345"));
    assert!(result.is_err());
}
