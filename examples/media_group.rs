use std::env;

use dotenv::dotenv;
use futures_util::future::BoxFuture;

use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::{InputFile, InputMediaPhoto, InputMediaVideo, MediaGroup, MediaGroupItem, SendMediaGroup, Update},
};

#[derive(Clone)]
struct Handler {
    client: Client,
    photo_path: String,
    photo_url: String,
    video_path: String,
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            log::info!("got an update: {:?}\n", update);
            if let Some(chat_id) = update.get_chat_id() {
                let photo_url = InputFile::url(this.photo_url);
                let photo_path = InputFile::path(this.photo_path).await.unwrap();
                let video_path = InputFile::path(this.video_path).await.unwrap();
                let media = MediaGroup::new(vec![
                    MediaGroupItem::for_photo(photo_url, InputMediaPhoto::default().with_caption("Photo 01")),
                    MediaGroupItem::for_photo(photo_path, InputMediaPhoto::default().with_caption("Photo 02")),
                    MediaGroupItem::for_video(video_path, InputMediaVideo::default().with_caption("Video 01")),
                ])
                .unwrap();

                let method = SendMediaGroup::new(chat_id, media);
                this.client.execute(method).await.unwrap();
            }
        })
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let photo_path = env::var("TGBOT_PHOTO_PATH").expect("TGBOT_PHOTO_PATH is not set");
    let photo_url = env::var("TGBOT_PHOTO_URL").expect("TGBOT_PHOTO_URL is not set");
    let video_path = env::var("TGBOT_VIDEO_PATH").expect("TGBOT_VIDEO_PATH is not set");
    let client = Client::new(token).expect("Failed to create API");
    LongPoll::new(
        client.clone(),
        Handler {
            client,
            photo_path,
            photo_url,
            video_path,
        },
    )
    .run()
    .await;
}
