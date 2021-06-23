use mangadex_rust::types::*;
use serde_json::{Value};


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
fn test_get_manga() ->  serde_json::Result<()>{
    let json_str = r#"
        {"result": "ok","data": {"id": "a96676e5-8ae2-425e-b549-7f15dd34a6d8","type": "manga","attributes": {"title": {"en": "Komi-san wa Komyushou Desu."},"altTitles": [{"en": "Comi-san ha Comyusho Desu."},{"en": "Komi Can't Communicate"},{"en": "Komi-san Can't Communicate."},{"en": "Komi-san Has a Communication Disorder."},{"en": "Komi-san wa Komyushou Desu"},{"en": "Komi-san wa, Communication Shougai desu."},{"en": "Komi-san wa, Comyushou desu."},{"en": "Komi-san wa, Komyushou desu."},{"en": "Miss Komi Is Bad at Communication."},{"en": "У Коми-сан проблемы с общением"},{"en": "โฉมงามพูดไม่เก่งกับผองเพื่อนไม่เต็มเต็ง"},{"en": "古見さんは、コミュ症です。"},{"en": "古見同學有交流障礙症"},{"en": "코미 양은, 커뮤증이에요"}],"description": {"en": "Komi-san is a beautiful and admirable girl that no one can take their eyes off of. Almost the whole school sees her as the cold beauty that's out of their league, but Tadano Hitohito knows the truth: she's just really bad at communicating with others. \r\n\r\nKomi-san, who wishes to fix this bad habit of hers, tries to improve it with the help of Tadano-kun by achieving her goal of having 100 friends.\r\n\r\n[b]Links:[/b] [url=https://twitter.com/ooodaaaatooo]Author's Twitter[/url]\r\n\r\nComi-san ha Comyusho Desu. is the official romanization.\r\n\r\nPL\r\n\r\n[spoiler]Komi-san jest piękną i godną podziwu dziewczyną, od której nikt nie może oderwać oczu. Prawie cała szkoła postrzega ją jako zimne piękno, które jest poza ich zasięgiem, ale Tadano Hitohito zna prawdę: młoda piękność po prostu źle komunikuje się z innymi. Komi-san, chce to zmienić, a ma jej w tym pomóc Tadano.[/spoiler]"},"links": {"al": "97852","ap": "komi-cant-communicate","bw": "series/129153","dj": "958365","kt": "37855","mu": "127281","amz": "https://www.amazon.co.jp/gp/product/B07CBD8DKM","cdj": "http://www.cdjapan.co.jp/product/NEOBK-1985640","ebj": "https://www.ebookjapan.jp/ebj/382444/","mal": "99007","raw": "https://websunday.net/rensai/komisan/","engtl": "https://www.viz.com/komi-can-t-communicate"},"originalLanguage": "ja","lastVolume": null,"lastChapter": null,"publicationDemographic": "shounen","status": "ongoing","year": null,"contentRating": "safe","tags": [{"id": "423e2eae-a7a2-4a8b-ac03-a8351462d71d","type": "tag","attributes": {"name": {"en": "Romance"},"description": [],"group": "genre","version": 1}},{"id": "4d32cc48-9f00-4cca-9b5a-a839f0764984","type": "tag","attributes": {"name": {"en": "Comedy"},"description": [],"group": "genre","version": 1}},{"id": "caaa44eb-cd40-4177-b930-79d3ef2afe87","type": "tag","attributes": {"name": {"en": "School Life"},"description": [],"group": "theme","version": 1}},{"id": "e5301a23-ebd9-49dd-a0cb-2add944c7fe9","type": "tag","attributes": {"name": {"en": "Slice of Life"},"description": [],"group": "genre","version": 1}}],"createdAt": "2018-11-22T23:31:37+00:00","updatedAt": "2021-05-25T15:46:17+00:00","version": 2}},"relationships": [{"id": "4218b1ee-cde4-44dc-84c7-d9a794a7e56d","type": "author"},{"id": "4218b1ee-cde4-44dc-84c7-d9a794a7e56d","type": "artist"},{"id": "3730193d-a00f-4f67-a3c8-92118d857c31","type": "cover_art"}]}
    "#;
    let v: Value = serde_json::from_str(json_str)?;
    let manga: Manga = Manga::from(v);
    let title: &str = "Komi-san wa Komyushou Desu.";
    let author: &str = "";
    assert_eq!(manga.title["en"], title);
    Ok(())
    // assert_eq!(manga.author.name, author);
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