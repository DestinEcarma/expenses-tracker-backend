"use strict";(self.webpackChunkexpenses_tracker=self.webpackChunkexpenses_tracker||[]).push([[939],{939:(e,t,a)=>{a.r(t),a.d(t,{default:()=>k});var r=a(43);const s=(0,r.createContext)([]),n=(0,r.createContext)(void 0);function o(){const e=(0,r.useContext)(n);if(void 0===e)throw new Error("useSetCategoriesContext must be used within a SetCategoriesContext");return e}var c=a(698),l=a(865),i=a(861),d=a(216),u=a(596),f=a(156),h=a(184);const x={faArray:Object.entries(h),fa:{}};function m(e){const t=e.split(":");return x[t[0]][t[1]]}x.faArray.forEach((e=>{let[t,a]=e;return x.fa[t]=a}));var p=a(579);function b(e){let{setIcon:t}=e;const[a,s]=(0,r.useState)((()=>Object.fromEntries(x.faArray.map((e=>{let[t]=e;return[t,!1]}))))),[n,o]=(0,r.useState)(""),c=(0,r.useRef)([]);return(0,r.useEffect)((()=>{const e=new IntersectionObserver((e=>{e.forEach((e=>{e.isIntersecting&&s((t=>({...t,[e.target.id]:!0})))}))}),{root:null,rootMargin:"0px",threshold:0});return c.current.forEach((t=>{e.observe(t)})),()=>e.disconnect()}),[c]),(0,p.jsxs)(p.Fragment,{children:[(0,p.jsx)("input",{value:n,onChange:e=>o(e.target.value),placeholder:"Search...",className:"flex-grow p-2 border border-gray-500 rounded-md"}),(0,p.jsx)("div",{className:"flex-grow grid grid-cols-6 h-[300px] content-start rounded-md border border-gray-500 overflow-y-auto text-2xl items-center no-scrollbar",children:x.faArray.map((e=>{let[r,s]=e;return(0,p.jsx)("div",{"data-show":n.length<3||r.toLocaleLowerCase().includes(n.toLowerCase()),className:"w-full aspect-square flex justify-center items-center p-1 data-[show=false]:hidden",children:(0,p.jsx)("button",{onClick:()=>t("fa:".concat(r)),id:r,ref:e=>e&&c.current.push(e),type:"button",className:"flex justify-center items-center rounded shadow aspect-square w-full hover:bg-gray-200 hover:scale-110 transition-all",children:a[r]&&(0,p.jsx)(s,{})})},r)}))})]})}const g=function(e){let{setOffset:t,setIcon:a,icon:s}=e;const[n,o]=(0,r.useState)(0),c=m(s),l=(0,p.jsx)("div",{"data-open":n,className:"absolute left-0 top-0 -translate-y-full w-full data-[open='-1']:animate-fade-out data-[open='1']:animate-fade-in",children:(0,p.jsxs)("div",{className:"relative flex flex-col gap-4 w-full bg-white mb-4 rounded-md border border-gray-500 p-4",children:[(0,p.jsx)(b,{setIcon:e=>{a(e),1===n&&(setTimeout((()=>o(0)),200),t&&t(!1),o(-1))}}),(0,p.jsx)("div",{className:"absolute w-[15px] aspect-square rotate-45 -bottom-[1px] left-0 ml-3 translate-y-1/2 bg-white border-b border-r border-b-gray-500 border-r-gray-500"})]})});return(0,p.jsxs)(p.Fragment,{children:[0!==n&&l,(0,p.jsx)("button",{onClick:()=>{o((e=>0===e?(t&&t(!0),1):(setTimeout((()=>o(0)),200),t&&t(!1),-1*e)))},type:"button",className:"relative flex items-center justify-center border border-gray-500 hover:bg-gray-200 rounded-md p-2 h-full aspect-square transition-colors",children:(0,p.jsx)(c,{className:"text-2xl"})})]})};function w(e){let{setIsModalOpen:t,isModalOpen:a}=e;const[s,n]=(0,r.useState)("fa:FaHamburger"),[c,u]=(0,r.useState)(""),[f,h]=(0,r.useState)(!1),x=o(),m=(0,d.Zp)();return(0,p.jsx)("form",{onSubmit:e=>{e.preventDefault(),h(!0),(0,l._G)(c,s).then((e=>{let[t,a]=e;switch(a){case i.s.CREATED:return void x((e=>[...e,t]));case i.s.UNAUTHORIZED:return m("/login");default:throw new Error("Recieved an unexpected status code :: ".concat(a,"."))}})).catch(alert).then((()=>{t(-1),setTimeout((()=>t(0)),200)}))},"data-open":a,className:"absolute -translate-y-full -translate-x-1/2 left-1/2 flex w-dvw max-w-[400px] p-4 data-[open='-1']:animate-fade-out data-[open='1']:animate-fade-in",children:(0,p.jsxs)("div",{className:"relative bg-white p-4 rounded-lg flex flex-col flex-grow gap-4 drop-shadow-md",children:[(0,p.jsx)("h1",{className:"text-center text-xl font-extrabold",children:"Add Category"}),(0,p.jsxs)("div",{className:"relative flex flex-grow gap-4",children:[(0,p.jsx)(g,{icon:s,setIcon:n}),(0,p.jsx)("input",{value:c,onChange:e=>u(e.target.value),id:"name",name:"name",placeholder:"Name",required:!0,className:"flex-grow p-2 border border-gray-500 rounded-md"})]}),(0,p.jsx)("button",{disabled:f,type:"submit",className:"w-full text-center p-2 bg-green-500 hover:bg-green-600 rounded-md text-white font-bold transition-colors shadow-md disabled:bg-gray-400",children:"Submit"}),(0,p.jsx)("div",{className:"absolute w-[15px] aspect-square bottom-[1px] left-1/2 -translate-x-1/2 translate-y-1/2 rotate-45 bg-white"})]})})}const v=function(){const[e,t]=(0,r.useState)(0);return(0,p.jsxs)(p.Fragment,{children:[0!==e&&(0,p.jsx)("div",{"data-open":e,className:"w-full h-dvh fixed top-0 left-0 bg-gray-500/30 backdrop-blur-sm data-[open='-1']:animate-fade-out data-[open='1']:animate-fade-in"}),(0,p.jsxs)("div",{className:"relative",children:[0!==e&&(0,p.jsx)(w,{isModalOpen:e,setIsModalOpen:t}),(0,p.jsx)("button",{onClick:()=>t((e=>0===e?1:(setTimeout((()=>t(0)),200),-1*e))),"data-open":e,className:"rounded-full shadow-md text-white bg-blue-500 p-2 hover:bg-blue-600 data-[open='1']:rotate-45 data-[open='1']:bg-red-500 data-[open='1']:hover:bg-red-500 transition-all duration-500 ease-out",children:(0,p.jsx)(f.OiG,{className:"text-3xl"})})]})]})},j="style_parent__fLGai",y="style_outer-border__XzHln",N="style_inner-border__oUkG-",E="style_progress__VgYaJ";const R=function(e){let{percent:t=0,borderColor:a="#ff8000",borderWidth:s="8px",children:n}=e;const[o,c]=(0,r.useState)(0),[l,i]=(0,r.useState)(t);(0,r.useEffect)((()=>{l!==t&&(c(l),i(t))}),[t]);const d={"--percent":t+"%","--border-color":a,"--border-width":s},u={"--stroke-dasharray":"calc(2 * ".concat(Math.PI," * 45%)"),"--stroke-dashoffset":"calc(var(--stroke-dasharray) - var(--stroke-dasharray) * ".concat(t," / 100)"),"--stroke-before":"calc(var(--stroke-dasharray) - var(--stroke-dasharray) * ".concat(o," / 100)"),"--stroke-after":"calc(var(--stroke-dasharray) - var(--stroke-dasharray) * ".concat(l," / 100)")};return(0,p.jsxs)("div",{style:d,className:j,children:[(0,p.jsx)("div",{className:y,children:(0,p.jsx)("div",{className:N,children:(0,p.jsx)("div",{children:n})})}),(0,p.jsx)("svg",{xmlns:"http://www.w3.org/2000/svg",version:"1.1",className:E,children:(0,p.jsx)("circle",{cx:"50%",cy:"50%",r:"45%",className:E,style:u,transform:"rotate(-90 40 40)"})})]})};function S(e){let{closeModal:t,isModalOpen:a,name:s,id:n}=e;const[c,d]=(0,r.useState)(""),[u,f]=(0,r.useState)("0"),h=o(),x=(0,r.useRef)(null),m=/^(^\.\d*$)?(^\d*\.\d$)?(^\d*\.)?(\d*)*$/;return(0,p.jsx)("div",{"data-open":a,className:"fixed flex items-center justify-center w-full h-dvh top-0 left-0 z-10 bg-gray-500/30 backdrop-blur-sm data-[open='-1']:animate-fade-out data-[open='1']:animate-fade-in",children:(0,p.jsx)("div",{className:"max-w-[400px] w-full p-4",children:(0,p.jsxs)("form",{onSubmit:async e=>{var a,r,s,o;if(e.preventDefault(),parseFloat(u)<=0)return null===(s=x.current)||void 0===s||s.setCustomValidity("Amount must be greater than 0."),void(null===(o=x.current)||void 0===o||o.reportValidity());try{const[e,t]=await(0,l.Kq)(n,c,parseFloat(u));switch(t){case i.s.CREATED:h((e=>e.map((e=>(e.id===n&&(e.amount+=parseFloat(u),e.transactions++),e)))));break;case i.s.UNAUTHORIZED:return window.location.replace("/login");case i.s.BAD_REQUEST:return null===(a=x.current)||void 0===a||a.setCustomValidity("An error occurred. Please try again."),void(null===(r=x.current)||void 0===r||r.reportValidity());default:return void console.error("Status Code: ".concat(t," :: An error occurred."))}}catch(m){console.error(m)}setTimeout((()=>{d(""),f("0")}),200),t()},className:"flex flex-col gap-4 w-full p-4 rounded-md bg-white text-black font-medium",children:[(0,p.jsx)("h1",{className:"text-center text-xl font-extrabold capitalize",children:s}),(0,p.jsx)("input",{value:c,onChange:e=>d(e.target.value),type:"text",placeholder:"Description",className:"flex-grow p-2 border border-gray-500 rounded-md"}),(0,p.jsx)("input",{value:u,onChange:e=>{var t;const a=e.target.value;if(""===a)return f("0");!1!==m.test(a)&&(f("0"===u?parseFloat(a).toString():a),null===(t=x.current)||void 0===t||t.setCustomValidity(""))},type:"text",ref:x,className:"p-2 border border-gray-500 rounded-md",required:!0}),(0,p.jsxs)("div",{className:"flex justify-between text-white font-bold",children:[(0,p.jsx)("button",{onClick:t,type:"button",className:"w-1/4 rounded-md p-2 bg-red-500 hover:bg-red-600 transition-colors shadow-md",children:"Close"}),(0,p.jsx)("button",{type:"submit",className:"w-1/4 rounded-md p-2 bg-green-500 hover:bg-green-600 transition-colors shadow-md",children:"Submit"})]})]})})})}const C=function(e){let{name:t,id:a}=e;const[s,n]=(0,r.useState)(0);return(0,p.jsxs)(p.Fragment,{children:[0!==s&&(0,p.jsx)(S,{closeModal:()=>{setTimeout((()=>n(0)),200),n(-1)},isModalOpen:s,name:t,id:a}),(0,p.jsx)("button",{onClick:()=>n(1),className:"p-2 bg-green-500 hover:bg-green-600 rounded-full shadow-md transition-colors",children:(0,p.jsx)(h.FaPlus,{})})]})},_=["#ff8000","#ffff00","#00ff00","#00ffff","#0000ff","#8000ff","#ff0080"];const A=function(e){let{id:t,name:a,icon:s}=e;const[n,c]=(0,r.useState)(0),[u,x]=(0,r.useState)(!1),[m,b]=(0,r.useState)(""),[w,v]=(0,r.useState)(s),[j,y]=(0,r.useState)(!1),N=o(),E=(0,d.Zp)(),R=()=>{setTimeout((()=>c(0)),200),c(-1)};return(0,p.jsxs)(p.Fragment,{children:[0!==n&&(0,p.jsx)("div",{"data-open":n,className:"fixed flex justify-center items-center w-full h-dvh top-0 left-0 z-10 bg-gray-500/30 backdrop-blur-sm data-[open='-1']:animate-fade-out data-[open='1']:animate-fade-in",children:(0,p.jsx)("div",{"data-offset":u,className:"max-w-[400px] w-full p-4 data-[offset='true']:pt-52 transition-all",children:(0,p.jsxs)("form",{onSubmit:async e=>{e.preventDefault(),y(!0),(0,l.bz)(t,m,w).then((e=>{switch(e){case i.s.OK:return void N((e=>e.map((e=>e.id===t?{...e,name:m,icon:w}:e))));case i.s.UNAUTHORIZED:return E("/login");default:throw new Error("Recieved an unexpected status code :: ".concat(e,"."))}})).catch(alert).then(R)},className:"flex flex-col gap-4 w-full p-4 rounded-md bg-white text-black font-medium",children:[(0,p.jsxs)("div",{className:"relative",children:[(0,p.jsx)("h1",{className:"text-center text-xl font-extrabold capitalize",children:a}),(0,p.jsx)("button",{onClick:()=>{y(!0),(0,l.Ee)(t).then((e=>{switch(e){case i.s.OK:return void N((e=>e.filter((e=>e.id!==t))));case i.s.UNAUTHORIZED:return E("/login");default:throw new Error("Recieved an unexpected status code :: ".concat(e,"."))}})).catch(alert).then(R)},disabled:j,type:"button",className:"absolute top-1/2 right-0 -translate-y-1/2 text-white p-2 bg-red-500 rounded-full hover:bg-red-600 transition-colors shadow-md disabled:bg-gray-400",children:(0,p.jsx)(f.qbC,{})})]}),(0,p.jsxs)("div",{className:"relative flex flex-grow gap-4",children:[(0,p.jsx)(g,{icon:w,setIcon:v,setOffset:x}),(0,p.jsx)("input",{value:m,onChange:e=>b(e.target.value),id:"name",placeholder:"Name",required:!0,className:"flex-grow p-2 border border-gray-500 rounded-md"})]}),(0,p.jsxs)("div",{className:"flex justify-between text-white font-bold",children:[(0,p.jsx)("button",{disabled:j,onClick:R,type:"button",className:"w-1/4 rounded-md p-2 bg-red-500 hover:bg-red-600 transition-colors shadow-md disabled:bg-gray-400",children:"Close"}),(0,p.jsx)("button",{disabled:j,type:"submit",className:"w-1/4 rounded-md p-2 bg-green-500 hover:bg-green-600 transition-colors shadow-md disabled:bg-gray-400",children:"Submit"})]})]})})}),(0,p.jsx)("button",{onClick:()=>c((e=>0===e?1:(setTimeout((()=>c(0)),200),-1*e))),className:"p-2 bg-blue-500 hover:bg-blue-600 rounded-full shadow-md transition-colors",children:(0,p.jsx)(h.FaPencilAlt,{})})]})};function O(e){let{id:t,icon:a,color:r,name:s,precent:n,amount:o,transactions:c,showLine:l}=e;const i=m(a);return(0,p.jsxs)("div",{className:"flex gap-4 px-4 last:mb-4",children:[(0,p.jsx)("div",{className:"relative h-20 aspect-square text-gray-300",children:(0,p.jsx)("a",{href:"./tracker/category?id=".concat(t),className:"w-full aspect-square outline-none hover:text-gray-500 transition-colors",children:(0,p.jsx)(R,{percent:n,borderColor:r,children:(0,p.jsx)(i,{className:"text-4xl text-shadow drop-shadow-md"})})})}),(0,p.jsxs)("div",{className:"relative flex justify-between items-center gap-4 w-full font-bold min-w-0",children:[(0,p.jsxs)("div",{className:"flex flex-col justify-between w-max overflow-auto",children:[(0,p.jsx)("span",{className:"capitalize text-black overflow-auto",children:s}),(0,p.jsxs)("span",{className:"text-gray-500 text-sm tracking-tight overflow-auto",children:[c," ",c>1?"transactions":"transaction"]})]}),(0,p.jsxs)("div",{className:"flex flex-col items-end justify-between w-max",children:[(0,p.jsxs)("span",{className:"text-red-500 w-max",children:["\u20b1",(0,u.U)(o)]}),(0,p.jsxs)("div",{className:"flex gap-2 mr-[1px] text-white",children:[(0,p.jsx)(C,{name:s,id:t}),(0,p.jsx)(A,{id:t,name:s,icon:a})]})]}),l&&(0,p.jsx)("hr",{className:"absolute -bottom-2 border-1 border-gray-500 w-full"})]})]})}function T(e){let{sumAmount:t}=e;const a=(0,r.useContext)(s),n=a.length;return(0,p.jsx)("div",{className:"flex flex-col gap-4 h-full overflow-y-auto no-scrollbar",children:a.sort(((e,t)=>e.amount<t.amount?1:e.amount>t.amount?-1:0)).map(((e,a)=>(0,p.jsx)(O,{...e,showLine:a!==n-1,color:_[a%_.length],precent:Math.min(100,e.amount/(t>0?t:1)*100)},e.id)))})}const k=function(){const[e,t]=(0,r.useState)([]),[a,o]=(0,r.useState)(0),f=(0,d.Zp)();(0,r.useEffect)((()=>{(0,l.dT)().then((e=>{let[a,r]=e;switch(r){case i.s.OK:return t(a),void o(a.reduce(((e,t)=>e+t.amount),0));case i.s.UNAUTHORIZED:return f("/login");default:throw new Error("Recieved an unexpected status code :: ".concat(r,"."))}})).catch(alert)}),[]),(0,r.useEffect)((()=>{o(e.reduce(((e,t)=>e+t.amount),0))}),[e]);const h=(new Date).toLocaleString("default",{month:"short",year:"numeric"});return(0,p.jsxs)("div",{className:"flex flex-col mx-auto h-full max-h-full max-w-[720px]",children:[(0,p.jsx)("div",{className:"mb-4 py-4",children:(0,p.jsx)("h1",{className:"font-bold text-center",children:"EXPENSES"})}),(0,p.jsx)(n.Provider,{value:t,children:(0,p.jsxs)(s.Provider,{value:e,children:[(0,p.jsxs)("div",{className:"flex flex-col gap-10 flex-grow overflow-auto",children:[(0,p.jsxs)("div",{className:"flex flex-col mx-auto w-min",children:[(0,p.jsx)("span",{className:"leading-none tracking-tight font-bold text-gray-400 w-max",children:h}),(0,p.jsxs)("span",{className:"mb-4 font-extrabold text-4xl w-min",children:["\u20b1",(0,u.U)(a)]})]}),(0,p.jsx)("div",{className:"flex-grow overflow-hidden",children:(0,p.jsx)(T,{sumAmount:a})})]}),(0,p.jsx)("div",{className:"px-4 pb-4",children:(0,p.jsxs)("div",{className:"flex justify-between border-t border-gray-400 w-full pt-4",children:[(0,p.jsx)("button",{disabled:!0,className:"disabled:text-gray-300",children:(0,p.jsx)(c.QXR,{className:"text-3xl"})}),(0,p.jsx)(v,{}),(0,p.jsx)("button",{onClick:async()=>{(0,l.fQ)().then((()=>{window.location.replace("/login")})).catch((()=>{alert("An error occurred. Please try again.")}))},type:"button",children:(0,p.jsx)(c.ueG,{className:"text-3xl"})})]})})]})})]})}},865:(e,t,a)=>{a.d(t,{AK:()=>C,Ee:()=>S,Hx:()=>g,Kq:()=>E,Nj:()=>b,_G:()=>N,az:()=>w,bz:()=>R,dT:()=>j,fQ:()=>v,w2:()=>y});var r=a(923),s=a(861);const n="".concat("https://expenses-tracker.shuttleapp.rs/api","/user"),o="".concat(n,"/auth"),c="".concat(o,"/tracker"),l="".concat(c,"/category"),i="".concat(c,"/categories"),d={deep:!0},u={headers:{"Content-Type":"application/json"},credentials:"include"},f={...u,method:"POST"},h={...u,method:"GET"},x={...u,method:"DELETE"},m={...u,method:"PATCH"},p="The server encountered an error. Please try again later.";async function b(){return fetch("".concat(o),h).then((e=>{if(e.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);return e.status}))}async function g(e,t){return fetch("".concat(n,"/sign-up"),{...f,body:JSON.stringify({username:e,password:t})}).then((e=>{if(e.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);return e.status}))}async function w(e,t){return fetch("".concat(n,"/login"),{...f,body:JSON.stringify({username:e,password:t})}).then((e=>{if(e.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);return e.status}))}async function v(){return fetch("".concat(n,"/logout"),x).then((e=>{if(e.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);return e.status}))}async function j(){return fetch("".concat(i),h).then((async e=>{if(e.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);return[(0,r.A)(await e.json(),d),e.status]}))}async function y(e){return fetch("".concat(l,"/").concat(e,"/transactions"),h).then((async e=>{if(e.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);const t=(0,r.A)(await e.json(),d);return t.transactions=t.transactions.map((e=>({...e,createdAt:new Date(e.createdAt)}))),[t,e.status]}))}async function N(e,t){return fetch("".concat(i),{...f,body:JSON.stringify({name:e,icon:t})}).then((async a=>{if(a.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);return[{...await a.json(),name:e,icon:t,transactions:0,amount:0,precent:0,color:"#000000"},a.status]}))}async function E(e,t,a){return fetch("".concat(l,"/").concat(e,"/transactions"),{...f,body:JSON.stringify({amount:a,description:t})}).then((async e=>{if(e.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);return[(await e.json()).id,e.status]}))}async function R(e,t,a){return fetch("".concat(l,"/").concat(e),{...m,body:JSON.stringify({name:t,icon:a})}).then((e=>{if(e.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);return e.status}))}async function S(e){return fetch("".concat(i,"/").concat(e),x).then((e=>{if(e.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);return e.status}))}async function C(e,t){return fetch("".concat(l,"/").concat(e,"/transactions/").concat(t),x).then((e=>{if(e.status===s.s.INTERNAL_SERVER_ERROR)throw new Error(p);return e.status}))}},861:(e,t,a)=>{a.d(t,{s:()=>r});const r={OK:200,CREATED:201,CONFLICT:409,BAD_REQUEST:400,UNAUTHORIZED:401,INTERNAL_SERVER_ERROR:500}},596:(e,t,a)=>{function r(e){return e.toLocaleString("en-US",{minimumFractionDigits:2})}a.d(t,{U:()=>r})}}]);
//# sourceMappingURL=939.afc0d4c5.chunk.js.map