use std::collections::HashMap;
use amiquip::{Channel, Connection, Exchange, ExchangeDeclareOptions, ExchangeType, Publish};

pub struct RabbitChannel<'a>{
  inner: Channel,
  exchanges: HashMap<String, Exchange<'a>>
}

impl<'a> RabbitChannel<'a>{
  pub fn new(connection: &mut Connection, channel_id: u16)-> Result<RabbitChannel<'a>, ()> {
    let channel_result = connection.open_channel(Some(channel_id));
    let channel = match channel_result {
      Ok(channel) => Ok(RabbitChannel{inner: channel, exchanges: HashMap::new()}),
      Err(_err)=> Err(())
    };
    channel
  }

  pub fn exchange_declare(&'a mut self, type_: ExchangeType, exchange_name: &str, exchange_options: Option<ExchangeDeclareOptions>){
    let options = exchange_options.unwrap_or_else(ExchangeDeclareOptions::default);
    let exchange = self.inner.exchange_declare(type_, exchange_name, options).unwrap();
    self.exchanges.insert(exchange_name.to_string(), exchange);
  }

  pub fn publish(&self, exchange_name: &str, routing_key: &str, body:&[u8]){
    let exchange = self.exchanges.get(exchange_name);
    match exchange {
      Some(exchange)=>{
        match exchange.publish(Publish::new(body, routing_key)){
          Ok(_)=>{
            println!("Message sent");
          },
          Err(err)=>{
            println!("{:?}", err);
          }
        }

      },
      None => {
        println!("Exchange not fount for this channel. Please define an exchange named {exchange_name}");
      }
    }
  }
}