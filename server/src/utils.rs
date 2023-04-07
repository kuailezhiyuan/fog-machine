pub fn random_token(validate_token: impl Fn(&str) -> bool) -> String {
    use rand::distributions::{Alphanumeric, DistString};
    loop {
        let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 24);
        if validate_token(&token) {
            return token;
        }
    }
}

pub fn json_decode(data: &str) -> anyhow::Result<serde_json::Value> {
    let obj: serde_json::Value = match serde_json::from_str(data) {
        Ok(decoded) => decoded,
        Err(ref e) => return Err(anyhow!("Json decode error: {}", e)),
    };
    let dic = obj.as_object().unwrap();
    let code = if dic.contains_key("errcode") {
        "errcode"
    } else {
        "code"
    };

    let code = match obj[code].as_i64() {
        Some(v) => v,
        None => 0,
    };
    if code != 0 {
        let msg: String = if dic.contains_key("msg") {
            obj["msg"].to_string()
        } else {
            obj["errmsg"].to_string()
        };
        return Err(anyhow!(format!("code: {},msg: {}", code, msg)));
    }
    Ok(obj)
}
