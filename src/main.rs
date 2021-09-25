use tokio::{net::{TcpListener, TcpStream}};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    //Bind TcpListener to address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop{
        //get socket from listener and ignore the IP and port of the connection
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    //Connection enables reading/writing redis frames
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        //Respond with error since not implemented yet
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}