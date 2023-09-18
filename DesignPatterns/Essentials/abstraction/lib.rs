
// Abstraction:
// Reduce complexity by hiding unnecessary details

pub mod service{

    pub struct MailService;

    impl MailService {
        pub fn send_email(&self){
            self.connect();
            self.authenticate();
            self.disconnect();
        }

        fn connect(&self){
            println!("Connect");
        }

        fn authenticate(&self){
            println!("Authenticate");
        }

        fn disconnect(&self){
            println!("Disconnect");
        }
    }
}

