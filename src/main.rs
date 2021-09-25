use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    //Bind TcpListener to address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    //data storage
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let db = db.clone();

        //get socket from listener and ignore the IP and port of the connection
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    //Connection enables reading/writing redis frames
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                //lock db to get access
                let mut db = db.lock().unwrap();
                //Store value as 'Vec<u8>'
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
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
