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
        {"attributes":{"altTitles":[{"en":"Comi-san ha Comyusho Desu."},{"en":"Komi Can't Communicate"},{"en":"Komi-san Can't Communicate."},{"en":"Komi-san Has a Communication Disorder."},{"en":"Komi-san wa Komyushou Desu"},{"en":"Komi-san wa, Communication Shougai desu."},{"en":"Komi-san wa, Comyushou desu."},{"en":"Komi-san wa, Komyushou desu."},{"en":"Miss Komi Is Bad at Communication."},{"en":"У Коми-сан проблемы с общением"},{"en":"โฉมงามพูดไม่เก่งกับผองเพื่อนไม่เต็มเต็ง"},{"en":"古見さんは、コミュ症です。"},{"en":"古見同學有交流障礙症"},{"en":"코미 양은, 커뮤증이에요"}],"contentRating":"safe","createdAt":"2018-11-22T23:31:37+00:00","description":{"en":"Komi-san is a beautiful and admirable girl that no one can take their eyes off of. Almost the whole school sees her as the cold beauty that's out of their league, but Tadano Hitohito knows the truth: she's just really bad at communicating with others. \r\n\r\nKomi-san, who wishes to fix this bad habit of hers, tries to improve it with the help of Tadano-kun by achieving her goal of having 100 friends.\r\n\r\n[b]Links:[/b] [url=https://twitter.com/ooodaaaatooo]Author's Twitter[/url]\r\n\r\nComi-san ha Comyusho Desu. is the official romanization.\r\n\r\nPL\r\n\r\n[spoiler]Komi-san jest piękną i godną podziwu dziewczyną, od której nikt nie może oderwać oczu. Prawie cała szkoła postrzega ją jako zimne piękno, które jest poza ich zasięgiem, ale Tadano Hitohito zna prawdę: młoda piękność po prostu źle komunikuje się z innymi. Komi-san, chce to zmienić, a ma jej w tym pomóc Tadano.[/spoiler]"},"lastChapter":null,"lastVolume":null,"links":{"al":"97852","amz":"https://www.amazon.co.jp/gp/product/B07CBD8DKM","ap":"komi-cant-communicate","bw":"series/129153","cdj":"http://www.cdjapan.co.jp/product/NEOBK-1985640","dj":"958365","ebj":"https://www.ebookjapan.jp/ebj/382444/","engtl":"https://www.viz.com/komi-can-t-communicate","kt":"37855","mal":"99007","mu":"127281","raw":"https://websunday.net/rensai/komisan/"},"originalLanguage":"ja","publicationDemographic":"shounen","status":"ongoing","tags":[{"attributes":{"description":[],"group":"genre","name":{"en":"Romance"},"version":1},"id":"423e2eae-a7a2-4a8b-ac03-a8351462d71d","type":"tag"},{"attributes":{"description":[],"group":"genre","name":{"en":"Comedy"},"version":1},"id":"4d32cc48-9f00-4cca-9b5a-a839f0764984","type":"tag"},{"attributes":{"description":[],"group":"theme","name":{"en":"School Life"},"version":1},"id":"caaa44eb-cd40-4177-b930-79d3ef2afe87","type":"tag"},{"attributes":{"description":[],"group":"genre","name":{"en":"Slice of Life"},"version":1},"id":"e5301a23-ebd9-49dd-a0cb-2add944c7fe9","type":"tag"}],"title":{"en":"Komi-san wa Komyushou Desu."},"updatedAt":"2021-05-25T15:46:17+00:00","version":2,"year":null},"id":"a96676e5-8ae2-425e-b549-7f15dd34a6d8","type":"manga"}
    "#;
    let manga: Manga = serde_json::from_str(&json_str).unwrap();
    let title: &str = "Komi-san wa Komyushou Desu.";
    let author: &str = "";
    assert_eq!(manga.title["en"], title);
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