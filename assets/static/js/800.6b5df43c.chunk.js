"use strict";(self.webpackChunkexpenses_tracker=self.webpackChunkexpenses_tracker||[]).push([[800],{333:(t,e,n)=>{n.d(e,{A:()=>c});var a=n(43),r=n(825),s=n(579);const c=function(t){const[e,n]=(0,a.useState)(!1);return[e?"text":"password",(0,s.jsx)("button",{type:"button",className:t,onClick:()=>n((t=>!t)),children:e?(0,s.jsx)(r.cqH,{}):(0,s.jsx)(r.zgv,{})})]}},800:(t,e,n)=>{n.r(e),n.d(e,{default:()=>h});var a=n(43),r=n(865),s=n(861),c=n(462),o=n(394),u=n(216),l=n(720),i=n(333),d=n(579);const h=function(){const[{username:t,password:e,passwordConfirm:n},h]=(0,a.useState)({username:"",password:"",passwordConfirm:""}),[f,R]=(0,a.useState)(!1),p=(0,a.useRef)(null),E=(0,u.Zp)();(0,a.useEffect)((()=>{(0,r.Nj)().then((t=>{switch(t){case s.s.OK:return E("/tracker");case s.s.UNAUTHORIZED:return;default:throw new Error("Recieved an unexpected status code :: ".concat(t,"."))}})).catch(alert)}),[E]);const x=t=>{var e;const{name:n,value:a}=t.target;null===(e=p.current)||void 0===e||e.setCustomValidity(""),h((t=>({...t,[n]:a})))},[w,m]=(0,i.A)("text-2xl text-gray-400 text-2xl text-gray-400");return(0,d.jsxs)("form",{onSubmit:n=>{n.preventDefault(),R(!0),(0,r.Hx)(t,e).then((t=>{var e,n;switch(t){case s.s.CREATED:return window.location.replace("/tracker");case s.s.CONFLICT:return null===(e=p.current)||void 0===e||e.setCustomValidity("Username already exists."),void(null===(n=p.current)||void 0===n||n.reportValidity());default:throw new Error("Recieved an unexpected status code :: ".concat(t,"."))}})).catch(alert).finally((()=>R(!1)))},className:"h-full max-w-[720px] mx-auto px-4 py-20 flex flex-col",children:[(0,d.jsx)("h1",{className:"flex justify-center text-9xl mb-8 text-blue-600 drop-shadow-md",children:(0,d.jsx)(c.EcI,{})}),(0,d.jsxs)("div",{className:"flex flex-col gap-4 mb-auto",children:[(0,d.jsxs)("div",{className:"flex bg-gray-200 p-3 px-5 gap-3 rounded-full shadow-md",children:[(0,d.jsx)(o.sLb,{className:"text-2xl text-gray-400"}),(0,d.jsx)("input",{name:"username",value:t,onChange:x,ref:p,placeholder:"Username",type:"text",required:!0,pattern:"^[a-zA-Z0-9_]{3,20}$",title:"Username must be minimum of 3 and a max of 20 characters long and contain only letters (uppercase or lowercase), digits, or underscores.",className:"bg-transparent w-full outline-none placeholder:text-gray-400"})]}),(0,d.jsxs)("div",{className:"flex bg-gray-200 p-3 px-5 gap-3 rounded-full shadow-md",children:[(0,d.jsx)(l.nMq,{className:"text-2xl text-gray-400"}),(0,d.jsx)("input",{name:"password",value:e,onChange:x,placeholder:"Password",type:w,required:!0,pattern:"^(?=.*\\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[\\w]).{8,}$",title:"Password must contain at least one digit, one lowercase letter, one uppercase letter, one special character, and be at least 8 characters long.",className:"bg-transparent w-full outline-none placeholder:text-gray-400"}),m]}),(0,d.jsxs)("div",{className:"flex bg-gray-200 p-3 px-5 gap-3 rounded-full shadow-md",children:[(0,d.jsx)(l.nMq,{className:"text-2xl text-gray-400"}),(0,d.jsx)("input",{name:"passwordConfirm",value:n,placeholder:"password Confirm",type:w,required:!0,pattern:"^".concat(e,"$"),title:"Password does not match.",onChange:x,className:"bg-transparent w-full outline-none placeholder:text-gray-400"}),m]})]}),(0,d.jsxs)("div",{className:"flex flex-col gap-4 items-center",children:[(0,d.jsx)("button",{disabled:f,type:"submit",className:"text-white font-bold tracking-wider py-3 w-1/2 rounded-full shadow-md bg-blue-500 hover:bg-blue-600 transition-colors disabled:bg-gray-400",children:"Sign Up"}),(0,d.jsx)("a",{href:"/login",className:"bg-transparent text-blue-500 font-bold tracking-wider text-center py-3 w-1/2 rounded-full hover:text-blue-600 transition-colors",children:"Login"})]})]})}},865:(t,e,n)=>{n.d(e,{AK:()=>T,Ee:()=>j,Hx:()=>w,Kq:()=>_,Nj:()=>x,_G:()=>b,az:()=>m,bz:()=>A,dT:()=>g,fQ:()=>y,w2:()=>N});var a=n(923),r=n(861);const s="".concat("https://expenses-tracker.shuttleapp.rs/api","/user"),c="".concat(s,"/auth"),o="".concat(c,"/tracker"),u="".concat(o,"/category"),l="".concat(o,"/categories"),i={deep:!0},d={headers:{"Content-Type":"application/json"},credentials:"include"},h={...d,method:"POST"},f={...d,method:"GET"},R={...d,method:"DELETE"},p={...d,method:"PATCH"},E="The server encountered an error. Please try again later.";async function x(){return fetch("".concat(c),f).then((t=>{if(t.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);return t.status}))}async function w(t,e){return fetch("".concat(s,"/sign-up"),{...h,body:JSON.stringify({username:t,password:e})}).then((t=>{if(t.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);return t.status}))}async function m(t,e){return fetch("".concat(s,"/login"),{...h,body:JSON.stringify({username:t,password:e})}).then((t=>{if(t.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);return t.status}))}async function y(){return fetch("".concat(s,"/logout"),R).then((t=>{if(t.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);return t.status}))}async function g(){return fetch("".concat(l),f).then((async t=>{if(t.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);return[(0,a.A)(await t.json(),i),t.status]}))}async function N(t){return fetch("".concat(u,"/").concat(t,"/transactions"),f).then((async t=>{if(t.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);const e=(0,a.A)(await t.json(),i);return e.transactions=e.transactions.map((t=>({...t,createdAt:new Date(t.createdAt)}))),[e,t.status]}))}async function b(t,e){return fetch("".concat(l),{...h,body:JSON.stringify({name:t,icon:e})}).then((async n=>{if(n.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);return[{...await n.json(),name:t,icon:e,transactions:0,amount:0,precent:0,color:"#000000"},n.status]}))}async function _(t,e,n){return fetch("".concat(u,"/").concat(t,"/transactions"),{...h,body:JSON.stringify({amount:n,description:e})}).then((async t=>{if(t.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);return t.status}))}async function A(t,e,n){return fetch("".concat(u,"/").concat(t),{...p,body:JSON.stringify({name:e,icon:n})}).then((t=>{if(t.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);return t.status}))}async function j(t){return fetch("".concat(l,"/").concat(t),R).then((t=>{if(t.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);return t.status}))}async function T(t,e){return fetch("".concat(u,"/").concat(t,"/transactions/").concat(e),R).then((t=>{if(t.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(E);return t.status}))}},861:(t,e,n)=>{n.d(e,{s:()=>a});const a={OK:200,CREATED:201,CONFLICT:409,BAD_REQUEST:400,UNAUTHORIZED:401,INTERNAL_SERVER_ERROR:500}}}]);
//# sourceMappingURL=800.6b5df43c.chunk.js.map