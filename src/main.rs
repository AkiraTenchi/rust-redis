use tokio::{net::{TcpListener, TcpStream}};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    //Bind TcpListener to address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop{
        //get socket from listener and ignore the IP and port of the connection
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    //data storage
    let mut db = HashMap::new();

    //Connection enables reading/writing redis frames
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                //Store value as 'Vec<u8>'
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        //send response to client
        connection.write_frame(&response).await.unwrap();
    }
}