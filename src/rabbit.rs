use std::collections::HashMap;
use amiquip::{Connection, ExchangeDeclareOptions, ExchangeType, Result};
use crate::channel::RabbitChannel;

pub struct Rabbit<'a> {
    connection: Connection,
    channels: HashMap<String, RabbitChannel<'a>>,
    current_channel: String,
}

impl<'a> Rabbit<'a> {
    pub fn init(host: &str, username: Option<&str>, password: Option<&str>, virtual_host: Option<&str>) -> Result<Self> {
        let user = username.unwrap_or("");
        let pass = password.unwrap_or("");
        let vhost = virtual_host.unwrap_or("");

        let url = if user.is_empty() && pass.is_empty() {
            format!("amqp://{}/{}", host, vhost)
        } else if pass.is_empty() {
            format!("amqp://{}@{}/{}", user, host, vhost)
        } else {
            format!("amqp://{}:{}@{}/{}", user, pass, host, vhost)
        };

        let mut connection = Connection::insecure_open(&url)?;
        let default_channel_result = RabbitChannel::new(&mut connection, 0);
        let channel = match default_channel_result {
          Ok(channel) => channel,
          Err(_err) => panic!("Fail initiating default channel")
        };
        let mut channels = HashMap::new();
        channels.insert(Rabbit::channel_key(0), channel);
        Ok(Rabbit {
            connection,
            channels,
            current_channel: Rabbit::channel_key(0),
        })
    }

    fn channel_key(channel_id: u16)-> String{
      format!("channel_{channel_id}")
    }

    pub fn create_channel(&mut self, channel_id: u16) {
      if self.channels.contains_key(&Rabbit::channel_key(channel_id)){
        println!("Channel id already exists");
      }
      let channel = RabbitChannel::new(&mut self.connection, channel_id).unwrap();

      self.channels.insert(Rabbit::channel_key(channel_id), channel);
    }

    pub fn set_default_channel(&mut self, channel_id: u16) {
        self.current_channel = Rabbit::channel_key(channel_id);
    }

    pub fn publish(&self, exchange_name: &str, routing_key: &str, body: &[u8], channel_id: Option<u16>) {
      let channel_key = Rabbit::channel_key(channel_id.unwrap_or_else(||0));
      self.channels.get(&channel_key).unwrap().publish(exchange_name, routing_key, body);
    }

    pub fn exchange_declare(&'a mut self, type_: ExchangeType, exchange_name: &str, options: Option<ExchangeDeclareOptions>, channel_id: Option<u16>) {
        let key = Rabbit::channel_key(channel_id.unwrap_or_else(|| 0));
        let channel = self.channels.get_mut(&key).unwrap();
        channel.exchange_declare(type_, exchange_name, options);
    }
}
