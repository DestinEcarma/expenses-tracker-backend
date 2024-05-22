"use strict";(self.webpackChunkexpenses_tracker=self.webpackChunkexpenses_tracker||[]).push([[251],{251:(e,t,n)=>{n.r(t),n.d(t,{default:()=>p});var s=n(43);const r=(0,s.createContext)(""),a=(0,s.createContext)([]),o=(0,s.createContext)(void 0);var c=n(475),i=n(216),l=n(861),d=n(865),u=n(596),h=n(156),f=n(579);const x=function(e){let{id:t}=e;const[n,a]=(0,s.useState)(0),c=(0,s.useContext)(r),i=function(){const e=(0,s.useContext)(o);if(void 0===e)throw new Error("useTransactionsContext must be used within a SetTransactionsContext");return e}(),l=()=>{setTimeout((()=>a(0)),200),a(-1)};return(0,f.jsxs)(f.Fragment,{children:[0!==n&&(0,f.jsx)("div",{"data-open":n,className:"z-50 fixed top-0 left-0 flex justify-center items-center w-full h-dvh bg-gray-500/30 backdrop-blur-sm data-[open='-1']:animate-fade-out data-[open='1']:animate-fade-in",children:(0,f.jsx)("form",{onSubmit:async e=>{try{e.preventDefault(),200===await(0,d.AK)(c,t)&&i((e=>e.filter((e=>e.id!==t)))),l()}catch(n){console.error(n)}},"data-open":n,className:"absolute w-max data-[open='-1']:animate-fade-out data-[open='1']:animate-fade-in",children:(0,f.jsxs)("div",{className:"left-0 flex flex-col gap-4 w-max bg-white p-4 rounded-md drop-shadow-md",children:[(0,f.jsx)("h1",{className:"text-center text-black text-xl font-bold",children:"Are you sure"}),(0,f.jsx)("p",{className:"text-gray-400",children:"This action cannot be undone."}),(0,f.jsxs)("div",{className:"flex-grow flex justify-between font-bold text-white",children:[(0,f.jsx)("button",{onClick:l,type:"button",className:"w-1/4 py-2 bg-blue-500 hover:bg-blue-600 rounded-md transition-colors shadow-md",children:"No"}),(0,f.jsx)("button",{type:"submit",className:"w-1/4 py-2 bg-red-500 hover:bg-red-600 rounded-md transition-colors shadow-md",children:"Yes"})]})]})})}),(0,f.jsx)("button",{onClick:()=>a((e=>0===e?1:(setTimeout((()=>a(0)),200),-1*e))),className:"p-2 text-white bg-red-500 hover:bg-red-600 rounded-full shadow-md transition-colors",children:(0,f.jsx)(h.qbC,{})})]})},m="default",R={day:"numeric"},w={weekday:"short"},E={hour12:!0,hour:"2-digit",minute:"2-digit"};function N(e){let{id:t,description:n,amount:s,createdAt:r,showLine:a}=e;return(0,f.jsxs)("div",{className:"flex gap-4 px-4 last:mb-4",children:[(0,f.jsxs)("div",{className:"flex flex-col justify-center items-center aspect-square h-20 text-gray-300 font-bold rounded-full border-8 border-gray-300 drop-shadow-md hover:text-gray-400 transition-colors",children:[(0,f.jsx)("h1",{className:"text-xl leading-[0.8]",children:r.toLocaleDateString(m,w)}),(0,f.jsx)("h2",{className:"text-4xl leading-[0.8]",children:r.toLocaleDateString(m,R)})]}),(0,f.jsxs)("div",{className:"relative flex-grow flex justify-between gap-4",children:[(0,f.jsxs)("div",{className:"flex-grow flex flex-col justify-center",children:[(0,f.jsx)("span",{className:"font-bold",children:r.toLocaleTimeString(m,E)}),(0,f.jsx)("span",{className:"text-gray-400",children:n})]}),(0,f.jsxs)("div",{className:"flex-grow flex flex-col items-end justify-center",children:[(0,f.jsxs)("span",{className:"text-right text-red-500 font-bold",children:["\u20b1",(0,u.U)(s)]}),(0,f.jsx)(x,{id:t})]}),a&&(0,f.jsx)("hr",{className:"absolute -bottom-2 border-1 border-gray-500 w-full"})]})]})}const g=function(){const e=(0,s.useContext)(a);return(0,f.jsx)("div",{className:"flex flex-col gap-4 h-full overflow-y-auto no-scrollbar",children:e.sort(((e,t)=>{const n=new Date(e.createdAt);return new Date(t.createdAt).getTime()-n.getTime()})).map(((t,n)=>(0,f.jsx)(N,{...t,showLine:n!==e.length-1},t.id)))})};const p=function(){const[e,t]=(0,s.useState)([]),[n,x]=(0,s.useState)(""),[m,R]=(0,s.useState)(0),[w,E]=(0,s.useState)("none"),[N]=(0,c.ok)(),p=(0,i.Zp)();(0,s.useEffect)((()=>{const e=N.get("id");console.log(e),e?x(e):p("/tracker")}),[N,p]),(0,s.useEffect)((()=>{n&&(0,d.w2)(n).then((e=>{let[{category:n,transactions:s},r]=e;switch(r){case l.s.OK:return E(n.name),t(s),void R(s.reduce(((e,t)=>e+t.amount),0));case l.s.UNAUTHORIZED:return p("/login");default:throw new Error("Recieved an unexpected status code :: ".concat(r,"."))}})).catch(alert)}),[n,p]),(0,s.useEffect)((()=>{R(e.reduce(((e,t)=>e+t.amount),0))}),[e]);const y=(new Date).toLocaleString("default",{month:"short",year:"numeric"});return(0,f.jsxs)("div",{className:"flex flex-col h-full max-h-full max-w-[720px] mx-auto",children:[(0,f.jsxs)("div",{className:"relative mb-4 py-4",children:[(0,f.jsx)("a",{href:"/tracker",className:"absolute top-1/2 -translate-y-1/2 ml-4 text-3xl",children:(0,f.jsx)(h.QVr,{})}),(0,f.jsx)("h1",{className:"font-bold text-center uppercase",children:w})]}),(0,f.jsxs)("div",{className:"flex flex-col gap-10 flex-grow overflow-auto",children:[(0,f.jsxs)("div",{className:"flex flex-col mx-auto w-min",children:[(0,f.jsx)("span",{className:"leading-none tracking-tight font-bold text-gray-400 w-max",children:y}),(0,f.jsxs)("span",{className:"mb-4 font-extrabold text-4xl w-min",children:["\u20b1",(0,u.U)(m)]})]}),(0,f.jsxs)("div",{className:"flex-grow overflow-hidden pb-4 h-full",children:[(0,f.jsx)(r.Provider,{value:n,children:(0,f.jsx)(o.Provider,{value:t,children:(0,f.jsx)(a.Provider,{value:e,children:(0,f.jsx)(g,{})})})}),(0,f.jsx)("hr",{className:"flex-grow mx-4 border-1 border-gray-500"})]})]})]})}},865:(e,t,n)=>{n.d(t,{AK:()=>T,Ee:()=>S,Hx:()=>E,Kq:()=>j,Nj:()=>w,_G:()=>b,az:()=>N,bz:()=>v,dT:()=>p,fQ:()=>g,w2:()=>y});var s=n(923),r=n(861);const a="".concat("https://expenses-tracker.shuttleapp.rs/api","/user"),o="".concat(a,"/auth"),c="".concat(o,"/tracker"),i="".concat(c,"/category"),l="".concat(c,"/categories"),d={deep:!0},u={headers:{"Content-Type":"application/json"},credentials:"include"},h={...u,method:"POST"},f={...u,method:"GET"},x={...u,method:"DELETE"},m={...u,method:"PATCH"},R="The server encountered an error. Please try again later.";async function w(){return fetch("".concat(o),f).then((e=>{if(e.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);return e.status}))}async function E(e,t){return fetch("".concat(a,"/sign-up"),{...h,body:JSON.stringify({username:e,password:t})}).then((e=>{if(e.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);return e.status}))}async function N(e,t){return fetch("".concat(a,"/login"),{...h,body:JSON.stringify({username:e,password:t})}).then((e=>{if(e.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);return e.status}))}async function g(){return fetch("".concat(a,"/logout"),x).then((e=>{if(e.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);return e.status}))}async function p(){return fetch("".concat(l),f).then((async e=>{if(e.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);return[(0,s.A)(await e.json(),d),e.status]}))}async function y(e){return fetch("".concat(i,"/").concat(e,"/transactions"),f).then((async e=>{if(e.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);const t=(0,s.A)(await e.json(),d);return t.transactions=t.transactions.map((e=>({...e,createdAt:new Date(e.createdAt)}))),[t,e.status]}))}async function b(e,t){return fetch("".concat(l),{...h,body:JSON.stringify({name:e,icon:t})}).then((async n=>{if(n.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);return[{...await n.json(),name:e,icon:t,transactions:0,amount:0,precent:0,color:"#000000"},n.status]}))}async function j(e,t,n){return fetch("".concat(i,"/").concat(e,"/transactions"),{...h,body:JSON.stringify({amount:n,description:t})}).then((async e=>{if(e.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);return e.status}))}async function v(e,t,n){return fetch("".concat(i,"/").concat(e),{...m,body:JSON.stringify({name:t,icon:n})}).then((e=>{if(e.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);return e.status}))}async function S(e){return fetch("".concat(l,"/").concat(e),x).then((e=>{if(e.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);return e.status}))}async function T(e,t){return fetch("".concat(i,"/").concat(e,"/transactions/").concat(t),x).then((e=>{if(e.status===r.s.INTERNAL_SERVER_ERROR)throw new Error(R);return e.status}))}},861:(e,t,n)=>{n.d(t,{s:()=>s});const s={OK:200,CREATED:201,CONFLICT:409,BAD_REQUEST:400,UNAUTHORIZED:401,INTERNAL_SERVER_ERROR:500}},596:(e,t,n)=>{function s(e){return e.toLocaleString("en-US",{minimumFractionDigits:2})}n.d(t,{U:()=>s})}}]);
//# sourceMappingURL=251.2e93d3d6.chunk.js.map