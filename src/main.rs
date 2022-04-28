use std::io::prelude::*;
use axum::{extract::{ws::{Message as wMes, WebSocket, WebSocketUpgrade},TypedHeader},Json, response::{Html, IntoResponse, Response},routing::get,routing::post,routing::post_service,Router,http::{Uri, header::{self, HeaderMap, HeaderName}},extract::{Extension, FromRequest, RequestParts}};
use std::fs::File;
use std::fs;
use std::str::FromStr;
use std::env;
use rand::Rng;
use std::thread;
use axum::body::Full;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;
use std::{net::SocketAddr, time::Duration};
use serde::{Serialize, Deserialize};
use serde::ser::{SerializeStruct, Serializer};
use sqlx::postgres::types::PgTimeTz;
use postgres::types::Type;
use axum::extract::Form;
use rdkafka::config::ClientConfig;
use rdkafka::message::{OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::producer::{BaseProducer, BaseRecord};
use rdkafka::util::get_rdkafka_version;
use rdkafka::client::ClientContext;
use rdkafka::producer::DefaultProducerContext;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::producer::ThreadedProducer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::consumer::DefaultConsumerContext;
use rdkafka::consumer::BaseConsumer;
use sqlx::Either::Left;
use sqlx::Either::Right;
use std::pin::Pin;
use futures_core::stream::Stream;
use futures_core::task::Poll::Ready;
use futures_core::task::Waker;
use rdkafka::producer::Producer;
use futures_util::StreamExt;
use futures_util::Future;
use futures_util::poll;
use futures::executor::block_on;

#[derive(Serialize, Deserialize)]
struct User
{
    username: String,
    full_name: String,
    bio: String
}

#[derive(Serialize, Deserialize)]
struct Follow
{
    followerid: i32,
    followedid: i32
}

#[derive(Serialize, Deserialize)]
struct Followq
{
    followedid: i32,
    followers: String
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
        .unwrap_or_else(|_| "postgres://ycxoygomdkgjda:4d1e5d861e8844d4b740a932b32bfac17c36625ef46d9460596497d8a9a84d91@ec2-34-247-72-29.eu-west-1.compute.amazonaws.com:5432/d24bkboe7oopjq".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .unwrap();
    let pool2 = pool.clone();
    let app = Router::new()
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
        .layer(Extension(pool2))
        .route(
        "/ws", get(handlerws2))
        .route(
        "/ws2", get(handlerws))
        .route(
        "/qw.js", get(qwjs))
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
        "/pkg/webgl_bg.wasm", get(pkgbg));
    let q = env::var("PORT")
        .unwrap_or_else(|_| "80".to_string())
        .to_string();
    let brokers = "127.0.0.1:9092";
    let topic = "follows";
    let group_id = "group_id";
    let mut topics : Vec<&str> = Vec::new();
    topics.push(topic);
    tokio::spawn(async move {
    println!("2none");
    let consumer: BaseConsumer<DefaultConsumerContext> = ClientConfig::new()
        .set("group.id", "follower")
        .set("bootstrap.servers", "127.0.0.1:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .create_with_context(DefaultConsumerContext)
        .expect("Consumer creation failed");
    let producerq: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9092")
        .set("message.timeout.ms", "5420")
        .create()
        .expect("Producer creation error");
    let mut topics : Vec<&str> = Vec::new();
    topics.push("follows");
    consumer.subscribe(&topics).unwrap();
    let mut s : String = String::new();
    let mut fs : Vec<Follow> = Vec::new();
    let mut wq : Vec<Followq> = Vec::new();
    let mut sm : String = String::new();
    let mut l=0;
    loop 
    {
        let q = match consumer.poll(Duration::from_millis(222)){
                    None => 
                    {
                        let mut lw=0;
                        let mut qlw=0;
                        s+="SELECT followers FROM users WHERE ID = 3\r\n";
                        let mut wsq : String = s.clone();
                        let mut r = sqlx::query(&wsq).fetch_many(&pool);
                        while lw<l
                        {
                            qlw=lw/3;
                                let mut sqw : Vec<i32>= r.next().await.unwrap().unwrap().right().unwrap().get(0);
                                lw+=1;
                                let mut sq : Vec<i32>= r.next().await.unwrap().unwrap().right().unwrap().get(0);
                                lw+=1;
                                let mut sw : Vec<i32>= r.next().await.unwrap().unwrap().right().unwrap().get(0);
                                lw+=1;
                                sm+="UPDATE users SET followers = '{";
                                let mut followerid = fs[qlw].followerid;
                                let mut followedid = fs[qlw].followedid;
                                let mut r=0;
                                let mut q=false;
                                let mut w=r.clone();
                                while r<sqw.len()
                                {
                                    if sqw[r]==followerid
                                    {
                                        q=true;
                                        w=r;
                                    }
                                    r+=1;
                                }
                                if q==true
                                {
                                    sqw.drain(w..w+1);
                                }
                                else if followerid!=followedid
                                {
                                    sqw.push(followerid);
                                }
                                let mut r=0;
                                let mut wsqw : Vec<i32> = Vec::new();
                                while r<sqw.len()
                                {
                                    wsqw.push(sqw[r]);
                                    r+=1;
                                }
                                let mut smq : String = String::new();
                                smq+=&(&wsqw.len()-1).to_string();
                                smq+=" (Followed by: ";
                                r+=1;
                                let mut r=0;
                                r+=1;
                                while r<wsqw.len()
                                {
                                    smq+="User ";
                                    smq+=&wsqw[r].to_string();
                                    if r+1<wsqw.len()
                                    {
                                        smq+=", ";
                                    }
                                    r+=1;
                                }
                                let mut r=0;
                                while r<sqw.len()
                                {
                                    sm+=&sqw[r].to_string();
                                    if r+1<sqw.len()
                                    {
                                        sm+=",";
                                    }
                                    r+=1;
                                }
                                smq+=")";
                                sm+="}'";
                                sm+=&format!(" WHERE ID = {} ;\r\n", followedid);
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
                                println!("{}", smq);
                                wq.push(Followq{followedid: followedid, followers: smq.clone()});
                                sm+="}'";
                                sm+=&format!(" WHERE ID = {} ;\r\n", followerid);
                            }
                            sm+="UPDATE users SET ID = 3 WHERE ID = 3\r\n";
                                    println!("{}", &((sm)[..]));
                                let mut sl=sm.clone();
                                let mut q=sqlx::Executor::execute_many(&pool, &(sl[..]));
                                let mut r=0;
                                println!("{}", &wq.len());
                                while r<wq.len()
                                {
                                    println!("{:?}", &wq[r].followers);
                                    producerq.send(FutureRecord::to("followsq").payload(&format!(r#""followedid":{}, "followers":"{}""#, wq[r].followedid, wq[r].followers)).key(&format!("2")), Duration::from_secs(0)).await.unwrap();
                                    r+=1;
                                }
                                
                        let mut qw=q.next().await.unwrap().unwrap();
                        
                        l=0;
                        wq.clear();
                        s.clear();
                        fs.clear();
                        sm.clear();
                    },
                    Some(Ok(w)) => 
                    {
                        if l<75
                        {
                            let qw = w.payload_view::<str>().unwrap().unwrap();
                            println!("{}", (&("{".to_owned()+&qw+"}")));
                            let follow : Follow = serde_json::from_str(&("{".to_owned()+&qw+"}")).unwrap();
                            s+=&format!("SELECT followers FROM users WHERE ID = {} UNION ALL\r\n", follow.followedid)[..];
                            s+=&format!("SELECT followers FROM users WHERE ID = {} UNION ALL\r\n", follow.followerid)[..];
                            s+=&format!("SELECT follows FROM users WHERE ID = {} UNION ALL\r\n", follow.followerid)[..];
                            fs.push(follow);
                            l+=1;
                            consumer.commit_message(&w, CommitMode::Async).unwrap();
                        }
                        else
                        {
                            let mut lw=0;
                            let mut qlw=0;
                            s+="SELECT followers FROM users WHERE ID = 3\r\n";
                            let mut wsq : String = s.clone();
                            let mut r = sqlx::query(&wsq).fetch_many(&pool);
                            while lw<l
                            {
                                qlw=lw/3;
                                    let mut sqw : Vec<i32>= r.next().await.unwrap().unwrap().right().unwrap().get(0);
                                    lw+=1;
                                    let mut sq : Vec<i32>= r.next().await.unwrap().unwrap().right().unwrap().get(0);
                                    lw+=1;
                                    let mut sw : Vec<i32>= r.next().await.unwrap().unwrap().right().unwrap().get(0);
                                    lw+=1;
                                    sm+="UPDATE users SET followers = '{";
                                    let mut followerid = fs[qlw].followerid;
                                    let mut followedid = fs[qlw].followedid;
                                    let mut r=0;
                                    let mut q=false;
                                    let mut w=r.clone();
                                    while r<sqw.len()
                                    {
                                        if sqw[r]==followerid
                                        {
                                            q=true;
                                            w=r;
                                        }
                                        r+=1;
                                    }
                                    if q==true
                                    {
                                        sqw.drain(w..w+1);
                                    }
                                    else if followerid!=followedid
                                    {
                                        sqw.push(followerid);
                                    }
                                    let mut r=0;
                                    let mut wsqw : Vec<i32> = Vec::new();
                                    while r<sqw.len()
                                    {
                                        wsqw.push(sqw[r]);
                                        r+=1;
                                    }
                                    let mut smq : String = String::new();
                                    smq+=&(&wsqw.len()-1).to_string();
                                    smq+=" (Followed by: ";
                                    r+=1;
                                    let mut r=0;
                                    r+=1;
                                    while r<wsqw.len()
                                    {
                                        smq+="User ";
                                        smq+=&wsqw[r].to_string();
                                        if r+1<wsqw.len()
                                        {
                                            smq+=", ";
                                        }
                                        r+=1;
                                    }
                                    let mut r=0;
                                    while r<sqw.len()
                                    {
                                        sm+=&sqw[r].to_string();
                                        if r+1<sqw.len()
                                        {
                                            sm+=",";
                                        }
                                        r+=1;
                                    }
                                    smq+=")";
                                    sm+="}'";
                                    sm+=&format!(" WHERE ID = {} ;\r\n", followedid);
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
                                    println!("{}", smq);
                                    wq.push(Followq{followedid: followedid, followers: smq.clone()});
                                    sm+="}'";
                                    sm+=&format!(" WHERE ID = {} ;\r\n", followerid);
                                }
                                sm+="UPDATE users SET ID = 3 WHERE ID = 3\r\n";
                                        println!("{}", &((sm)[..]));
                                    let mut sl=sm.clone();
                                    let mut q=sqlx::Executor::execute_many(&pool, &(sl[..]));
                                    let mut r=0;
                                    println!("{}", &wq.len());
                                    while r<wq.len()
                                    {
                                        println!("{:?}", &wq[r].followers);
                                        producerq.send(FutureRecord::to("followsq").payload(&format!(r#""followedid":{}, "followers":"{}""#, wq[r].followedid, wq[r].followers)).key(&format!("2")), Duration::from_secs(0)).await.unwrap();
                                        r+=1;
                                    }
                                
                            let mut qw=q.next().await.unwrap().unwrap();
                        
                            l=0;
                            wq.clear();
                            s.clear();
                            fs.clear();
                            sm.clear();
                        }
                    },
                    Some(Err(w)) => 
                    {
                        println!("Bad.");
                        consumer.poll(Duration::from_millis(8422)).unwrap();
                    }
                };
    }
;
    });
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
    let s = sqlx::query("SELECT ID, username, full_name, created_at, bio FROM users ORDER BY ID")
        .fetch_all(&pool)
        .await
        .unwrap();
    let mut sq=String::new();
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
    while p<s.len()-7/7
    {
        q=s[p].get(0);
        sq+=&format!("SELECT followers FROM users WHERE ID = {} UNION ALL\r\n", q)[..];
        p+=1;
    }
    q=s[p].get(0);
    sq+=&format!("SELECT followers FROM users WHERE ID = {}", q)[..];
    println!("{}", sq);
    let mut sw = sqlx::query(&sq).fetch_many(&pool);
    p=0;
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
        let mut s : Vec<i32>= sw.next().await.unwrap().unwrap().right().unwrap().get(0);
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
    let mut p=String::new();
    p+="wss://axumsersql4.herokuapp.com/ws";
    env::var("PORT").unwrap_or_else(|_| {p="ws://localhost/ws".to_string(); "80".to_string()});
    r+=&format!(r#"<script>
var u;
var ws = new WebSocket("{}");
var ws2 = new WebSocket("{}2");


ws2.addEventListener("message", sock);
function sock(l)
        {{   document.getElementById("wq").innerHTML=l.data;         
            var w=JSON.parse('{{'+l.data+'}}');
            

            document.getElementById("w"+w.followedid).innerText=w.followers;
        }};
user.addEventListener("mouseup", userq);
function userq(l)
        {{
u=document.getElementById("userid").value;
document.getElementById("userid").value="";
document.getElementById("userid").disabled=2;
        document.getElementById("user").disabled=2;
        document.getElementById("qw").innerText="Logged in as User "+u;
        }}"#, p, p)[..];
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
            
        ws.send('{{"followerid":'+u+', "followedid":{}}}');
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
r#"""#;
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

async fn handlerws(ws: WebSocketUpgrade) -> impl IntoResponse {

        ws.on_upgrade(move |mut sock| async move{
        
            let consumer: BaseConsumer<DefaultConsumerContext> = ClientConfig::new()
                .set("group.id", &("follow".to_owned()+&rand::thread_rng().gen_range(324..324324824 as i32).to_string()))
                .set("bootstrap.servers", "127.0.0.1:9092")
                .set("enable.partition.eof", "false")
                .set("session.timeout.ms", "6000")
                .set("enable.auto.commit", "true")
                //.set("statistics.interval.ms", "30000")
                //.set("auto.offset.reset", "smallest")
                .create_with_context(DefaultConsumerContext)
                .expect("Consumer creation failed");
            let mut topics : Vec<&str> = Vec::new();
            topics.push("followsq");
            consumer.subscribe(&topics).unwrap();
            loop
            {
                let q = match consumer.poll(Duration::from_millis(822)){
                            None => 
                            {
                        
                            },
                            Some(Ok(w)) => 
                            {
                                sock.send(axum::extract::ws::Message::Text(w.payload_view::<str>().unwrap().unwrap().to_string())).await.unwrap();
                                consumer.commit_message(&w, CommitMode::Async).unwrap();
                            },
                            Some(Err(w)) => 
                            {

                            }
                        };
            }
    })
}

async fn handlerws2(ws: WebSocketUpgrade) -> impl IntoResponse {

        ws.on_upgrade(move |mut sock| async move{
        
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "127.0.0.1:9092")
            .set("message.timeout.ms", "5420")
            .create()
            .expect("Producer creation error");
    loop 
{
    let follow : Follow=serde_json::from_str(&sock.recv().await.unwrap().unwrap().into_text().unwrap()).unwrap();
    println!(r#""followerid": {}, "followedid": {}"#, follow.followerid, follow.followedid);
producer.send(FutureRecord::to("follows").payload(&format!(r#""followerid": {}, "followedid": {}"#, follow.followerid, follow.followedid)[..]).key(&format!("2")), Duration::from_secs(0)).await.unwrap();
}
    
})
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

async fn consume(brokers: &str, group_id: &str, topics: &[&str]) {
    let context = DefaultConsumerContext;

    let consumer: StreamConsumer<DefaultConsumerContext> = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

async fn produce(brokers: &str, topic_name: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "50")
        .create()
        .expect("Producer creation error");

    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..5)
        .map(|i| async move {
            // The send operation on the topic returns a future, which will be
            // completed once the result or failure from Kafka is received.
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        ,
                    Duration::from_secs(0),
                )
                .await;

            // This will be executed when the result is received.
            println!("Delivery status for message {} received", i);
            delivery_status
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        println!("Future completed. Result: {:?}", future.await);
    }
}