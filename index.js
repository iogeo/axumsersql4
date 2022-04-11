import init, {start} from "./pkg/webgl.js";
var u, u2;
function c(l)
{
u-=l.movementX;
u2-=l.movementY;
document.getElementById("new").innerHTML=document.getElementById("old").innerHTML-u;
document.getElementById("new2").innerHTML=document.getElementById("old2").innerHTML-u2;
document.getElementById("f").innerHTML=3;
}
function e(l)
{
document.getElementById("new").innerHTML=l.clientX;
document.getElementById("new2").innerHTML=l.clientY;
document.getElementById("f").innerHTML=4;
document.getElementById("sw").innerHTML=document.getElementById("old").innerHTML;
document.getElementById("sr").innerHTML=document.getElementById("old2").innerHTML;
document.getElementById("swe").innerHTML=document.getElementById("new").innerHTML;
document.getElementById("sre").innerHTML=document.getElementById("new2").innerHTML;
canvas.removeEventListener("mousemove", c);
}
function a(l)
{
document.getElementById("sw").innerHTML=document.getElementById("old").innerHTML;
document.getElementById("sr").innerHTML=document.getElementById("old2").innerHTML;
document.getElementById("swe").innerHTML=document.getElementById("new").innerHTML;
document.getElementById("sre").innerHTML=document.getElementById("new2").innerHTML;
document.getElementById("old").innerHTML=l.clientX;
document.getElementById("old2").innerHTML=l.clientY;
document.getElementById("f").innerHTML=2;
u=0;
u2=0;
document.addEventListener("mouseup", e);
canvas.addEventListener("mousemove", c);
}
function z(l)
{
l.preventDefault();
var j;
j=document.getElementById("w").innerHTML;
j-=l.deltaY/25;
if(j<0)
j=0;
document.getElementById("w").innerHTML=j;
}
      init()
        .then(() => {
start();
canvas.addEventListener("mousedown", a);
canvas.addEventListener("wheel", z);
})
    