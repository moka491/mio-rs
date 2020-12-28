use lazy_static::lazy_static;

use crate::core::checks::ISNSFW_CHECK;
use regex::{Captures, Regex};
use serde::Deserialize;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

lazy_static! {
    // Regex to parse nhentai IDs from command input
    static ref ID_REGEX: Regex = Regex::new(r"([0-9]{1,6})(?:,|\s|$)+").unwrap();
}

#[command]
#[aliases("nh")]
#[description(
    "Looks up one or multiple nhentai IDs and returns information about the associated doujinshi."
)]
#[checks("IsNSFW")]
pub async fn nhentai(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let ids_raw = args.message();
    let id_captures: Vec<Captures> = ID_REGEX.captures_iter(ids_raw).collect();
    let client = reqwest::Client::new();

    for id_capture in id_captures {
        let id = id_capture.get(1).unwrap().as_str();

        let data: GalleryResponse = client
            .get(format!("https://nhentai.net/api/gallery/{}", id).as_str())
            .send()
            .await?
            .json()
            .await?;

        let _ = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                let msg = m.embed(|e| {
                    e.title(data.title.pretty)
                        .color(0xEC2854)
                        .url(format!("https://nhentai.net/g/{}", data.id))
                        .thumbnail(get_cover_url(&data.media_id, &data.images.cover.t))
                        .fields(vec![
                            ("Tags", build_tag_string(&data.tags), false),
                            (
                                "Stats",
                                format!(
                                    "**{pages}** pages, **{favorites}** favorites",
                                    pages = data.num_pages,
                                    favorites = data.num_favorites
                                ),
                                false,
                            ),
                        ])
                });

                println!("{:?}", msg);

                msg
            })
            .await;
    }

    Ok(())
}

fn get_cover_url(media_id: &str, cover_ext_raw: &str) -> String {
    let cover_ext = parse_extension(cover_ext_raw);

    format!(
        "https://t.nhentai.net/galleries/{}/cover.{}",
        media_id, cover_ext
    )
}

fn parse_extension(raw_ext: &str) -> String {
    match raw_ext {
        "j" => "jpg",
        "p" => "png",
        "g" => "gif",
        _ => "",
    }
    .to_string()
}

fn build_tag_string(all_tags: &Vec<GalleryTagInfo>) -> String {
    let tags = filter_tags_by_type(all_tags, TagType::Tag);
    let parodies = filter_tags_by_type(all_tags, TagType::Parody);
    let characters = filter_tags_by_type(all_tags, TagType::Character);
    let artists = filter_tags_by_type(all_tags, TagType::Artist);
    let groups = filter_tags_by_type(all_tags, TagType::Group);
    let languages = filter_tags_by_type(all_tags, TagType::Language);
    let categories = filter_tags_by_type(all_tags, TagType::Category);

    let mut tags_string = format_tag_items(&tags) + "\n";

    if !parodies.is_empty() {
        tags_string += format!("\n Parodies: {}", format_tag_items(&parodies)).as_str();
    }

    if !characters.is_empty() {
        tags_string += format!("\n Characters: {}", format_tag_items(&characters)).as_str();
    }

    if !artists.is_empty() {
        tags_string += format!("\n Artists: {}", format_tag_items(&artists)).as_str();
    }

    if !groups.is_empty() {
        tags_string += format!("\n Groups: {}", format_tag_items(&groups)).as_str();
    }

    if !languages.is_empty() {
        tags_string += format!("\n Languages: {}", format_tag_items(&languages)).as_str();
    }

    if !categories.is_empty() {
        tags_string += format!("\n Categories: {}", format_tag_items(&categories)).as_str();
    }

    tags_string
}

fn filter_tags_by_type(tags: &Vec<GalleryTagInfo>, tag_type: TagType) -> Vec<&GalleryTagInfo> {
    tags.iter().filter(|t| t.tag_type == tag_type).collect()
}

fn format_tag_items(tags: &Vec<&GalleryTagInfo>) -> String {
    tags.iter().fold("".to_string(), |mut tag_string, tag| {
        tag_string.push_str(format!("`{}` ", tag.name).as_str());
        tag_string
    })
}

#[derive(Deserialize, Debug)]
struct GalleryResponse {
    id: i32,
    media_id: String,
    title: GalleryTitle,
    images: GalleryImages,
    scanlator: String,
    tags: Vec<GalleryTagInfo>,
    num_pages: i32,
    num_favorites: i32,
}

#[derive(Deserialize, Debug)]
struct GalleryTitle {
    english: String,
    japanese: String,
    pretty: String,
}

#[derive(Deserialize, Debug)]
struct GalleryImages {
    pages: Vec<GalleryImageInfo>,
    cover: GalleryImageInfo,
    thumbnail: GalleryImageInfo,
}

#[derive(Deserialize, Debug)]
struct GalleryTagInfo {
    id: i64,

    #[serde(rename = "type")]
    tag_type: TagType,

    name: String,
    url: String,
    count: i32,
}

#[derive(Deserialize, Debug)]
struct GalleryImageInfo {
    t: String,
    w: i16,
    h: i16,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum TagType {
    Parody,
    Tag,
    Artist,
    Group,
    Language,
    Category,
    Character,
}
