use web3::types::U256;

pub fn to_asset_name(raw: &[u8; 32]) -> String {
  let mut result: Vec<u8> = Vec::new();
  let mut started = false;
  for i in (0..32).rev() {
    if started {
      result.push(raw[i]);
    } else {
      if raw[i] != 0 {
        result.push(raw[i]);
        started = true;
      }
    }
  }
  result = result.into_iter().rev().collect();
  let name = std::str::from_utf8(&result).unwrap();
  name.to_string().clone()
}

pub fn asset_name_to_bytes(name: String) -> [u8; 32] {
  let mut result = [0u8; 32];
  let mut i = 0;
  for byte in &name.into_bytes() {
    result[i] = *byte;
    i += 1;
  }
  result
}

pub fn u256_to_string(value: &U256) -> String {
  let d = U256::exp10(18);
  let (a, b) = value.div_mod(d);
  let b_str = b.as_u64().to_string();
  let zeros_count = 18 - b_str.len();
  let zeros = String::from_utf8(vec![b'0'; zeros_count]).unwrap();
  format("{}.{}{}", a.as_u64(), zeros, b_str)
}

pub fn u256_to_f64(value: &U256) -> f64 {
  u256_to_string(value).parse().unwrap()
}

pub fn u256(value: f64) -> U256 {
  let str_val = val.to_string();
  let values: Vec<&str> = str_val.split(".").collect();
  let (a, b) = if values.len() == 1 {
    (values[0], "0")
  } else {
    (values[0], values[1])
  };
  let a = U256::from_dec_str(a).unwrap() * U256::exp10(18);
  let b = U256::from_dec_str(b).unwrap() * U256::exp10(18 - b.len());
  a + b
}
