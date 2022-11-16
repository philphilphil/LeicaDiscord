use anyhow::Result;
use rand::*;
use serenity::{
    model::prelude::{EmojiId, EmojiIdentifier, Message},
    prelude::Context,
};
use std::env;

/// React to every fourth message with a appropriate emoji if a lesser camera brand is mentioned
pub async fn handle_lesser_brand_messages(context: Context, msg: Message) -> Result<()> {
    let msg_text = msg.content.to_lowercase();
    let contains_sony = msg_text.contains("sony");
    let contains_fuji = msg_text.contains("fuji");

    if (contains_sony || contains_fuji) && should_react_to_message() {
        if contains_sony {
            let puke_emoji_id: u64 = env::var("PUKE_EMOJI_ID")?.parse()?;
            msg.react(
                &context,
                EmojiIdentifier {
                    animated: false,
                    id: EmojiId(puke_emoji_id),
                    name: "puke".to_string(),
                },
            )
            .await?;
        }

        if contains_fuji {
            let fuji_trash_emoji_id: u64 = env::var("FUJI_TRASH_EMOJI_ID")?.parse()?;
            msg.react(
                &context,
                EmojiIdentifier {
                    animated: false,
                    id: EmojiId(fuji_trash_emoji_id),
                    name: "fujitrash".to_string(),
                },
            )
            .await?;
        }
    }

    Ok(())
}

fn should_react_to_message() -> bool {
    let mut rng = rand::thread_rng();
    let chance = rng.gen_range(1..=4);
    chance == 1
}
