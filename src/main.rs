#[macro_use]
extern crate rustless;
extern crate valico;
extern crate iron;
extern crate coap;

use valico::json_dsl;
use rustless::{
    Application, Api, Nesting, Versioning
};
use rustless::json::{ToJson, JsonValue};
use coap::{CoAPClient, CoAPRequest, IsMessage, MessageType, CoAPOption};

fn coap_power(power: bool) {
    let addr = "[fdde:ad00:beef:0:4127:83b3:14e0:97e2]:5683";
    let endpoint = "light";
    let client = CoAPClient::new(addr).unwrap();
    let mut request = CoAPRequest::new();
    request.set_version(1);
    request.set_type(MessageType::Confirmable);
    request.set_code("0.01");
    request.set_message_id(1);
    request.set_token(vec![0x51, 0x55, 0x77, 0xE8]);
    request.add_option(CoAPOption::UriPath, endpoint.to_string().into_bytes());
    request.set_payload(vec![if power {'1'} else {'0'} as u8]);
    client.send(&request).unwrap();
}

fn main() {
    // let api = Api::build(|api| {
    //     api.prefix("api");
    //     api.mount(Api::build(|light_api| {
    //         light_api.post("light", |endpoint| {
    //             endpoint.handle(|client, params| {
    //                 match params {
    //                     &JsonValue::Object(ref obj) => {
    //                         if let Some(&JsonValue::String(ref power)) = obj.get("power") {
    //                             coap_power(power == "on");
    //                         }
    //                     }
    //                     _ => {}
    //                 }
    //                 client.json(&params.to_json())
    //             })
    //         });
    //     }));
    // });
    coap_power(true);
    // let app = Application::new(api);
    // iron::Iron::new(app).http("0.0.0.0:8080").unwrap();
}
