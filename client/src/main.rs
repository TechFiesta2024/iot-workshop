use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use paho_mqtt as mqtt;

async fn page() -> Html<&'static str> {
    // let msg = mqtt::Message::new("test", "Hello from rust", 0);
    // mqtt_client.publish(msg).await.unwrap();
    // println!("Message sent!");
    Html(
        r#"
        <html>
            <head>
                <title>IoT Workshop</title>
                <link href="https://unpkg.com/nes.css@latest/css/nes.min.css" rel="stylesheet" />
                <link href="https://fonts.googleapis.com/css?family=Press+Start+2P" rel="stylesheet">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <meta charset="UTF-8">
            </head>
            <body>
                <style>
                    * {
                        margin: 0;
                        padding: 0;
                        box-sizing: border-box;
                    }

                    .container {
                        height: 200px;
                        position: relative;
                    }

                    .center {  
                        margin: 0;
                        position: absolute;
                        top: 50%;
                        left: 50%;
                        transform: translate(-50%, -50%);
                    }
                </style>
                <div class="container">
                <div class="center">
                    <h1>Fun little MQTT Client.</h1>
                    <p>power by rust 🦀</p>
                    <button id="led1" type="button" class="nes-btn is-primary">Led 1</button>
                    <button id="led2" type="button" class="nes-btn is-success">Led 2</button>
                    <button id="led3" type="button" class="nes-btn is-warning">Led 3</button>
                    <button id="led4" type="button" class="nes-btn is-error">Led 4</button>
                </div>
                </div>

                <script>
                    function sendGetRequest(url) {
                        fetch(url)
                            .then(response => {
                                if (!response.ok) {
                                    throw new Error('Network response was not ok');
                                }
                                return response.json();
                            })
                            .then(data => {
                                console.log(data); // Response from server
                            })
                            .catch(error => {
                                console.error('There was a problem with the fetch operation:', error);
                            });
                    }

                    document.getElementById("led1").addEventListener("click", function() {
                        sendGetRequest("/led/1");
                    });

                    document.getElementById("led2").addEventListener("click", function() {
                        sendGetRequest("/led/2");
                    });

                    document.getElementById("led3").addEventListener("click", function() {
                        sendGetRequest("/led/3");
                    });

                    document.getElementById("led4").addEventListener("click", function() {
                        sendGetRequest("/led/4");
                    });
                </script>
            </body>
        </html>
    "#,
    )
}

async fn toggle_led(State(mqtt_client): State<mqtt::AsyncClient>, Path(led): Path<String>) {
    let topic = format!("led/{}", led);
    let msg = mqtt::Message::new(topic, "toogle", 0);
    mqtt_client.publish(msg).await.unwrap();
    println!("Message sent!");
}

#[tokio::main]
async fn main() {
    let client_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("ssl://df24f150d5654c89913485bda5986e90.s1.eu.hivemq.cloud:8883")
        .client_id("rustclient")
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(5))
        .ssl_options(mqtt::ssl_options::SslOptionsBuilder::new().finalize())
        .clean_session(true)
        .user_name("iotworkshop")
        .password("IotWorkshop1")
        .finalize();

    let client = mqtt::AsyncClient::new(client_opts).unwrap();
    client.connect(conn_opts).await.unwrap();

    let app = Router::new()
        .route("/", get(page))
        .route("/led/:led", get(toggle_led))
        .with_state(client.clone());

    // client.disconnect(None).await.unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
