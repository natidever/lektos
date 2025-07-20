
        use feed_rs::parser;

#[cfg(test)]
mod tests {
    use super::*;      


    #[test]

    fn test_valid_rss(){

let xml = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    
</body>
</html>
"#;
let feed = parser::parse(xml.as_bytes()).unwrap();

println!("feedx:{:?}", feed);



    }




}