use std::io::prelude::*;
use axum::{Json, response::{Html, IntoResponse, Response},routing::get,routing::post,routing::post_service,Router,http::{Uri, header::{self, HeaderMap, HeaderName}},extract::{Extension, FromRequest, RequestParts}};
use std::fs::File;
use std::fs;
use std::str::FromStr;
use std::env;
use axum::body::Full;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;
use std::{net::SocketAddr, time::Duration};
use serde::{Serialize, Deserialize};
use serde::ser::{SerializeStruct, Serializer};
use sqlx::postgres::types::PgTimeTz;
use postgres::types::Type;
use axum::extract::Form;

#[derive(Serialize, Deserialize)]
struct User
{
    username: String,
    full_name: String,
    bio: String
}

async fn response() -> axum::http::response::Builder {
    Response::builder()
}

async fn root() -> Html<String>{
    let mut r=File::open("index.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    let mut r=File::open("visits").unwrap();
    let mut e = String::new();
    r.read_to_string(&mut e);
    let mut f = i32::from_str(&e).unwrap();
    f+=1;
    fs::write("visits", &f.to_string().as_bytes()).unwrap();
    p+=&("Visits: ".to_owned()+&f.to_string());
    Html(p)
}

async fn canvas() -> Html<String>{
    let mut r=File::open("canvas.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    Html(p)
}

async fn discussions() -> Html<String>{
    let mut r=File::open("discussions").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    Html(p)
}

async fn rest() -> Html<String>{
    let mut r=File::open("rest.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    Html(p)
}

async fn reset() -> Html<String>{
    fs::write("visits", "0".as_bytes()).unwrap();
    let mut r=File::open("reset").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    Html(p)
}

async fn indexjs() -> impl IntoResponse{
    let mut r=File::open("index.js").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/javascript; charset=UTF-8")
        .body(Full::from(p))
        .unwrap()
}

async fn pkgjs() -> impl IntoResponse{
    let mut r=File::open("./pkg/webgl.js").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/javascript; charset=UTF-8")
        .body(Full::from(p))
        .unwrap()
}

async fn pkgbg() -> impl IntoResponse{
    let mut r=File::open("./pkg/webgl_bg.wasm").unwrap();
    let mut p = vec![];
    r.read_to_end(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","application/wasm")
        .body(Full::from(p))
        .unwrap()
}



#[tokio::main]
async fn main() {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://gugzerxbugsalp:eb8178bf93a439085553be5b5e8c346e4b7b4b9c321591ea0e49206c2573bbf2@ec2-99-81-137-11.eu-west-1.compute.amazonaws.com:5432/dek8a1pejic2ln".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .unwrap();
    let app = Router::new()
        .route(
        "/", get(root))
        .route(
        "/canvas.html", get(canvas))
        .route(
        "/discussions", get(discussions))
        .route(
        "/rest.html", get(rest))
        .route(
        "/reset", get(reset))
        .route(
        "/index.js", get(indexjs))
        .route(
        "/pkg/webgl.js", get(pkgjs))
        .route(
        "/pkg/webgl_bg.wasm", get(pkgbg))
        .route(
        "/sql",
        get(using_connection_pool_extractor))
        .route(
        "/sqlq",
        get(using_connection_pool_extractorq))
        .route(
        "/sqlz",
        get(using_connection_pool_extractorz))
        .route(
        "/getusers", get(getusers))
        .route(
        "/getuserspasswords", get(using_connection_pool_extractorz))
        .route(
        "/makeuserssql", get(using_connection_pool_extractorz))
        .route(
        "/makedummyuser", get(makedummyuser))
        .route(
        "/makeuser", get(show_form).post(accept_form))
        .route(
        "/deleteuser", get(deleteuserq).post(deleteuser))
        .route(
        "/deleteallusers", get(deleteallusers))
        .route(
        "/followuser", get(followuserq).post(followuser))
        .layer(Extension(pool));
    let q = env::var("PORT")
        .unwrap_or_else(|_| "7878".to_string())
        .to_string();
    axum::Server::bind(&("0.0.0.0:".to_owned()+&q).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn using_connection_pool_extractor(
    Extension(pool): Extension<PgPool>,
) -> String {
    sqlx::Executor::execute(&pool, "INSERT INTO i (e) VALUES ('ep')").await.unwrap();
        "".to_string()
}

async fn using_connection_pool_extractorz(
    Extension(pool): Extension<PgPool>,
) -> String {
    sqlx::Executor::execute(&pool, "CREATE TABLE users (ID    SERIAL PRIMARY KEY, username    TEXT NOT NULL, full_name    TEXT NOT NULL, created_at    timestamp , bio    TEXT NOT NULL, followers  integer[])")
        .await
        .unwrap()
        .rows_affected()
        .to_string();
        "".to_string()
}

async fn using_connection_pool_extractorq(
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let s : String = sqlx::query("SELECT e FROM i")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
    Html(s)
}

async fn makeuser(
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse{
    let username="ab";
    let full_name="ab cd";
    let bio="abcdabcd qw";
    sqlx::Executor::execute(&pool, &format!("INSERT INTO users (username, full_name, created_at, bio) VALUES ({}, {}, now, {})", username, full_name, bio)[..]).await.unwrap();
    response()
        .await.status(204)
        .body(Full::from("".to_string()))
        .unwrap()
}

async fn makedummyuser(
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse{
    let username2="'ab'";
    let full_name2="'abcd'";
    let bio2="'abcdabcdqw'";
    sqlx::Executor::execute(&pool, &format!(r#"INSERT INTO users (username, full_name, created_at, bio, followers) VALUES ({}, {}, 'now', {}, {})"#, username2, full_name2, bio2, "'{987878}'")[..]).await.unwrap();
    response()
        .await.status(200)
        .body(Full::from("Done".to_string()))
        .unwrap()
}

async fn getusers(Extension(pool): Extension<PgPool>,
) -> impl IntoResponse{
    let s = sqlx::query("SELECT ID, username, full_name, created_at, bio FROM users ORDER BY ID")
        .fetch_all(&pool)
        .await
        .unwrap();
    let mut p=0;
    let mut q : i32=0;
    let mut r=String::new();
    r+="<html><head></head><body>";
    while p<s.len()
    {
        r+="<p>ID: ";
        q=s[p].get(0);
        r+=&q.to_string();
        r+=" Username: ";
        r+=s[p].get(1);
        r+=" Full name: ";
        r+=s[p].get(2);
        r+=" Created_at: ";
        let w : sqlx::types::chrono::NaiveDateTime=s[p].get(3);
        r+=&w.to_string();
        r+=" Bio: ";
        r+=s[p].get(4);
        r+=" Followers: ";
        let mut s : Vec<i32>= sqlx::query(&format!("SELECT followers FROM users WHERE ID = {}", q))
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
        let mut w=0;
        w+=1;
        let mut qw=0;
        if(s.len()>0)
        {
            while w<s.len()
            {
                qw+=1;
                w+=1;
            }
        }
        r+=&qw.to_string();
        r+=" (Followed by:";
        w=0;
        w+=1;
        if(s.len()>0)
        {
            while w<s.len()
            {
                r+=" User ";
                r+=&s[w].to_string();
                if w+1<s.len()
                {
                    r+=",";
                }
                w+=1;
            }
        }
        r+=")\r\n</p>";
        p+=1;
    }
    r+=r#"<p>
                <form action="/followuser" method="post">
                    <p><label>
                        Follow/Unfollow user 
                        <input type="text" name="followed">
                    </label></p>
                    <p><label>
                        as user
                        <input type="text" name="follower">
                    </label></p>
                    <p><input type="submit" value="Follow/Unfollow"></p>
                </form></p>
            </body>
        </html>
        "#;
    response()
        .await.status(200)
        .body(Full::from(r))
        .unwrap()
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/makeuser" method="post">
                    <p><label for="name">
                        Username:
                        <input type="text" name="username">
                    </label></p>
                    <p><label>
                        Full name:
                        <input type="text" name="full_name">
                    </label></p>
                    <p><label>
                        Bio:
                        <input type="text" name="bio">
                    </label></p>
                    <p><input type="submit" value="Make user"></p>
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
struct UserN{
    name: String,
    email: String,
}

async fn accept_form(q: Form<User>, Extension(pool): Extension<PgPool>,
) -> impl IntoResponse{
    let username=q.0.username;
    let full_name=q.0.full_name;
    let bio=q.0.bio;
    sqlx::Executor::execute(&pool, &format!(r#"INSERT INTO users (username, full_name, created_at, bio, followers) VALUES ('{}', '{}', 'now', '{}', {})"#, username, full_name, bio, "'{987878}'")[..]).await.unwrap();
    response()
        .await.status(200)
        .body(Full::from(r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/makeuser" method="post">
                    <p><label for="name">
                        Username:
                        <input type="text" name="username">
                    </label></p>
                    <p><label>
                        Full name:
                        <input type="text" name="full_name">
                    </label></p>
                    <p><label>
                        Bio:
                        <input type="text" name="bio">
                    </label></p>
                    <p><input type="submit" value="Make user"></p>
                </form>
            </body>
        </html>
        "#.to_string()))
        .unwrap()
}

async fn deleteuserq() -> impl IntoResponse{
    response()
        .await.status(200)
        .body(Full::from(r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/deleteuser" method="post">
                    <p><label>
                        User ID:
                        <input type="text" name="userid">
                    </label></p>
                    <p><input type="submit" value="Delete user"></p>
                </form>
            </body>
        </html>
        "#.to_string()))
        .unwrap()
}

#[derive(Deserialize)]
struct UserID{
    userid: String
}

async fn deleteuser(q: Form<UserID>, Extension(pool): Extension<PgPool>,
) -> impl IntoResponse{
    let id : i32=q.0.userid.parse().unwrap();
    sqlx::Executor::execute(&pool, &format!("DELETE  FROM users WHERE ID = {}", id)[..]).await.unwrap();
    response()
        .await.status(200)
        .body(Full::from(r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/deleteuser" method="post">
                    <p><label>
                        User ID:
                        <input type="text" name="userid">
                    </label></p>
                    <p><input type="submit" value="Delete user"></p>
                </form>
            </body>
        </html>
        "#.to_string()))
        .unwrap()
}
async fn followuserq() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/followuser" method="post">
                    <p><label>
                        Follow/Unfollow user 
                        <input type="text" name="followed">
                    </label></p>
                    <p><label>
                        as user
                        <input type="text" name="follower">
                    </label></p>
                    <p><input type="submit" value="Follow/Unfollow"></p>
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize)]
struct FollowUserIDs{
    follower: String,
    followed: String
}

async fn followuser(q: Form<FollowUserIDs>, Extension(pool): Extension<PgPool>,
) -> impl IntoResponse{
    let followerid : i32=q.0.follower.parse().unwrap();
    let followedid : i32=q.0.followed.parse().unwrap();
    let mut s : Vec<i32>= sqlx::query(&format!("SELECT followers FROM users WHERE ID = {}", followedid))
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
    let mut r=0;
    let mut q=false;
    let mut w=r.clone();
    while r<s.len()
    {
        if s[r]==followerid
        {
            q=true;
            w=r;
        }
        r+=1;
    }
    if q==true
    {
        s.drain(w..w+1);
    }
    else if followerid!=followedid
    {
        s.push(followerid);
    }
    let mut sm=String::new();
    sm+="UPDATE users SET followers = '{";
    r=0;
    while r<s.len()
    {
        sm+=&s[r].to_string();
        if r+1<s.len()
        {
            sm+=",";
        }
        r+=1;
    }
    sm+="}'";
    sm+=&format!(" WHERE ID = {}", followedid);
    sqlx::Executor::execute(&pool, &sm[..]).await.unwrap();
    let s = sqlx::query("SELECT ID, username, full_name, created_at, bio FROM users ORDER BY ID")
        .fetch_all(&pool)
        .await
        .unwrap();
    let mut p=0;
    let mut q : i32=0;
    let mut r=String::new();
    r+="<html><head></head><body>";
    while p<s.len()
    {
        r+="<p>ID: ";
        q=s[p].get(0);
        r+=&q.to_string();
        r+=" Username: ";
        r+=s[p].get(1);
        r+=" Full name: ";
        r+=s[p].get(2);
        r+=" Created_at: ";
        let w : sqlx::types::chrono::NaiveDateTime=s[p].get(3);
        r+=&w.to_string();
        r+=" Bio: ";
        r+=s[p].get(4);
        r+=" Followers: ";
        let mut s : Vec<i32>= sqlx::query(&format!("SELECT followers FROM users WHERE ID = {}", q))
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
        let mut w=0;
        w+=1;
        let mut qw=0;
        if(s.len()>0)
        {
            while w<s.len()
            {
                qw+=1;
                w+=1;
            }
        }
        r+=&qw.to_string();
        r+=" (Followed by:";
        w=0;
        w+=1;
        if(s.len()>0)
        {
            while w<s.len()
            {
                r+=" User ";
                r+=&s[w].to_string();
                if w+1<s.len()
                {
                    r+=",";
                }
                w+=1;
            }
        }
        r+=")\r\n</p>";
        p+=1;
    }
    r+=r#"<p>
                <form action="/followuser" method="post">
                    <p><label>
                        Follow/Unfollow user 
                        <input type="text" name="followed">
                    </label></p>
                    <p><label>
                        as user
                        <input type="text" name="follower">
                    </label></p>
                    <p><input type="submit" value="Follow/Unfollow"></p>
                </form></p>
            </body>
        </html>
        "#;
    response()
        .await.status(200)
        .body(Full::from(r))
        .unwrap()
}
async fn deleteallusers(Extension(pool): Extension<PgPool>,
) -> impl IntoResponse{
    sqlx::Executor::execute(&pool, &format!("DELETE  FROM users")[..]).await.unwrap();
    response()
        .await.status(200)
        .body(Full::from(r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                Done.
                <p><a href=/>Index</a></p>
            </body>
        </html>
        "#.to_string()))
        .unwrap()
}
