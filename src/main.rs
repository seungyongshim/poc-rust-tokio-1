use coerce::actor::new_actor;
use tokio::sync::mpsc;

pub struct EchoActor{}

#[async_trait]
impl Actor for EchoActor{}

pub struct EchoMessage(String);

impl Message for EchoMessage {
    type Result = String;
}

#[async_trait]
impl Handler<EchoMessage> for EchoActor{
    async fn handle(
        &mut self,
        message: EchoMessage,
        _ctx: &mut ActorContext,
    ) -> String {
        message.0.clone()
    }
}

#[tokio::main]
async fn main(){

    let actor = new_actor(EchoActor {}).await.unwrap();

    let hello_world = "hello, world".to_string();
    let result = actor.send(EchoMessage(hello_world)).await;

    println!("{:?}", result);

}