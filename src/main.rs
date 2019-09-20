use blowfish::block_cipher_trait::generic_array::GenericArray;
use blowfish::block_cipher_trait::BlockCipher;

fn main() -> Result<(), failure::Error> {
    let base64ed_key =
        "qBkyTNYnDOz5GTXspQBJnwvXBqNhjqInvaR4H1Q577LslgVScSK/r5cePInzHbFO6LPTmxg/u3k=";
    let key = base64::decode(base64ed_key)?;

    let encrypter = blowfish::Blowfish::<byteorder::BigEndian>::new(GenericArray::from_slice(&key));

    let mut text = String::from("Helloooo").into_bytes();
    println!("clear_text: {}", String::from_utf8(text.clone()).unwrap());
    encrypter.encrypt_block(GenericArray::from_mut_slice(&mut text));
    println!("encrypted_text: {}", unsafe {
        String::from_utf8_unchecked(text.clone())
    });
    encrypter.decrypt_block(GenericArray::from_mut_slice(&mut text));
    println!("clear_text: {}", String::from_utf8(text).unwrap());
    Ok(())
}
