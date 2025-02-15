// use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// pub struct Collection {
//     pub name: String,
// }

#[cfg(test)]

mod tests {
    use std::fs;

    use kdl::KdlDocument;

    #[test]
    fn de() {
        let content =
            fs::read_to_string("./tests/requests/TestRequest/TestRequest.http.sapic").unwrap();
        let r: KdlDocument = content.parse().unwrap();
        dbg!(r);
        // let config: Config = parse::de::from_str(&content)?;

        // println!("{:#?}", r.);
    }
}
