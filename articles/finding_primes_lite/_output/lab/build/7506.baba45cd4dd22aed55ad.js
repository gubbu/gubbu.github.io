"use strict";(self.webpackChunk_JUPYTERLAB_CORE_OUTPUT=self.webpackChunk_JUPYTERLAB_CORE_OUTPUT||[]).push([[7506],{17506:(e,t,s)=>{s.r(t),s.d(t,{IDocumentProviderFactory:()=>h,ProviderMock:()=>c,WebSocketProviderWithLocks:()=>P,getAnonymousUserName:()=>o,getRandomColor:()=>a,moonsOfJupyter:()=>n,userColors:()=>i});const n=["Metis","Adrastea","Amalthea","Thebe","Io","Europa","Ganymede","Callisto","Themisto","Leda","Ersa","Pandia","Himalia","Lysithea","Elara","Dia","Carpo","Valetudo","Euporie","Eupheme","Helike","Euanthe","Hermippe","Praxidike","Thyone","Thelxinoe","Ananke","Mneme","Orthosie","Harpalyke","Iocaste","Erinome","Aitne","Herse","Taygete","Eukelade","Carme","Isonoe","Autonoe","Philophrosyne","Cyllene","Pasithee","Pasiphae","Sponde","Eurydome","Kalyke","Hegemone","Kale","Kallichore","Chaldene","Arche","Eirene","Kore","Megaclite","Aoede","Callirrhoe","Sinope"],o=()=>"Anonymous "+n[Math.floor(Math.random()*n.length)],i=["#12A0D3","#17AB30","#CC8500","#A79011","#ee6352","#609DA9","#4BA749","#00A1B3"],a=()=>i[Math.floor(Math.random()*i.length)];class c{requestInitialContent(){return Promise.resolve(!1)}putInitializedState(){}acquireLock(){return Promise.resolve(0)}releaseLock(e){}destroy(){}setPath(e){}}var r=s(93315);const h=new r.Token("@jupyterlab/docprovider:IDocumentProviderFactory");var l=s(94072),d=s(84795),u=s(17543),_=s(62538),w=s(40870),m=s(68078),f=s(6493),y=s(31955),p=s(42959),b=s(21332),g=s(69600),v=s(59406);const C=[];C[0]=(e,t,s,n,o)=>{d.uE(e,0);const i=m.zu(t,e,s.doc,s);n&&i===m.Px&&!s.synced&&(s.synced=!0)},C[3]=(e,t,s,n,o)=>{d.uE(e,1),d.mP(e,f.xq(s.awareness,Array.from(s.awareness.getStates().keys())))},C[1]=(e,t,s,n,o)=>{f.oy(s.awareness,l.HN(t),s)},C[2]=(e,t,s,n,o)=>{((e,t,s)=>{switch(l.yg(e)){case 0:s(t,l.kf(e))}})(t,s.doc,k)};const k=(e,t)=>console.warn(`Permission denied to access ${e.url}.\n${t}`),I=(e,t,s)=>{const n=l.l1(t),o=d.Mf(),i=l.yg(n),a=e.messageHandlers[i];return a?a(o,n,e,s,i):console.error("Unable to compute message"),o},E=e=>{if(e.shouldConnect&&null===e.ws){const t=new e._WS(e.url);t.binaryType="arraybuffer",e.ws=t,e.wsconnecting=!0,e.wsconnected=!1,e.synced=!1,t.onmessage=s=>{e.wsLastMessageReceived=w.ZG();const n=I(e,new Uint8Array(s.data),!0);d.kE(n)>1&&t.send(d._f(n))},t.onclose=()=>{e.ws=null,e.wsconnecting=!1,e.wsconnected?(e.wsconnected=!1,e.synced=!1,f.Ag(e.awareness,Array.from(e.awareness.getStates().keys()).filter((t=>t!==e.doc.clientID)),e),e.emit("status",[{status:"disconnected"}])):e.wsUnsuccessfulReconnects++,setTimeout(E,b.VV(1200*b.mv(e.wsUnsuccessfulReconnects+1),2500),e)},t.onopen=()=>{e.wsLastMessageReceived=w.ZG(),e.wsconnecting=!1,e.wsconnected=!0,e.wsUnsuccessfulReconnects=0,e.emit("status",[{status:"connected"}]);const s=d.Mf();if(d.uE(s,0),m._J(s,e.doc),t.send(d._f(s)),null!==e.awareness.getLocalState()){const s=d.Mf();d.uE(s,1),d.mP(s,f.xq(e.awareness,[e.doc.clientID])),t.send(d._f(s))}},e.emit("status",[{status:"connecting"}])}},M=(e,t)=>{e.wsconnected&&e.ws.send(t),e.bcconnected&&e.mux((()=>{_.nY(e.bcChannel,t)}))};class U extends p.y{constructor(e,t,s,{connect:n=!0,awareness:o=new f.GL(s),params:i={},WebSocketPolyfill:a=WebSocket,resyncInterval:c=-1}={}){for(super();"/"===e[e.length-1];)e=e.slice(0,e.length-1);const r=(e=>g.UI(e,((e,t)=>`${encodeURIComponent(t)}=${encodeURIComponent(e)}`)).join("&"))(i);this.bcChannel=e+"/"+t,this.url=e+"/"+t+(0===r.length?"":"?"+r),this.roomname=t,this.doc=s,this._WS=a,this.awareness=o,this.wsconnected=!1,this.wsconnecting=!1,this.bcconnected=!1,this.wsUnsuccessfulReconnects=0,this.messageHandlers=C.slice(),this.mux=y.M(),this._synced=!1,this.ws=null,this.wsLastMessageReceived=0,this.shouldConnect=n,this._resyncInterval=0,c>0&&(this._resyncInterval=setInterval((()=>{if(this.ws){const e=d.Mf();d.uE(e,0),m._J(e,s),this.ws.send(d._f(e))}}),c)),this._bcSubscriber=e=>{this.mux((()=>{const t=I(this,new Uint8Array(e),!1);d.kE(t)>1&&_.nY(this.bcChannel,d._f(t))}))},this._updateHandler=(e,t)=>{if(t!==this){const t=d.Mf();d.uE(t,0),m.lr(t,e),M(this,d._f(t))}},this.doc.on("update",this._updateHandler),this._awarenessUpdateHandler=({added:e,updated:t,removed:s},n)=>{const i=e.concat(t).concat(s),a=d.Mf();d.uE(a,1),d.mP(a,f.xq(o,i)),M(this,d._f(a))},this._beforeUnloadHandler=()=>{f.Ag(this.awareness,[s.clientID],"window unload")},"undefined"!=typeof window?window.addEventListener("beforeunload",this._beforeUnloadHandler):v.on("exit",(()=>this._beforeUnloadHandler)),o.on("update",this._awarenessUpdateHandler),this._checkInterval=setInterval((()=>{this.wsconnected&&3e4<w.ZG()-this.wsLastMessageReceived&&this.ws.close()}),3e3),n&&this.connect()}get synced(){return this._synced}set synced(e){this._synced!==e&&(this._synced=e,this.emit("synced",[e]),this.emit("sync",[e]))}destroy(){0!==this._resyncInterval&&clearInterval(this._resyncInterval),clearInterval(this._checkInterval),this.disconnect(),"undefined"!=typeof window?window.removeEventListener("beforeunload",this._beforeUnloadHandler):v.off("exit",(()=>this._beforeUnloadHandler)),this.awareness.off("update",this._awarenessUpdateHandler),this.doc.off("update",this._updateHandler),super.destroy()}connectBc(){this.bcconnected||(_.Ld(this.bcChannel,this._bcSubscriber),this.bcconnected=!0),this.mux((()=>{const e=d.Mf();d.uE(e,0),m._J(e,this.doc),_.nY(this.bcChannel,d._f(e));const t=d.Mf();d.uE(t,0),m.K0(t,this.doc),_.nY(this.bcChannel,d._f(t));const s=d.Mf();d.uE(s,3),_.nY(this.bcChannel,d._f(s));const n=d.Mf();d.uE(n,1),d.mP(n,f.xq(this.awareness,[this.doc.clientID])),_.nY(this.bcChannel,d._f(n))}))}disconnectBc(){const e=d.Mf();d.uE(e,1),d.mP(e,f.xq(this.awareness,[this.doc.clientID],new Map)),M(this,d._f(e)),this.bcconnected&&(_.r1(this.bcChannel,this._bcSubscriber),this.bcconnected=!1)}disconnect(){this.shouldConnect=!1,this.disconnectBc(),null!==this.ws&&this.ws.close()}connect(){this.shouldConnect=!0,this.wsconnected||null!==this.ws||(E(this),this.connectBc())}}var L=s(40999);class P extends U{constructor(e){var t;super(e.url,e.contentType+":"+e.path,e.ymodel.ydoc,{awareness:e.ymodel.awareness}),this._currentLockRequest=null,this._initialContentRequest=null,this._path=e.path,this._contentType=e.contentType,this._serverUrl=e.url;const s="#"+L.jS("--usercolor",a().slice(1)),n=decodeURIComponent(L.jS("--username",o())),i=e.ymodel.awareness.getLocalState();i&&null==(null===(t=i.user)||void 0===t?void 0:t.name)&&e.ymodel.awareness.setLocalStateField("user",{name:n,color:s}),this.messageHandlers[127]=(e,t,s,n,o)=>{const i=l.Jl(t),a=this._currentLockRequest;this._currentLockRequest=null,a&&a.resolve(i)},this.messageHandlers[125]=(e,t,s,n,o)=>{const i=l.iU(t);i.byteLength>0&&setTimeout((()=>{u.applyUpdate(this.doc,i)}),0);const a=this._initialContentRequest;this._initialContentRequest=null,a&&a.resolve(i.byteLength>0)},this._isInitialized=!1,this._onConnectionStatus=this._onConnectionStatus.bind(this),this.on("status",this._onConnectionStatus)}setPath(e){if(e!==this._path){this._path=e;const t=d.Mf();d.cW(t,123);const s=unescape(encodeURIComponent(this._contentType+":"+e));for(let e=0;e<s.length;e++)d.cW(t,s.codePointAt(e));this._sendMessage(d._f(t)),this.disconnectBc(),this.bcChannel=this._serverUrl+"/"+this._contentType+":"+this._path,this.url=this.bcChannel,this.connectBc()}}requestInitialContent(){return this._initialContentRequest||(this._initialContentRequest=new r.PromiseDelegate,this._sendMessage(new Uint8Array([125])),setTimeout((()=>{var e;return null===(e=this._initialContentRequest)||void 0===e?void 0:e.resolve(!1)}),1e3)),this._initialContentRequest.promise}putInitializedState(){const e=d.Mf();d.uE(e,124),d.HK(e,u.encodeStateAsUpdate(this.doc)),this._sendMessage(d._f(e)),this._isInitialized=!0}acquireLock(){if(this._currentLockRequest)return this._currentLockRequest.promise;let e,t;this._sendMessage(new Uint8Array([127])),this._requestLockInterval&&clearInterval(this._requestLockInterval),this._requestLockInterval=setInterval((()=>{this.wsconnected&&this._sendMessage(new Uint8Array([127]))}),500);const s=new Promise(((s,n)=>{e=s,t=n}));return this._currentLockRequest={promise:s,resolve:e,reject:t},s}releaseLock(e){const t=d.Mf();d.uE(t,126),d.Ep(t,e),this._sendMessage(d._f(t)),this._requestLockInterval&&clearInterval(this._requestLockInterval)}_sendMessage(e){const t=()=>{setTimeout((()=>{this.wsconnected?this.ws.send(e):this.once("status",t)}),0)};t()}async _onConnectionStatus(e){if(this._isInitialized&&"connected"===e.status){const e=await this.acquireLock();await this.requestInitialContent()||this.putInitializedState(),this.releaseLock(e)}}}},62538:(e,t,s)=>{s.d(t,{Ld:()=>h,r1:()=>l,nY:()=>d});var n=s(23205),o=s(24803),i=s(55852);const a=new Map,c="undefined"==typeof BroadcastChannel?class{constructor(e){this.room=e,this.onmessage=null,i.z((t=>t.key===e&&null!==this.onmessage&&this.onmessage({data:o.Gh(t.newValue||"")})))}postMessage(e){i.X.setItem(this.room,o.s3(o.eh(e)))}}:BroadcastChannel,r=e=>n.Yu(a,e,(()=>{const t=new Set,s=new c(e);return s.onmessage=e=>t.forEach((t=>t(e.data))),{bc:s,subs:t}})),h=(e,t)=>r(e).subs.add(t),l=(e,t)=>r(e).subs.delete(t),d=(e,t)=>{const s=r(e);s.bc.postMessage(t),s.subs.forEach((e=>e(t)))}},31955:(e,t,s)=>{s.d(t,{M:()=>n});const n=()=>{let e=!0;return(t,s)=>{if(e){e=!1;try{t()}finally{e=!0}}else void 0!==s&&s()}}},68078:(e,t,s)=>{s.d(t,{Wh:()=>a,Px:()=>c,_J:()=>r,K0:()=>h,lr:()=>d,zu:()=>_});var n=s(84795),o=s(94072),i=s(17543);const a=0,c=1,r=(e,t)=>{n.uE(e,a);const s=i.encodeStateVector(t);n.mP(e,s)},h=(e,t,s)=>{n.uE(e,c),n.mP(e,i.encodeStateAsUpdate(t,s))},l=(e,t,s)=>{try{i.applyUpdate(t,o.HN(e),s)}catch(e){console.error("Caught error while handling a Yjs update",e)}},d=(e,t)=>{n.uE(e,2),n.mP(e,t)},u=l,_=(e,t,s,n)=>{const i=o.yg(e);switch(i){case a:((e,t,s)=>{h(t,s,o.HN(e))})(e,t,s);break;case c:l(e,s,n);break;case 2:u(e,s,n);break;default:throw new Error("Unknown message type")}return i}}}]);
//# sourceMappingURL=7506.baba45cd4dd22aed55ad.js.map