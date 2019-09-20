use blowfish::block_cipher_trait::generic_array::typenum::{U1, U8};
use blowfish::block_cipher_trait::generic_array::{ArrayLength, GenericArray};
use blowfish::block_cipher_trait::BlockCipher;

fn main() -> Result<(), failure::Error> {
    let base64ed_key =
        "qBkyTNYnDOz5GTXspQBJnwvXBqNhjqInvaR4H1Q577LslgVScSK/r5cePInzHbFO6LPTmxg/u3k=";
    let key = base64::decode(base64ed_key)?;

    let encrypter = blowfish::Blowfish::<byteorder::BigEndian>::new(GenericArray::from_slice(&key));

    let text = String::from("testtest@example.com");
    println!("clear_text: {}", &text);
    let mut prepared_blocks = into_blocks(text.into_bytes());
    (&mut prepared_blocks)
        .into_iter()
        .for_each(|b: &mut _| encrypter.encrypt_block(b));
    let encrypted_vec = into_vec(prepared_blocks);
    println!("encrypted_base64_text: {}", base64::encode(&encrypted_vec));
    let mut encrypted_blocks = into_blocks(encrypted_vec);
    (&mut encrypted_blocks)
        .into_iter()
        .for_each(|b: &mut _| encrypter.decrypt_block(b));
    let decrypted_vec = into_vec(encrypted_blocks);
    println!("decrypted_vec: {:#?}", &decrypted_vec);
    println!(
        "decrypted_text: {:#?}",
        String::from_utf8(decrypted_vec).unwrap().trim().as_bytes()
    );

    Ok(())
}

fn into_blocks(mut vec: Vec<u8>) -> Vec<GenericArray<u8, U8>> {
    vec.extend(std::iter::repeat(' ' as u8).take((8 - vec.len() % 8) % 8));
    vec.chunks(8)
        .map(|ref mut block| GenericArray::clone_from_slice(block))
        .collect::<Vec<_>>()
}

fn into_vec(blocks: Vec<GenericArray<u8, U8>>) -> Vec<u8> {
    blocks
        .iter()
        .map(|block| block.iter().collect::<Vec<&u8>>())
        .flatten()
        .map(Clone::clone)
        .collect()
}
