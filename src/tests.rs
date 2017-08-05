use rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

#[test]
fn bad_leftpad_get() {
    let client = Client::new(rocket()).unwrap();

    // Try to get a message with an ID that doesn't exist.
    let mut res = client
        .get("/leftpad?str=99")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::NotFound);

    let body = res.body_string().unwrap();
    assert!(body.contains("404"));
    assert!(body.contains("The requested resource could not be found."));

    let mut res = client
        .get("/leftpad?str=99&len=")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::NotFound);

    let body = res.body_string().unwrap();
    assert!(body.contains("404"));
    assert!(body.contains("The requested resource could not be found."));

    let mut res = client
        .get("/leftpad?str=test&len=1000")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert!(body.contains("illegal padding length"));

    let mut res = client
        .get("/leftpad?str=&len=1000")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert!(body.contains("illegal content length"));
}

#[test]
fn good_leftpad_get() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .get("/leftpad?str=test&len=10")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert_eq!(
        body,
        "{\"original\":\"test\",\"padded\":\"      test\",\"length\":10}"
    );


    let mut res = client
        .get("/leftpad?str=test&len=10&ch=1")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert_eq!(
        body,
        "{\"original\":\"test\",\"padded\":\"111111test\",\"length\":10}"
    );

    let mut res = client
        .get("/leftpad?str=test&len=10&ch=1111")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert_eq!(
        body,
        "{\"original\":\"test\",\"padded\":\"111111test\",\"length\":10}"
    );
}


#[test]
fn bad_rightpad_get() {
    let client = Client::new(rocket()).unwrap();

    // Try to get a message with an ID that doesn't exist.
    let mut res = client
        .get("/rightpad?str=99")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::NotFound);

    let body = res.body_string().unwrap();
    assert!(body.contains("404"));
    assert!(body.contains("The requested resource could not be found."));

    let mut res = client
        .get("/rightpad?str=99&len=")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::NotFound);

    let body = res.body_string().unwrap();
    assert!(body.contains("404"));
    assert!(body.contains("The requested resource could not be found."));

    let mut res = client
        .get("/rightpad?str=test&len=1000")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert!(body.contains("illegal padding length"));

    let mut res = client
        .get("/rightpad?str=&len=1000")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert!(body.contains("illegal content length"));
}


#[test]
fn good_rightpad_get() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .get("/rightpad?str=test&len=10")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert_eq!(
        body,
        "{\"original\":\"test\",\"padded\":\"test      \",\"length\":10}"
    );

    let mut res = client
        .get("/rightpad?str=test&len=10&ch=1")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert_eq!(
        body,
        "{\"original\":\"test\",\"padded\":\"test111111\",\"length\":10}"
    );

    let mut res = client
        .get("/rightpad?str=test&len=10&ch=1111")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body_string().unwrap();
    assert_eq!(
        body,
        "{\"original\":\"test\",\"padded\":\"test111111\",\"length\":10}"
    );
}
