
#[path = "./lib.rs"]
mod service;
use service::service::MailService;

fn main() {
    let my_mailservice = MailService;
    
    // Hides the complexity of connecting, authenticating, disconnecting
    my_mailservice.send_email();
    
    // ERROR
    // my_mailservice.connect();
    // my_mailservice.disconnect();
}