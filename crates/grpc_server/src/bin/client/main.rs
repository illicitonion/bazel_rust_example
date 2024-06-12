use proto_bindings::proto::greeter_client::GreeterClient;
use proto_bindings::proto::HelloRequest;

// https://github.com/hyperium/tonic/blob/master/examples/src/helloworld/client.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
