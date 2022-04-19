use std::io::prelude::*;
use axum::{extract::{ws::{Message, WebSocket, WebSocketUpgrade},TypedHeader},Json, response::{Html, IntoResponse, Response},routing::get,routing::post,routing::post_service,Router,http::{Uri, header::{self, HeaderMap, HeaderName}},extract::{Extension, FromRequest, RequestParts}};
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

async fn qwjs() -> impl IntoResponse{
    let mut r=File::open("qw.js").unwrap();
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
        .unwrap_or_else(|_| "postgres://cqbsfwjelclezx:34504970f333d727a191365905486ac48abd898e1ad3d975b002890ea67a6f24@ec2-52-48-159-67.eu-west-1.compute.amazonaws.com:5432/d6jquit4idoppv".to_string());
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
        .route(
        "/ws", get(handlerws))
        .route(
        "/qw.js", get(qwjs))
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
    sqlx::Executor::execute(&pool, "CREATE TABLE users (ID    SERIAL PRIMARY KEY, username    TEXT NOT NULL, full_name    TEXT NOT NULL, created_at    timestamp , bio    TEXT NOT NULL, followers  integer[], follows  integer[])")
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
    sqlx::Executor::execute(&pool, &format!(r#"INSERT INTO users (username, full_name, created_at, bio, followers, follows) VALUES ({}, {}, 'now', {}, {}, {})"#, username2, full_name2, bio2, "'{-2}'", "'{-2}'")[..]).await.unwrap();
    response()
        .await.status(200)
        .body(Full::from("Done".to_string()))
        .unwrap()
}

async fn getusers(Extension(pool): Extension<PgPool>,
) -> impl IntoResponse{
    let qws = env::var("PORT")
        .unwrap_or_else(|_| "7878".to_string())
        .to_string();
    let s = sqlx::query("SELECT ID, username, full_name, created_at, bio FROM users ORDER BY ID")
        .fetch_all(&pool)
        .await
        .unwrap();
    let mut p=0;
    let mut q : i32=0;
    let mut r=String::new();
    let mut qw: Vec<i32>=Vec::new();
    r+=r#"<html><head>
</head><body>
<span id=wq hidden></span>
<p>Log in as User <input type="text" id=userid></input> <button id=user>Log in</button> <span id=qw></span></p>
<table>
    <thead>
        <tr>
            <th>ID</th>
<th>Username</th>
<th>Full Name</th>
<th>Created At</th>
<th>Bio</th>
<th>Followers</th>
<th></th>
        </tr>
    </thead>
<tbody>"#;
    while p<s.len()
    {
        r+="<tr><td>";
        q=s[p].get(0);
        qw.push(s[p].get(0));
        r+=&q.to_string();
        r+="<td>";
        r+=s[p].get(1);
        r+="<td>";
        r+=s[p].get(2);
        r+="<td>";
        let w : sqlx::types::chrono::NaiveDateTime=s[p].get(3);
        let mut wr=&mut w.to_string();
        wr.drain(wr.len()-7..);
        r+=&wr;
        r+="<td>";
        r+=s[p].get(4);
        r+=&format!("<td><span id=w{}>", qw[p])[..];
        let mut s : Vec<i32>= sqlx::query(&format!("SELECT followers FROM users WHERE ID = {}", q))
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
        let mut w=0;
        w+=1;
        let mut qwr=0;
        if(s.len()>0)
        {
            while w<s.len()
            {
                qwr+=1;
                w+=1;
            }
        }
        r+=&qwr.to_string();
        if qwr>0
        {
            r+=" (Followed by:";
        }
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
        if qwr>0
        {
            r+=")";
        }
        r+=&format!("</span><td><button id=r{}>Follow/Unfollow</button>", qw[p])[..];
        r+="\r\n</p>";
        p+=1;
    }

    r+="</tbody></table>";
    r+=format!(r#"<script>
var u;
var ws = new WebSocket("ws://localhost:{}/ws");
ws.addEventListener("message", sock);
function sockq(l)
        {{
            document.getElementById(document.getElementById("wq").innerText).innerText=l.data;
            ws.removeEventListener("message", sockq);
            ws.addEventListener("message", sock);
        }}
function sock(l)
        {{
            ws.removeEventListener("message", sock);
            document.getElementById("wq").innerText="w"+l.data;
            ws.addEventListener("message", sockq);


        }};
user.addEventListener("mouseup", userq);
function userq(l)
        {{
u=document.getElementById("userid").value;
document.getElementById("userid").value="";
document.getElementById("userid").disabled=2;
        document.getElementById("user").disabled=2;
        document.getElementById("qw").innerText="Logged in as User "+u;
        }}"#, qws);
let mut p=0;
while p<s.len()
    {
        r+=&format!(r#"
        r{}.addEventListener("mouseup", e{});"#, qw[p], qw[p])[..];
        p+=1;
    }

let mut p=0;
while p<s.len()
    {
        r+=&format!(r#"
function e{}(l)
        {{
        ws.send({});
        ws.send(u);
        }}"#, qw[p], qw[p])[..];
        p+=1;
    }

r+="</script>
            </body>
        </html>
        ";

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
    sqlx::Executor::execute(&pool, &format!(r#"INSERT INTO users (username, full_name, created_at, bio, followers, follows) VALUES ('{}', '{}', 'now', '{}', {}, {})"#, username, full_name, bio, "'{-2}'", "'{-2}'")[..]).await.unwrap();
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
    let mut s : Vec<i32>= sqlx::query(&format!("SELECT follows FROM users WHERE ID = {}", id))
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
    let mut r=0;
    r+=1;
    while r<s.len()
    {
        let mut sw : Vec<i32>= sqlx::query(&format!("SELECT followers FROM users WHERE ID = {}", s[r]))
            .fetch_one(&pool)
            .await
            .unwrap()
            .get(0);
        let mut rq=0;
        let mut q=false;
        let mut w=rq.clone();
        while rq<sw.len()
        {
            if sw[rq]==id
            {
                q=true;
                w=rq;
            }
            rq+=1;
        }
        if q==true
        {
            sw.drain(w..w+1);
        }
        let mut sm=String::new();
        sm+="UPDATE users SET followers = '{";
        rq=0;
        while rq<sw.len()
        {
            sm+=&sw[rq].to_string();
            if rq+1<sw.len()
            {
                sm+=",";
            }
            rq+=1;
        }
        sm+="}'";
        sm+=&format!(" WHERE ID = {}", s[r]);
        sqlx::Executor::execute(&pool, &sm[..]).await.unwrap();
        r+=1;
    }
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
    let followedid : i32=q.0.followed.parse().unwrap();
let followerid : i32=q.0.followed.parse().unwrap();
    let mut s : Vec<i32>= sqlx::query(&format!("SELECT followers FROM users WHERE ID = {}", followedid))
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
    let mut sq : Vec<i32>= sqlx::query(&format!("SELECT followers FROM users WHERE ID = {}", followerid))
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
    let mut sw : Vec<i32>= sqlx::query(&format!("SELECT follows FROM users WHERE ID = {}", followerid))
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
    let mut r=0;
    let mut q=false;
    let mut w=r.clone();
    while r<sw.len()
    {
        if sw[r]==followedid
        {
            q=true;
            w=r;
        }
        r+=1;
    }
    if q==true
    {
        sw.drain(w..w+1);
    }
    else if followedid!=followerid
    {
        sw.push(followedid);
    }
    let mut sm=String::new();
    sm+="UPDATE users SET follows = '{";
    r=0;
    while r<sw.len()
    {
        sm+=&sw[r].to_string();
        if r+1<sw.len()
        {
            sm+=",";
        }
        r+=1;
    }
    sm+="}'";
    sm+=&format!(" WHERE ID = {}", followerid);
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
        r+=" | Username: ";
        r+=s[p].get(1);
        r+=" | Full name: ";
        r+=s[p].get(2);
        r+=" | Created_at: ";
        let w : sqlx::types::chrono::NaiveDateTime=s[p].get(3);
        r+=&w.to_string();
        r+=" | Bio: ";
        r+=s[p].get(4);
        r+=" | Followers: ";
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
                <p><a href=/>Index</a></p>
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

async fn handlerws(Extension(pool): Extension<PgPool>,ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("ef");
    ws.on_upgrade(move |mut sock| async move{
    while let followedid=sock.recv().await.unwrap().unwrap().into_text().unwrap().parse().unwrap()
{
    let followerid : i32=sock.recv().await.unwrap().unwrap().into_text().unwrap().parse().unwrap();
println!("{}", followedid);
println!("{}", followerid);
    let mut s : Vec<i32>= sqlx::query(&format!("SELECT followers FROM users WHERE ID = {}", followedid))
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
    let mut sq : Vec<i32>= sqlx::query(&format!("SELECT followers FROM users WHERE ID = {}", followerid))
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
    let mut sw : Vec<i32>= sqlx::query(&format!("SELECT follows FROM users WHERE ID = {}", followerid))
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
    let mut r=0;
    let mut q=false;
    let mut w=r.clone();
    while r<sw.len()
    {
        if sw[r]==followedid
        {
            q=true;
            w=r;
        }
        r+=1;
    }
    if q==true
    {
        sw.drain(w..w+1);
    }
    else if followedid!=followerid
    {
        sw.push(followedid);
    }
    let mut sm=String::new();
    sm+="UPDATE users SET follows = '{";
    r=0;
    while r<sw.len()
    {
        sm+=&sw[r].to_string();
        if r+1<sw.len()
        {
            sm+=",";
        }
        r+=1;
    }
    sm+="}'";
    sm+=&format!(" WHERE ID = {}", followerid);
    sqlx::Executor::execute(&pool, &sm[..]).await.unwrap();
sock.send(axum::extract::ws::Message::Text(followedid.to_string())).await.unwrap();
let mut r=String::new();
println!("{}", followedid);
let mut s : Vec<i32>= sqlx::query(&format!("SELECT followers FROM users WHERE ID = {}", followedid))
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
if(qw>0)
        {
        r+=" (Followed by:";
        w=0;
}
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
            }if(qw>0)
        {
r+=")";
        }}
sock.send(axum::extract::ws::Message::Text(r)).await.unwrap();
}})
}
