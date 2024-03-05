use atrium_api::app::bsky::feed::post::Record;
use atrium_api::com::atproto::sync::subscribe_repos::Message;
use atrium_xrpc_server::stream::frames::Frame;
use futures::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite};
use rs_car::Cid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut stream, _) =
        connect_async("wss://bsky.network/xrpc/com.atproto.sync.subscribeRepos").await?;

    while let Some(Ok(tungstenite::Message::Binary(message))) = stream.next().await {
        process_message(&message).await.unwrap();
    }
    Ok(())
}

async fn process_message(message: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match Frame::try_from(message)? {
        Frame::Message(message) => {
            match message.body {
                Message::Commit(commit) => {
                    for op in commit.ops {
                        let collection = op.path.split('/').next().expect("op.path is empty");
                        if op.action != "create" || collection != "app.bsky.feed.post" {
                            continue;
                        }
                        let (items, _) =
                            rs_car::car_read_all(&mut commit.blocks.as_slice(), true).await?;

                        let cid = op.cid.unwrap().link;
                        let op_cid_64: Option<Cid> = Some(Cid::try_from(cid.as_str())?);

                        if let Some((_, item)) = items.iter().find(|(cid, _)| Some(*cid) == op_cid_64) {
                            if let Ok(value) =
                                ciborium::de::from_reader::<Record, _>(&mut item.as_slice())
                            {
                                println!("{}: {}", value.created_at.as_ref(), value.text);
                            } else {
                                println!("FAILED: could not deserialize post from item of length: {}", item.len());

                            }
                        } else {
                            println!(
                                "FAILED: could not find item with operation cid {} out of {} items",
                                cid,
                                items.len()
                            );
                        }
                    }
                }
                _ => unimplemented!("{:?}", message.body),
            }
        }
        Frame::Error(err) => panic!("{err:?}"),
    }
    Ok(())
}
