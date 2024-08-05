use ring::hmac::{Key, Tag};
use ring::{hmac, rand};
use ring::rand::SecureRandom;
use ring::error::Unspecified;

struct SendMsg<'a>{
    message:&'a str,
    signature:Tag
}
impl<'a> SendMsg<'a>{
    fn new(message:&'a str,signature:Tag)->SendMsg{
        SendMsg { message: message, signature: signature }
    }
}
fn main() {
    let (s,k)=signate("wodwadjaiowd").unwrap();
    println!("{:?}",descrypt(s, k));

}

fn signate(message:&str)->Result<(SendMsg,Key), Unspecified>{
    //定义了一个长度为 48 的字节数组 key_value，用于存储 HMAC 密钥。
    let mut key_value = [0; 48];
    //创建了一个新的 SystemRandom 对象，它是一个安全的随机数生成器。
    let rng = rand::SystemRandom::new();
    //使用 SystemRandom 对象生成随机数，并填充到 key_value 数组中。如果随机数生成失败，那么 ? 操作符将导致函数立即返回一个错误。
    rng.fill(&mut key_value)?;
    //使用 key_value 数组创建了一个新的 HMAC 密钥。
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);
    //定义了一个字符串 message，它是将要被签名的消息
    // let message = "Legitimate and important message.";
    //使用 HMAC 密钥对消息进行签名，并返回签名。
    let signature = hmac::sign(&key, message.as_bytes());
    //验证消息的 HMAC 签名。如果签名验证失败，那么 ? 操作符将导致函数立即返回一个错误。
    // hmac::verify(&key, message.as_bytes(), signature.as_ref())?;
    Ok((SendMsg::new(message, signature),key))
}
fn descrypt(s:SendMsg,key:Key)->bool{
    let message = s.message;
    let signature = s.signature;
    match hmac::verify(&key, message.as_bytes(), signature.as_ref()){
        Ok(_) => true,
        Err(_)=> false
    }
}