//使用 PBKDF2 对密码进行加密（salt）和散列（hash）运算 
// 对于通过 PBKDF2 密钥派生函数 pbkdf2::derive 生成的加密（加盐算法）密码，
//使用 ring::pbkdf2 进行散列（哈希）运算，使用 pbkdf2::verify 
//验证散列（哈希）运算是否正确。salt 值是使用 SecureRandom::fill 生成的，
//salt 字节数组被其安全生成的随机数填充。
use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

fn main() -> Result<(), Unspecified> {
    // 定义凭证长度为SHA-512哈希输出的长度
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

    // 设置PBKDF2迭代次数为100,000次
    let n_iter = NonZeroU32::new(100_000).unwrap();

    // 创建一个安全随机数生成器
    let rng = rand::SystemRandom::new();

    // 定义一个长度为CREDENTIAL_LEN的数组用于存储盐值
    let mut salt = [0u8; CREDENTIAL_LEN];

    // 使用随机数生成器填充盐值
    rng.fill(&mut salt)?;

    // 定义密码
    let password = "Guess Me If You Can!";

    // 定义一个长度为CREDENTIAL_LEN的数组用于存储PBKDF2的哈希值
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    // 使用PBKDF2算法生成哈希值
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512, // 使用PBKDF2-HMAC-SHA512算法
        n_iter,                     // 迭代次数
        &salt,                      // 盐值
        password.as_bytes(),        // 密码的字节表示
        &mut pbkdf2_hash,           // 输出哈希值存储位置
    );

    // 输出盐值和PBKDF2哈希值，使用HEXUPPER编码
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    // 验证生成的哈希值是否正确
    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );

    // 使用错误的密码进行验证
    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    // 断言正确的密码验证通过
    assert!(should_succeed.is_ok());

    // 断言错误的密码验证失败
    assert!(!should_fail.is_ok());

    // 返回成功
    Ok(())
}
