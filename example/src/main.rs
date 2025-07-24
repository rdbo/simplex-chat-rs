use anyhow::Result;
use simplex_chat::{ChatClient, ChatInfo, ChatResponse, ChatType};

async fn process_messages(mut chat: ChatClient) -> Result<()> {
    loop {
        let message = chat.next_message().await?;
        println!("Received message: {:#?}", message);

        match message.resp {
            ChatResponse::NewChatItems { chat_items, .. } => {
                println!("CHATITEMS");
                for chat_item in chat_items {
                    println!("New message: {}", chat_item.chat_item.meta.item_text);
                    println!("Message date: {}", chat_item.chat_item.meta.item_ts);
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let mut chat = ChatClient::start("ws://localhost:5225").await?;

    let user = chat.api_get_active_user().await?;
    println!("Active User: {:?}", user);

    let address = match chat.api_get_user_address().await? {
        Some(addr) => addr,
        None => chat.api_create_user_address(user.user_id).await?,
    };
    println!("Address: {:?}", address);

    let chats = chat
        .api_chats()
        .await?
        .into_iter()
        .filter_map(|c| match c.chat_info {
            ChatInfo::Direct { contact, .. } => Some(format!("@{}", contact.local_display_name)),
            ChatInfo::Group { group_info, .. } => {
                Some(format!("#{}", group_info.group_profile.display_name))
            }
            _ => None,
        })
        .collect::<Vec<String>>();
    println!("Chats: {:?}", chats);

    let members = chat.api_list_members(1).await?;
    println!("Members of group ID '1': {:?}", members);

    chat.api_send_text_message(ChatType::Direct, 1, "Hello! This message was automated")
        .await?;

    process_messages(chat).await?;

    Ok(())
}
