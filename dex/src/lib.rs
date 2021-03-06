mod chapter;
mod chapter_list;
mod cover;
mod cover_list;
mod creator;
mod creator_list;
mod error;
mod manga;
mod manga_list;
mod utils;

use std::collections::HashMap;

pub use chapter::Chapter;
pub use chapter_list::ChapterList;
pub use cover::Cover;
pub use cover_list::CoverList;
pub use creator::CreatorTemplate as Author;
pub use creator_list::CreatorList as AuthorList;
pub use manga::Manga;
pub use manga_list::MangaList;

pub use chapter::request::CqueryParams as ParamSet;
pub(crate) use self::ParamType as DynParam;
pub use error::ErrorList as DexErrorList;
pub use utils::{DexWrappedObject, DexError};

pub enum ParamType<'a> {
    String(Option<&'a str>),
    Array(Vec<String>),
    Integer(Option<i32>)
}

pub(crate) async fn get_data(uri: &str) -> Result<String, reqwest::Error> {
    let json_text = reqwest::get(uri).await?.text().await?;

    Ok(json_text)
}

pub(crate) fn parse_url(query: &str, query_params: HashMap<&str, DynParam<'_>>) -> String {
    let mut url = format!("{}?", query);

    for (k, v) in query_params.iter() {
        match v {
            DynParam::String(v)=> {
                if let Some(v) = v {
                    url = format!("{}{}={}&", url, k, v);
                }
            }
            DynParam::Array(v) => {
                for v in v.iter() {
                    url = format!("{}{}[]={}&", url, k, v);
                }
            }
            DynParam::Integer(v) => {
                if let Some(v) = v {
                    url = format!("{}{}={}&", url, k, v);
                }
            }
        } 
    }

    url
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashMap;

//     #[test]
//     fn url_parsing_1() {
//         let mut query_params = HashMap::new();
//         query_params.insert("limit", Some("2"));
//         assert_eq!(
//             "https://api.mangadex.org/manga?limit=2&",
//             parse_url("https://api.mangadex.org/manga", query_params)
//         );
//     }

//     #[test]
//     fn url_parsing_2() {
//         let mut query_params = HashMap::new();
//         query_params.insert("limit", Some("2"));
//         query_params.insert("offset", Some("5"));
//         assert_eq!(
//             "https://api.mangadex.org/manga?offset=5&limit=2&",
//             parse_url("https://api.mangadex.org/manga", query_params)
//         );
//     }

//     #[test]
//     fn query_checker() {
//         let mut query_params = HashMap::new();
//         query_params.insert("limit", "2");
//         query_params.insert("offset", "5");
//         let uri = parse_url("https://api.mangadex.org/manga", query_params);
//         let text = match get_data(&uri) {
//             Ok(s) => s,
//             Err(_) => String::from(""),
//         };
//         assert_eq!(text, "{\"results\":[{\"result\":\"ok\",\"data\":{\"id\":\"eb2d1a45-d4e7-4e32-a171-b5b029c5b0cb\",\"type\":\"manga\",\"attributes\":{\"title\":{\"en\":\"Kumo Desu ga, Nani ka?\"},\"altTitles\":[{\"en\":\"\\u00d6r\\u00fcmce\\u011fim ben, ne olmu\\u015f?\"},{\"en\":\"A&Aring;&iexcl; voras, ir kas i&Aring;&iexcl; to?\"},{\"en\":\"Aku ini laba laba, lalu apa ?\"},{\"en\":\"Jag &auml;r en spindel, vad&aring; d&aring;?\"},{\"en\":\"Jeg er en edderkopp, hva s&aring;?\"},{\"en\":\"kumoko\"},{\"en\":\"No i co z tego \\u017ce jestem paj\\u0105kiem?\"},{\"en\":\"So I'm a Spider, So What?\"},{\"en\":\"Sono un ragno, e quindi?\"},{\"en\":\"Soy una ara&ntilde;a, y que?\"},{\"en\":\"\\u0414\\u0430, \\u044f \\u043f\\u0430\\u0443\\u043a, \\u0438 \\u0447\\u0442\\u043e \\u0436\\u0435?\"},{\"en\":\"\\u0414\\u0430, \\u044f \\u043f\\u0430\\u0443\\u043a, \\u0438 \\u0447\\u0442\\u043e \\u0441 \\u0442\\u043e\\u0433\\u043e?\"},{\"en\":\"\\u0623\\u0646\\u0627 \\u0639\\u0646\\u0643\\u0628\\u0648\\u062a\\u060c \\u0648\\u0645\\u0627\\u0630\\u0627 \\u0641\\u064a \\u0630\\u0644\\u0643\\u061f\"},{\"en\":\"\\u0645\\u0646 \\u06cc\\u06a9 \\u0639\\u0646\\u06a9\\u0628\\u0648\\u062a\\u0645. \\u062e\\u0628 \\u06a9\\u0647 \\u0686\\u06cc\\u061f\"},{\"en\":\"\\u0e41\\u0e21\\u0e07\\u0e21\\u0e38\\u0e21\\u0e41\\u0e25\\u0e49\\u0e27\\u0e44\\u0e07, \\u0e02\\u0e49\\u0e2d\\u0e07\\u0e43\\u0e08\\u0e40\\u0e2b\\u0e23\\u0e2d\\u0e04\\u0e30?\"},{\"en\":\"\\u4e0d\\u8fc7\\u662f\\u8718\\u86db\\u4ec0\\u4e48\\u7684\"},{\"en\":\"\\u8718\\u86db\\u3067\\u3059\\u304c\\u3001\\u306a\\u306b\\u304b\\uff1f\"},{\"en\":\"\\uac70\\ubbf8\\uc785\\ub2c8\\ub2e4\\ub9cc, \\ubb38\\uc81c\\ub77c\\ub3c4?\"}],\"description\":{\"en\":\"When a mysterious explosion killed an entire class full of high school students, the souls of everyone in class were transported into a fantasy world and reincarnated. While some students were reincarnated as princes or prodigies, others were not as blessed.\\r\\nOur heroine, who was the lowest in the class, discovered that she was reincarnated as a spider! Now at the bottom of the food chain, she needs to adapt to the current situation with willpower in order to live. Stuck in a dangerous labyrinth filled with monsters, it's eat or be eaten!\\r\\nThis is the story of a spider doing whatever she can in order to survive!\\r\\n\\r\\n[hr][b][u]Russian \\/ \\u0420\\u0443\\u0441\\u0441\\u043a\\u0438\\u0439:[\\/u][\\/b]\\r\\n[spoiler]\\u0412 \\u0440\\u0435\\u0437\\u0443\\u043b\\u044c\\u0442\\u0430\\u0442\\u0435 \\u043d\\u0435\\u043e\\u0431\\u044a\\u044f\\u0441\\u043d\\u0438\\u043c\\u043e\\u0433\\u043e \\u043f\\u0440\\u043e\\u0438\\u0441\\u0448\\u0435\\u0441\\u0442\\u0432\\u0438\\u044f \\u0441 \\u0443\\u0447\\u0430\\u0441\\u0442\\u0438\\u0435\\u043c \\u043f\\u0438\\u043a\\u0441\\u0435\\u043b\\u0435\\u0439 \\u043a\\u043b\\u0430\\u0441\\u0441 \\u043e\\u0431\\u044b\\u0447\\u043d\\u044b\\u0445 \\u044f\\u043f\\u043e\\u043d\\u0441\\u043a\\u0438\\u0445 \\u0448\\u043a\\u043e\\u043b\\u044c\\u043d\\u0438\\u043a\\u043e\\u0432 \\u043f\\u043e\\u0433\\u0438\\u0431\\u0430\\u0435\\u0442, \\u043d\\u043e \\u043f\\u0435\\u0440\\u0435\\u0440\\u043e\\u0436\\u0434\\u0430\\u0435\\u0442\\u0441\\u044f \\u0432 \\u0434\\u0440\\u0443\\u0433\\u043e\\u043c \\u043c\\u0438\\u0440\\u0435. \\u0410 \\u043d\\u0430\\u0448\\u0435\\u0439 \\u0433\\u0435\\u0440\\u043e\\u0438\\u043d\\u0435, \\u043a\\u043e\\u0442\\u043e\\u0440\\u0430\\u044f \\u0431\\u044b\\u043b\\u0430 \\u0441\\u0430\\u043c\\u043e\\u0439 \\u043d\\u0435\\u043f\\u043e\\u043f\\u0443\\u043b\\u044f\\u0440\\u043d\\u043e\\u0439 \\u0432 \\u043a\\u043b\\u0430\\u0441\\u0441\\u0435, \\u043f\\u043e\\u0432\\u0435\\u0437\\u043b\\u043e \\u043a\\u0443\\u0434\\u0430 \\u0431\\u043e\\u043b\\u044c\\u0448\\u0435. \\u041e\\u043d\\u0430 \\u043f\\u0435\\u0440\\u0435\\u0440\\u043e\\u0434\\u0438\\u043b\\u0430\\u0441\\u044c \\u043f\\u0430\\u0443\\u043a\\u043e\\u043c \\u0432 \\u043f\\u043e\\u0434\\u0437\\u0435\\u043c\\u0435\\u043b\\u044c\\u0435, \\u043f\\u043e\\u043b\\u043d\\u043e\\u043c \\u043e\\u043f\\u0430\\u0441\\u043d\\u043e\\u0441\\u0442\\u0435\\u0439. \\u0421\\u043c\\u043e\\u0436\\u0435\\u0442 \\u043b\\u0438 \\u043e\\u043d\\u0430, \\u0438\\u0441\\u043f\\u043e\\u043b\\u044c\\u0437\\u0443\\u044f \\u043b\\u0438\\u0448\\u044c \\u0441\\u0432\\u043e\\u044e \\u0445\\u0438\\u0442\\u0440\\u043e\\u0441\\u0442\\u044c \\u0438 \\u0443\\u0434\\u0430\\u0447\\u0443, \\u0432\\u044b\\u0436\\u0438\\u0442\\u044c?[\\/spoiler]\"},\"isLocked\":false,\"links\":{\"al\":\"86952\",\"ap\":\"so-im-a-spider-so-what\",\"bw\":\"series\\/75188\",\"kt\":\"37173\",\"mu\":\"129680\",\"nu\":\"kumo-desu-ga-nani-ka\",\"amz\":\"https:\\/\\/www.amazon.co.jp\\/gp\\/product\\/B0753HFC1X\",\"cdj\":\"http:\\/\\/www.cdjapan.co.jp\\/product\\/NEOBK-1963958\",\"ebj\":\"https:\\/\\/www.ebookjapan.jp\\/ebj\\/370713\",\"mal\":\"95353\",\"raw\":\"https:\\/\\/web-ace.jp\\/youngaceup\\/contents\\/1000013\",\"engtl\":\"https:\\/\\/yenpress.com\\/series-search\\/?series=yen23-So-Im-a-Spider-So-What-Manga\"},\"originalLanguage\":\"ja\",\"lastVolume\":\"6\",\"lastChapter\":\"30.2\",\"publicationDemographic\":\"seinen\",\"status\":\"ongoing\",\"year\":null,\"contentRating\":\"safe\",\"tags\":[{\"id\":\"0bc90acb-ccc1-44ca-a34a-b9f3a73259d0\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Reincarnation\"},\"description\":[],\"group\":\"theme\",\"version\":1}},{\"id\":\"36fd93ea-e8b8-445e-b836-358f02b3d33d\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Monsters\"},\"description\":[],\"group\":\"theme\",\"version\":1}},{\"id\":\"391b0423-d847-456f-aff0-8b0cfc03066b\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Action\"},\"description\":[],\"group\":\"genre\",\"version\":1}},{\"id\":\"39730448-9a5f-48a2-85b0-a70db87b1233\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Demons\"},\"description\":[],\"group\":\"theme\",\"version\":1}},{\"id\":\"4d32cc48-9f00-4cca-9b5a-a839f0764984\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Comedy\"},\"description\":[],\"group\":\"genre\",\"version\":1}},{\"id\":\"5fff9cde-849c-4d78-aab0-0d52b2ee1d25\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Survival\"},\"description\":[],\"group\":\"theme\",\"version\":1}},{\"id\":\"87cc87cd-a395-47af-b27a-93258283bbc6\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Adventure\"},\"description\":[],\"group\":\"genre\",\"version\":1}},{\"id\":\"a1f53773-c69a-4ce5-8cab-fffcd90b1565\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Magic\"},\"description\":[],\"group\":\"theme\",\"version\":1}},{\"id\":\"ace04997-f6bd-436e-b261-779182193d3d\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Isekai\"},\"description\":[],\"group\":\"genre\",\"version\":1}},{\"id\":\"b9af3a63-f058-46de-a9a0-e0c13906197a\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Drama\"},\"description\":[],\"group\":\"genre\",\"version\":1}},{\"id\":\"cdc58593-87dd-415e-bbc0-2ec27bf404cc\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Fantasy\"},\"description\":[],\"group\":\"genre\",\"version\":1}},{\"id\":\"dd1f77c5-dea9-4e2b-97ae-224af09caf99\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Monster Girls\"},\"description\":[],\"group\":\"theme\",\"version\":1}},{\"id\":\"f4122d1c-3b44-44d0-9936-ff7502c39ad3\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Adaptation\"},\"description\":[],\"group\":\"content\",\"version\":1}}],\"createdAt\":\"2018-12-12T11:03:14+00:00\",\"updatedAt\":\"2021-05-25T15:49:15+00:00\",\"version\":2}},\"relationships\":[{\"id\":\"7be584f3-1c1a-48ce-9205-4e5c7edc3fad\",\"type\":\"author\"},{\"id\":\"6a8a1bc6-b567-4b76-808f-b05861101308\",\"type\":\"artist\"},{\"id\":\"bd27837a-666e-4fa2-a8cb-c9c7079ff76b\",\"type\":\"cover_art\"}]},{\"result\":\"ok\",\"data\":{\"id\":\"d86cf65b-5f6c-437d-a0af-19a31f94ec55\",\"type\":\"manga\",\"attributes\":{\"title\":{\"en\":\"Ijiranaide, Nagatoro-san\"},\"altTitles\":[{\"en\":\"Don't Toy With Me, Miss Nagatoro\"},{\"en\":\"Jangan goda aku, Nagatoro\"},{\"en\":\"No te burles de mi, Nagatoro\"},{\"en\":\"Please Don't Bully Me, Nagatoro\"},{\"en\":\"S'il te pla&icirc;t ne me harc\\u00e8le pas, Nagatoro\"},{\"en\":\"\\u041d\\u0435 \\u0438\\u0437\\u0434\\u0435\\u0432\\u0430\\u0439\\u0441\\u044f, \\u041d\\u0430\\u0433\\u0430\\u0442\\u043e\\u0440\\u043e-\\u0441\\u0430\\u043d\"},{\"en\":\"\\u30a4\\u30b8\\u3089\\u306a\\u3044\\u3067\\u3001\\u9577\\u701e\\u3055\\u3093\"},{\"en\":\"\\uad34\\ub86d\\ud788\\uc9c0 \\ub9d0\\uc544\\uc694, \\ub098\\uac00\\ud1a0\\ub85c \\uc591\"}],\"description\":{\"en\":\"Our nameless&mdash;and spineless&mdash;hero (only known as &quot;Senpai&quot;) is a second year high school student and loner who spends his afternoons at the Arts Club room. He attracts the attention of one of his schoolmates, a sadistic freshman girl named Nagatoro. However, in between the bullying and teasing, something else begins to blossom. A lovey dovey(...?) story between a shy nerd and an S-Dere (Sadistic Tsundere) begins.\\r\\n\\r\\n[u][b]Russian \\/ \\u0420\\u0443\\u0441\\u0441\\u043a\\u0438\\u0439:[\\/b][\\/u]  [spoiler]\\u041d\\u0430\\u0433\\u0430\\u0442\\u043e\\u0440\\u043e - \\u043f\\u0435\\u0440\\u0435\\u0432\\u0435\\u0434\\u0451\\u043d\\u043d\\u0430\\u044f \\u0443\\u0447\\u0435\\u043d\\u0438\\u0446\\u0430 \\u0432 \\u0441\\u0442\\u0430\\u0440\\u0448\\u0435\\u0439 \\u0448\\u043a\\u043e\\u043b\\u0435, \\u043a\\u043e\\u0442\\u043e\\u0440\\u043e\\u0439 \\u043d\\u0440\\u0430\\u0432\\u0438\\u0442\\u0441\\u044f \\u0434\\u0440\\u0430\\u0437\\u043d\\u0438\\u0442\\u044c \\u0441\\u0432\\u043e\\u0435\\u0433\\u043e \\u0441\\u0435\\u043c\\u043f\\u0430\\u044f. \\u041d\\u043e \\u043e\\u043d \\u0432\\u0441\\u0451 \\u0441\\u0442\\u0435\\u0440\\u043f\\u0438\\u0442. \\u0418 \\u043f\\u0443\\u0441\\u0442\\u044c \\u0435\\u043c\\u0443 \\u043f\\u0440\\u0438\\u0434\\u0451\\u0442\\u0441\\u044f \\u043f\\u0440\\u043e\\u0439\\u0442\\u0438 \\u0447\\u0435\\u0440\\u0435\\u0437 \\u0432\\u0441\\u0435 \\u0432\\u0438\\u0434\\u044b \\u0438\\u0437\\u0434\\u0435\\u0432\\u0430\\u0442\\u0435\\u043b\\u044c\\u0441\\u0442\\u0432 - \\u043e\\u043d \\u0432\\u0441\\u0451 \\u043f\\u0440\\u043e\\u0441\\u0442\\u0438\\u0442, \\u043f\\u043e\\u0442\\u043e\\u043c\\u0443 \\u0447\\u0442\\u043e \\u0432\\u043b\\u044e\\u0431\\u043b\\u0451\\u043d. [\\/spoiler]\\r\\n[u][b]Czech \\/ \\u010cesky:[\\/b][\\/u]  [spoiler]Nagatoro je st\\u0159edo&scaron;kola\\u010dka, kter&aacute; r&aacute;da &scaron;k&aacute;dl&iacute; a tr&aacute;p&iacute; jej&iacute;ho star&scaron;&iacute;ho spolu\\u017e&aacute;ka! Jak&aacute; je jej&iacute; motivace a pro\\u010d si to Senpai nech&aacute; l&iacute;bit? Chce Nagatoro ud\\u011blat ze Senpaiova \\u017eivota peklo? Nebo se j&iacute; tajn\\u011b l&iacute;b&iacute;? [\\/spoiler]\\r\\n[hr][b]Links:[\\/b]\\r\\n[*][url=https:\\/\\/twitter.com\\/774nanash]Author's Twitter[\\/url]\\r\\n[*][url=https:\\/\\/www.animenewsnetwork.com\\/encyclopedia\\/anime.php?id=23755]Anime on ANN.[\\/url]\\r\\n[*][url=https:\\/\\/youtu.be\\/Bw5jwgdbqKc]Anime Official Trailer (2nd PV)[\\/url]\\r\\n[hr][b]Officially Published:[\\/b]\\r\\n[*]In French by [url=https:\\/\\/noeve.com\\/products\\/arrete-de-me-chauffer-nagatoro-tome-1]Noeve Grafx[\\/url]\"},\"isLocked\":false,\"links\":{\"al\":\"100664\",\"ap\":\"dont-toy-with-me-miss-nagatoro\",\"bw\":\"series\\/155089\",\"kt\":\"40324\",\"mu\":\"145530\",\"amz\":\"https:\\/\\/www.amazon.co.jp\\/gp\\/product\\/B07F7QD132\",\"cdj\":\"http:\\/\\/www.cdjapan.co.jp\\/product\\/NEOBK-2193546\",\"ebj\":\"https:\\/\\/www.ebookjapan.jp\\/ebj\\/443422\",\"mal\":\"110737\",\"raw\":\"https:\\/\\/pocket.shonenmagazine.com\\/episode\\/13932016480029136187\",\"engtl\":\"https:\\/\\/kodanshacomics.com\\/series\\/dont-toy-with-me-miss-nagatoro\"},\"originalLanguage\":\"ja\",\"lastVolume\":\"4\",\"lastChapter\":\"27.5\",\"publicationDemographic\":\"shounen\",\"status\":\"ongoing\",\"year\":null,\"contentRating\":\"safe\",\"tags\":[{\"id\":\"423e2eae-a7a2-4a8b-ac03-a8351462d71d\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Romance\"},\"description\":[],\"group\":\"genre\",\"version\":1}},{\"id\":\"4d32cc48-9f00-4cca-9b5a-a839f0764984\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Comedy\"},\"description\":[],\"group\":\"genre\",\"version\":1}},{\"id\":\"caaa44eb-cd40-4177-b930-79d3ef2afe87\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"School Life\"},\"description\":[],\"group\":\"theme\",\"version\":1}},{\"id\":\"e5301a23-ebd9-49dd-a0cb-2add944c7fe9\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Slice of Life\"},\"description\":[],\"group\":\"genre\",\"version\":1}},{\"id\":\"fad12b5e-68ba-460e-b933-9ae8318f5b65\",\"type\":\"tag\",\"attributes\":{\"name\":{\"en\":\"Gyaru\"},\"description\":[],\"group\":\"theme\",\"version\":1}}],\"createdAt\":\"2019-03-04T12:21:59+00:00\",\"updatedAt\":\"2021-05-25T15:48:19+00:00\",\"version\":2}},\"relationships\":[{\"id\":\"7e552c08-f7cf-4e0e-9723-409d749dd77c\",\"type\":\"author\"},{\"id\":\"7e552c08-f7cf-4e0e-9723-409d749dd77c\",\"type\":\"artist\"},{\"id\":\"65163395-201c-4f5a-b303-706f32bf2df4\",\"type\":\"cover_art\"}]}],\"limit\":2,\"offset\":5,\"total\":10000}");
//     }
// }