use std::io::Cursor;

use crate::{
    api::Form,
    types::{InputFile, InputSticker, InputStickers, MaskPosition, MaskPositionPoint},
};

#[test]
fn input_sticker() {
    let value = InputSticker::new(InputFile::file_id("file-id"), ["😻"]);
    let form: Form = value.try_into().unwrap();
    assert_eq!(
        Form::from([("sticker", r#"{"sticker":"file-id","emoji_list":["😻"]}"#.into())]),
        form
    );

    let value = InputSticker::new(InputFile::url("https://google.com/favicon.ico"), ["😻"])
        .with_keywords(["kw"])
        .with_mask_position(MaskPosition {
            point: MaskPositionPoint::Forehead,
            x_shift: 1.0,
            y_shift: 2.0,
            scale: 3.0,
        });
    let form: Form = value.try_into().unwrap();
    assert_eq!(
        Form::from([(
            "sticker",
            concat!(
                r#"{"sticker":"https://google.com/favicon.ico","emoji_list":["😻"],"#,
                r#""mask_position":{"point":"forehead","x_shift":1.0,"y_shift":2.0,"scale":3.0},"#,
                r#""keywords":["kw"]}"#
            )
            .into()
        )]),
        form
    );

    let value = InputSticker::new(Cursor::new("test"), ["😻"]);
    let form: Form = value.try_into().unwrap();
    assert_eq!(
        Form::from([
            ("tgbot_input_sticker", InputFile::reader(Cursor::new("test")).into()),
            (
                "sticker",
                r#"{"sticker":"attach://tgbot_input_sticker","emoji_list":["😻"]}"#.into()
            )
        ]),
        form
    );
}

#[test]
fn input_stickers() {
    let value = InputStickers::default()
        .with(InputSticker::new(InputFile::file_id("file-id"), ["😻"]))
        .with(InputSticker::new(
            InputFile::url("https://google.com/favicon.ico"),
            ["😻"],
        ));
    let form: Form = value.try_into().unwrap();
    assert_eq!(
        Form::from([(
            "stickers",
            concat!(
                r#"[{"sticker":"file-id","emoji_list":["😻"]},"#,
                r#"{"sticker":"https://google.com/favicon.ico","emoji_list":["😻"]}]"#
            )
            .into()
        )]),
        form
    );
}
