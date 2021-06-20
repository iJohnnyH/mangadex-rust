use mangadex_rust::types::*;

#[test]
fn test_author() {
    let json_str = r#"
    
    "#;

    let author: Author = serde_json::from_str(&json_str).unwrap();
    let name: &str = "";
    let id: &str = "";
    assert_eq!(author.name, name);
    assert_eq!(author.id, id);
}

#[test]
fn test_get_manga() {
    let json_str = r#"
    
    "#;

    let manga: Manga = serde_json::from_str(&json_str).unwrap();
    let title: &str = "";
    let author: &str = "";
    assert_eq!(manga.title, title);
    assert_eq!(manga.author.name, author);
}

#[test]
fn test_get_tag() {
    let json_str = r#"
    
    "#;

    let tag: MangaTag = serde_json::from_str(&json_str).unwrap();
    let desc: &str = "";
    let name: &str = "";
    assert_eq!(tag.desc, desc);
    assert_eq!(tag.name, name);
}