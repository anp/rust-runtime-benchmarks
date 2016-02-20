use std::collections::BTreeMap;
use std::iter::repeat;
use cbor::{Decoder, DirectDecoder, Encoder};
use rustc_serialize::{Decodable, Encodable};
use rustc_serialize::json::{self, Json, ToJson};

pub fn run_all() -> BTreeMap<String, Vec<u64>> {
    let mut results = BTreeMap::new();

    for _ in 0..3 {
        results.entry("encode_small_cbor".to_string()).or_insert(vec![]).push(encode_small_cbor());
        results.entry("encode_small_json".to_string()).or_insert(vec![]).push(encode_small_json());
        results.entry("encode_small_tojson".to_string())
               .or_insert(vec![])
               .push(encode_small_tojson());
        results.entry("encode_medium_cbor".to_string())
               .or_insert(vec![])
               .push(encode_medium_cbor());
        results.entry("encode_medium_json".to_string())
               .or_insert(vec![])
               .push(encode_medium_json());
        results.entry("encode_medium_tojson".to_string())
               .or_insert(vec![])
               .push(encode_medium_tojson());
        results.entry("read_small_cbor".to_string()).or_insert(vec![]).push(read_small_cbor());
        results.entry("read_small_json".to_string()).or_insert(vec![]).push(read_small_json());
        results.entry("read_medium_cbor".to_string()).or_insert(vec![]).push(read_medium_cbor());
        results.entry("read_medium_json".to_string()).or_insert(vec![]).push(read_medium_json());
        results.entry("decode_small_cbor".to_string()).or_insert(vec![]).push(decode_small_cbor());
        results.entry("decode_small_direct_cbor".to_string())
               .or_insert(vec![])
               .push(decode_small_direct_cbor());
        results.entry("decode_small_json".to_string()).or_insert(vec![]).push(decode_small_json());
        results.entry("decode_medium_cbor".to_string())
               .or_insert(vec![])
               .push(decode_medium_cbor());
        results.entry("decode_medium_direct_cbor".to_string())
               .or_insert(vec![])
               .push(decode_medium_direct_cbor());
        results.entry("decode_medium_json".to_string())
               .or_insert(vec![])
               .push(decode_medium_json());
    }

    results
}

fn cbor_encode<T: Encodable>(v: T) -> Vec<u8> {
    let mut enc = Encoder::from_memory();
    enc.encode(&[v]).unwrap();
    enc.as_bytes().to_vec()
}

fn encode_small_cbor() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);

    bench!(10_000_000, {
        cbor_encode(&data);
    })
}

fn encode_small_json() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);

    bench!(10_000_000, {
        json::encode(&data).unwrap();
    })
}

fn encode_small_tojson() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);

    bench!(1_000_000, {
        data.to_json().to_string();
    })
}

fn encode_medium_cbor() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let items = repeat(&data).take(10_000).collect::<Vec<_>>();

    bench!(10_000, {
        cbor_encode(&items);
    })
}


fn encode_medium_json() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let items = repeat(data).take(10_000).collect::<Vec<_>>();

    bench!(1_000, {
        json::encode(&items).unwrap();
    })
}


fn encode_medium_tojson() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let items = repeat(data).take(10_000).collect::<Vec<_>>();

    bench!(100, {
        items.to_json().to_string();
    })
}


fn read_small_cbor() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let mut enc = Encoder::from_memory();
    enc.encode(&[data]).unwrap();
    let bytes = enc.as_bytes();

    bench!(10_000_000, {
        let mut dec = Decoder::from_bytes(bytes.to_vec());
        dec.items().next().unwrap().unwrap();
    })
}


fn read_small_json() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let string = json::encode(&data).unwrap();

    bench!(1_000_000, {
        Json::from_str(&string).unwrap();
    })
}


fn read_medium_cbor() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let items = repeat(data).take(10_000).collect::<Vec<_>>();
    let mut enc = Encoder::from_memory();
    enc.encode(&[items]).unwrap();
    let bytes = enc.as_bytes();

    bench!(1_000, {
        let mut dec = Decoder::from_bytes(bytes.to_vec());
        dec.items().next().unwrap().unwrap();
    })
}


fn read_medium_json() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let items = repeat(data).take(10_000).collect::<Vec<_>>();
    let string = json::encode(&items).unwrap();

    bench!(100, {
        Json::from_str(&string).unwrap();
    })
}


fn decode_small_cbor() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let mut enc = Encoder::from_memory();
    enc.encode(&[data]).unwrap();
    let bytes = enc.as_bytes();

    bench!(1_000_000, {
        let mut dec = Decoder::from_bytes(bytes.to_vec());
        let _: (String, bool, (), Vec<u64>, f64) = dec.decode().next().unwrap().unwrap();
    })
}


fn decode_small_direct_cbor() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let mut enc = Encoder::from_memory();
    enc.encode(&[data]).unwrap();
    let bytes = enc.as_bytes();

    bench!(10_000_000, {
        let mut dec = DirectDecoder::from_bytes(bytes.to_vec());
        let _: (String, bool, (), Vec<u64>, f64) = Decodable::decode(&mut dec).unwrap();
    })
}


fn decode_small_json() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let string = json::encode(&data).unwrap();

    bench!(1_000_000, {
        let _: (String, bool, (), Vec<u64>, f64) = json::decode(&string).unwrap();
    })
}


fn decode_medium_cbor() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let items = repeat(data).take(10_000).collect::<Vec<_>>();
    let mut enc = Encoder::from_memory();
    enc.encode(&[items]).unwrap();
    let bytes = enc.as_bytes();

    bench!(100, {
        let mut dec = Decoder::from_bytes(bytes.to_vec());
        let _: Vec<(String, bool, (), Vec<u64>, f64)> = dec.decode().next().unwrap().unwrap();
    })
}


fn decode_medium_direct_cbor() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let items = repeat(data).take(10_000).collect::<Vec<_>>();
    let mut enc = Encoder::from_memory();
    enc.encode(&[items]).unwrap();
    let bytes = enc.as_bytes();

    bench!(1_000, {
        let mut dec = DirectDecoder::from_bytes(bytes.to_vec());
        let _: Vec<(String, bool, (), Vec<u64>, f64)> = Decodable::decode(&mut dec).unwrap();
    })
}


fn decode_medium_json() -> u64 {
    let data = ("hello, world".to_string(),
                true,
                (),
                vec![1, 1000, 100_000, 10_000_000],
                3.14);
    let items = repeat(data).take(10_000).collect::<Vec<_>>();
    let string = json::encode(&items).unwrap();

    bench!(100, {
        let _: Vec<(String, bool, (), Vec<u64>, f64)> = json::decode(&string).unwrap();
    })
}
